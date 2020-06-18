use gui::Widget;

#[derive(Debug)]
pub struct WidgetBuilderCache {
    pub cache_id: u64,
}

#[derive(Default)]
pub struct WidgetList {
    pub widgets: Vec<Box<dyn Widget>>,
    pub parents: Vec<Option<usize>>,
    pub postorder: Vec<usize>,
    pub widget_graph: Vec<Vec<usize>>,
    pub widget_depth: Vec<usize>,
    id_stack: Vec<usize>,
}

impl WidgetList {
    pub fn new() -> WidgetList {
        Default::default()
    }
    fn update_graph(&mut self, id: usize) {
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
    }
    pub fn parse_push_widget(&mut self, w: Box<dyn Widget>) {
        let id = self.widgets.len();
        self.update_graph(id);

        self.id_stack.push(id);
        self.widget_depth.push(self.id_stack.len() - 1);

        self.widgets.push(w);
    }
    pub fn parse_pop(&mut self) {
        let id = self.id_stack.pop().unwrap();
        self.postorder.push(id);
    }
    pub fn enter_builder(&mut self, _cache: WidgetBuilderCache) {}
    pub fn leave_builder(&mut self) {}
}
