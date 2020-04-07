use super::*;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug)]
pub struct WidgetBuildCache {
    cache_id: u64,
}

#[derive(Default)]
pub struct WidgetList {
    pub widgets: Vec<Box<dyn Widget>>,
    pub parents: Vec<Option<usize>>,
    pub postorder: Vec<usize>,
    pub widget_graph: Vec<Vec<usize>>,
    pub widget_depth: Vec<usize>,
    pub cache_loc: Vec<u64>,
    pub cache_details: Vec<WidgetBuildCache>,
    id_stack: Vec<usize>,
}

impl WidgetList {
    fn parse_push_widget(&mut self, w: Box<dyn Widget>) {
        let id = self.widgets.len();
        match self.id_stack.last() {
            None => {
                self.parents.push(None);
                self.widget_graph.push(vec![]);
            }
            Some(&parid) => {
                self.parents.push(Some(parid));
                self.widget_graph.push(vec![]);
                self.widget_graph[parid].push(id);
            }
        }

        self.cache_loc
            .push(self.cache_details.last().unwrap().cache_id);
        self.id_stack.push(id);
        self.widget_depth.push(self.id_stack.len() - 1);
        let children = w.expand();
        self.widgets.push(w);
        for c in children {
            self.parse_push_widget(c);
            self.parse_pop();
        }
    }
    fn parse_pop(&mut self) {
        let id = self.id_stack.pop().unwrap();
        self.postorder.push(id);
    }
    fn push_cache(&mut self, cache: WidgetBuildCache) {
        self.cache_details.push(cache);
    }
    fn pop_cache(&mut self) {
        self.cache_details.pop();
    }
}

thread_local! {
    static WIDGETPARSER_INSTANCE: RefCell<WidgetParser> = RefCell::new(WidgetParser::default());
}

#[derive(Default)]
pub struct WidgetParser {
    output: Option<WidgetList>,
    callbacks: HashMap<usize, Box<dyn Fn(&mut dyn Any)>>,
    next_callback_id: usize,
}

impl WidgetParser {
    pub fn produce_list<F>(generator: F) -> WidgetList
    where
        F: Fn(),
    {
        WIDGETPARSER_INSTANCE.with(|widget_parser| {
            widget_parser.borrow_mut().output = Some(WidgetList::default());
        });
        
        generator();
        
        let mut result = None;
        
        WIDGETPARSER_INSTANCE.with(|widget_parser| {
            result = widget_parser.borrow_mut().output.take();
        });
        
        result.unwrap()
    }
    pub fn parse_push<T>(w: T)
    where
        T: Widget + 'static,
    {
        WIDGETPARSER_INSTANCE.with(|widget_parser| {
            let mut widget_parser = widget_parser.borrow_mut();
            if let Some(widgetlist) = &mut widget_parser.output {
                widgetlist.parse_push_widget(Box::new(w));
            }
        });
    }
    pub fn parse_pop() {
        WIDGETPARSER_INSTANCE.with(|widget_parser| {
            let mut widget_parser = widget_parser.borrow_mut();
            if let Some(widgetlist) = &mut widget_parser.output {
                widgetlist.parse_pop();
            }
        });
    }
    pub fn push_cache(cache_id: u64) {
        WIDGETPARSER_INSTANCE.with(|widget_parser| {
            let mut widget_parser = widget_parser.borrow_mut();
            if let Some(widgetlist) = &mut widget_parser.output {
                widgetlist.push_cache(WidgetBuildCache {cache_id});
            }
        });
    }
    pub fn pop_cache() {
        WIDGETPARSER_INSTANCE.with(|widget_parser| {
            let mut widget_parser = widget_parser.borrow_mut();
            if let Some(widgetlist) = &mut widget_parser.output {
                widgetlist.pop_cache();
            }
        });
    }
    pub fn register_param<T>(_param: &T)
    where
        T: std::fmt::Debug,
    {
        // println!("Registered param {:?}", param);
    }
    pub fn make_callback<F, D>(f: F) -> GuiCallback
    where
        F: 'static + Fn(&mut D),
        D: 'static,
    {
        let mut id: usize = 0;
        
        WIDGETPARSER_INSTANCE.with(|widget_parser| {
            let mut widget_parser = widget_parser.borrow_mut();
            
            id = widget_parser.next_callback_id;
            widget_parser.next_callback_id += 1;
            
            widget_parser.callbacks.insert(id, Box::new(move |input: &mut dyn Any|{
                f(input.downcast_mut().unwrap());
            }));
        });
        
        GuiCallback {
            callback_id: Some(id),
        }
    }
    pub fn remove_callback(cb: &GuiCallback)
    {
        if let Some(id) = cb.callback_id {
            WIDGETPARSER_INSTANCE.with(|widget_parser| {
                let mut widget_parser = widget_parser.borrow_mut();
                widget_parser.callbacks.remove(&id);
            });
        }
    }
    pub fn execute_callback(cb: &GuiCallback, data: &mut dyn Any)
    {
        if let Some(id) = cb.callback_id {
            WIDGETPARSER_INSTANCE.with(|widget_parser| {
                let widget_parser = widget_parser.borrow();
                if let Some(fun) = widget_parser.callbacks.get(&id) {
                    fun(data);
                }
            });
        }
    }
}

