use super::*;

pub trait WidgetTreeParser {
    fn parse_push<T>(&mut self, w: T) -> bool
    where
        T: Widget + 'static;
    
    fn parse_pop<T>(&mut self)
    where
        T: Widget + 'static;
}

pub struct WidgetTreeToList {
    pub widgets: Vec<Box<dyn Widget>>,
    pub postorder: Vec<usize>,
    pub child_count: Vec<usize>,
    pub widget_depth: Vec<usize>,
    
    id_stack: Vec<usize>,
}

impl WidgetTreeToList {
    pub fn new() -> Self {
        Self {
            widgets: vec![],
            postorder: vec![],
            child_count: vec![],
            widget_depth: vec![],
            id_stack: vec![],
        }
    }
    
    pub fn widget_count(&self) -> usize {
        self.widgets.len()
    }
}

impl WidgetTreeParser for WidgetTreeToList {
    fn parse_push<T>(&mut self, w: T) -> bool
    where
        T: Widget + 'static,
    {
        match self.id_stack.last() {
            None => (),
            Some(&parid) => {
                self.child_count[parid] += 1;
            }
        }
        
        let id = self.widgets.len();
        self.id_stack.push(id);
        self.widgets.push(Box::new(w));
        self.child_count.push(0);
        self.widget_depth.push(self.id_stack.len()-1);
        
        true
    }
    
    fn parse_pop<T>(&mut self)
    where
        T: Widget + 'static,
    {
        let id = self.id_stack.pop().unwrap();
        self.postorder.push(id);
    }
}

pub struct WidgetLayoutBuilder {
    pub widgets: Vec<Box<dyn Widget>>,
    pub postorder: Vec<usize>,
    
    pub child_count: Vec<usize>,
    pub constraints: Vec<WidgetConstraints>,
    pub positions: Vec<Vec2px>,
    
    next_child_constraints: Vec<WidgetConstraints>,
    win_size: Vec2px,
}

impl WidgetLayoutBuilder {
    pub fn new(
        widgets: Vec<Box<dyn Widget>>,
        postorder: Vec<usize>,
        child_count: Vec<usize>,
    ) -> WidgetLayoutBuilder {
        WidgetLayoutBuilder {
            widgets,
            postorder,
            child_count,
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
    
    pub fn make_pos_abs(&mut self, index: usize, offset: Vec2px) -> usize {
        let n = self.child_count[index];
        let mut index = index + 1;
        for _ in 0..n {
            self.positions[index] += offset;
            index = self.make_pos_abs(index, self.positions[index]);
        }
        index
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
