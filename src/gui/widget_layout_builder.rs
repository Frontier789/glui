use super::widget::*;
use tools::*;

pub struct WidgetLayoutBuilder {
    pub widgets: Vec<Box<dyn Widget>>,
    pub postorder: Vec<usize>,
    pub widget_graph: Vec<Vec<usize>>,
    pub constraints: Vec<WidgetConstraints>,
    pub positions: Vec<WidgetPosition>,
    pub max_descent: Vec<f32>,
    root_descent: f32,
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
            root_descent: 0.015,
            win_size: Vec2px::zero(),
            max_descent: vec![],
        }
    }

    fn set_widget_count(&mut self, n: usize) {
        self.constraints.resize(n, Default::default());
        self.positions.resize(n, Default::default());
        self.max_descent.resize(n, Default::default());
        self.next_child_constraints.resize(n, Default::default());
    }
    fn pop(&mut self, id: usize, parent: Option<usize>) {
        self.positions[id] = match parent {
            Some(parid) => {
                let s = self.widgets[id].size();
                let p = self.widgets[parid].place_child(s, self.max_descent[id]);
                self.max_descent[parid] =
                    f32::max(self.max_descent[id] + p.depth, self.max_descent[parid]);
                p
            }
            None => {
                let d = self.root_descent;

                self.root_descent +=
                    self.max_descent[id] + WidgetPosition::from(Vec2px::origin()).depth;
                WidgetPosition::new(Vec2px::origin(), d)
            }
        };
        // println!("Widget id {}, size: {:?} has been put to {:?}", id, self.widgets[id].size(), self.positions[id]);
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
    pub fn make_pos_abs(&mut self) {
        let mut visited = vec![false; self.widgets.len()];
        for i in 0..visited.len() {
            if !visited[i] {
                visited[i] = true;
                self.make_pos_abs_rec(i, &mut visited);
            }
        }
    }
    fn make_pos_abs_rec(&mut self, index: usize, visited: &mut Vec<bool>) {
        let offset = self.positions[index];
        for &i in &self.widget_graph[index].clone() {
            self.positions[i].pos += offset.pos;
            self.positions[i].depth += offset.depth;
            visited[i] = true;
            self.make_pos_abs_rec(i, visited);
        }
    }
    pub fn build(&mut self, win_size: Vec2px) {
        let n = self.widgets.len();
        self.win_size = win_size;
        self.set_widget_count(n);

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
        self.make_pos_abs();
        // println!("depth are {:?}", self.positions.iter().map(|p| p.depth).collect::<Vec<f32>>());
    }
}