#[derive(Default)]
pub struct GuiCallback {
    callback_id: Option<usize>,
}

impl Drop for GuiCallback {
    fn drop(&mut self) {
        WidgetParser::remove_callback(self);
    }
}

pub struct CallbackExecutor<'a> {
    pub data: &'a mut dyn Any,
}

impl<'a> CallbackExecutor<'a> {
    pub fn execute(&mut self, cb: &GuiCallback) {
        WidgetParser::execute_callback(cb, self.data);
    }
}

pub struct WidgetLayoutBuilder {
    pub widgets: Vec<Box<dyn Widget>>,
    pub postorder: Vec<usize>,
    pub widget_graph: Vec<Vec<usize>>,
    pub constraints: Vec<WidgetConstraints>,
    pub positions: Vec<Vec2px>,
    next_child_constraints: Vec<WidgetConstraints>,
    win_size: Vec2px,
}

impl WidgetLayoutBuilder {
    pub fn new(
        widgets: Vec<Box<dyn Widget>>,
        postorder: Vec<usize>,
        widget_graph: Vec<Vec<usize>>,
    ) -> WidgetLayoutBuilder {
        WidgetLayoutBuilder {
            widgets,
            postorder,
            widget_graph,
            constraints: vec![],
            positions: vec![],
            next_child_constraints: vec![],
            win_size: Vec2px::zero(),
        }
    }

    fn adopt_size(&mut self, n: usize) {
        self.constraints.resize(n, Default::default());
        self.positions.resize(n, Default::default());
        self.next_child_constraints.resize(n, Default::default());
    }
    fn pop(&mut self, id: usize, parent: Option<usize>) {
        self.positions[id] = match parent {
            Some(parid) => {
                let s = self.widgets[id].size();
                self.widgets[parid].place_child(s)
            }
            None => Vec2px::zero(),
        }
    }
    fn push(&mut self, id: usize, parent: Option<usize>) {
        match parent {
            Some(parid) => {
                let c = self.widgets[parid].child_constraint();
                match c {
                    Some(cons) => {
                        self.next_child_constraints[parid] = cons;
                    }
                    None => {}
                }
                self.constraints[id] = self.next_child_constraints[parid];
            }
            None => {
                self.constraints[id] = WidgetConstraints {
                    max_size: self.win_size,
                };
            }
        }

        self.widgets[id].constraint(self.constraints[id]);
        self.next_child_constraints[id] = match self.widgets[id].child_constraint() {
            Some(cons) => cons,
            None => self.constraints[id],
        }
    }
    pub fn make_pos_abs(&mut self, index: usize, offset: Vec2px) {
        for &i in &self.widget_graph[index].clone() {
            self.positions[i] += offset;
            self.make_pos_abs(i, self.positions[i]);
        }
    }
    pub fn build(&mut self, win_size: Vec2px) {
        let n = self.widgets.len();
        self.win_size = win_size;
        self.adopt_size(n);

        let mut id_stack: Vec<usize> = vec![];
        let mut cid = 0 as usize;
        let mut pop_i = 0;
        while pop_i < n {
            match id_stack.last() {
                Some(&id) => {
                    if id == self.postorder[pop_i] {
                        id_stack.pop();
                        match id_stack.last() {
                            Some(&parid) => self.pop(id, Some(parid)),
                            None => self.pop(id, None),
                        }
                        pop_i += 1;
                    } else {
                        self.push(cid, Some(id));
                        id_stack.push(cid);
                        cid += 1;
                    }
                }
                None => {
                    self.push(cid, None);
                    id_stack.push(cid);
                    cid += 1;
                }
            }
        }
    }
}
