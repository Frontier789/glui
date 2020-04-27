use mecs::world::UiEvent;
use mecs::*;
use std::path::Path;
use super::draw::*;
use super::widget::*;
use super::transforms::*;
use tools::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum GrabState {
    NoGrab,
    GrabbedInside,
    GrabbedOutside,
}

pub struct GuiContext<D> {
    render_target: RenderTarget,
    draw_res: DrawResources,
    widgets: Vec<Box<dyn Widget + 'static>>,
    parents: Vec<Option<usize>>,
    widget_graph: Vec<Vec<usize>>,
    widget_depth: Vec<usize>,
    positions: Vec<WidgetPosition>,
    active_widget: Option<usize>,
    cursor_hierarchy: Option<usize>,
    cursor_grabbed: GrabState,
    cursor_pos: Vec2,
    render_seq: Option<RenderSequence>,
    render_dirty: bool,
    profiler: Profiler,
    build_data: D,
    cb_executor: CallbackExecutor,
    widget_builder: Box<dyn Fn(D)>,
}

impl<D> Actor for GuiContext<D>
where
    D: PartialEq + Clone + 'static,
{
    fn receive(&mut self, msg: Box<dyn Message>, world: &mut StaticWorld) {
        // println!("context got: {:?}",msg);
        match_downcast!( msg {
            ui_event : UiEvent => {
                match ui_event {
                    UiEvent::GlutinEvent(ev) => {
                        self.handle_event(ev);
                    },
                    UiEvent::Redraw => {
                        self.render();
                    },
                    UiEvent::EventsCleared => {
                        self.actualize_data();
                        if !self.cb_executor.sender.messages.is_empty() {
                            for message in self.cb_executor.sender.messages.drain(0..) {
                                world.send_annotated(message);
                            }
                        }
                    },
                }
            },
            _ => {}
        });
    }
}

impl<D> GuiContext<D>
where
    D: Clone + 'static + PartialEq,
{
    pub fn new<F>(
        target: RenderTarget,
        profile: bool,
        widget_builder: F,
        build_data: D,
    ) -> GuiContext<D>
    where
        F: Fn(D) + 'static,
    {
        GuiContext {
            render_target: target,
            widgets: vec![],
            parents: vec![],
            widget_graph: vec![],
            widget_depth: vec![],
            positions: vec![],
            cursor_hierarchy: None,
            active_widget: None,
            cursor_grabbed: GrabState::NoGrab,
            cursor_pos: Vec2::new(-1.0, -1.0),
            render_seq: None,
            render_dirty: true,
            draw_res: DrawResources::new(),
            profiler: Profiler::new(profile),
            build_data: build_data.clone(),
            cb_executor: CallbackExecutor { data: Box::new(build_data), sender: PostBox::new() },
            widget_builder: Box::new(widget_builder),
        }
    }
    pub fn init_gl_res(&mut self) {
        self.draw_res.create_defaults().unwrap();
    }
    fn rebuild_render_seq(&mut self) {
        self.profiler.begin("Rebuild_Render");
        let mut builder = DrawBuilder::new(&mut self.draw_res);
        let n = self.widgets.len();
        for i in 0..n {
            builder.offset = self.positions[i].to_pixels(1.0);
            self.widgets[i].on_draw_build(&mut builder);
            // builder.add_clr_rect(Rect::from_pos_size(Vec2::origin(), self.widgets[i].size().to_pixels(1.0)), Vec4::new(1.0,0.0,0.0,0.5));
        }
        self.render_seq = Some(builder.to_render_sequence(&self.render_target));
        self.render_dirty = false;
        self.profiler.end();
    }

    pub fn rebuild_gui(&mut self) {
        crate::tools::gltraits::check_glerr_debug();
        self.profiler.begin("Rebuild_Gui");
        let widget_builder = &self.widget_builder;
        let widget_list = WidgetParser::produce_list(|| {
            widget_builder(self.build_data.clone());
        });
        // println!("cache_details is {:?}",widget_list.cache_details);
        // println!("cache_loc is {:?}",widget_list.cache_loc);

        let mut layout_builder = WidgetLayoutBuilder::new(
            widget_list.widgets,
            widget_list.postorder,
            widget_list.widget_graph,
        );
        layout_builder.build(self.render_target.logical_size());
        self.cursor_hierarchy = None;
        self.widget_graph = layout_builder.widget_graph;
        self.parents = widget_list.parents;
        self.widget_depth = widget_list.widget_depth;
        self.widgets = layout_builder.widgets;
        self.positions = layout_builder.positions;
        self.profiler.end();
        self.rebuild_render_seq();
    }
    pub fn resized(&mut self, s: Vec2) {
        self.render_target.size = s;
        self.rebuild_gui();
    }
    pub fn widget_count(&self) -> usize {
        self.widgets.len()
    }
    pub fn button_released(&mut self, button: GlutinButton) {
        if button != GlutinButton::Left {
            return;
        }
        match (self.active_widget, self.cursor_grabbed) {
            (_, GrabState::GrabbedOutside) => {
                self.rebuild_cursor_inside(self.cursor_pos);
            }
            (Some(id), GrabState::GrabbedInside) => {
                if self.widgets[id].on_release(&mut self.cb_executor) == EventResponse::HandledRedraw {
                    self.render_dirty = true;
                }
            }
            _ => {}
        }
        self.cursor_grabbed = GrabState::NoGrab;
    }
    pub fn button_pressed(&mut self, button: GlutinButton) {
        if button != GlutinButton::Left {
            return;
        }
        match self.cursor_hierarchy {
            Some(mut id) => {
                let mut result = self.widgets[id].on_press(&mut self.cb_executor);
                while result == EventResponse::Pass {
                    if let Some(parent) = self.parents[id] {
                        result = self.widgets[parent].on_press(&mut self.cb_executor);
                        id = parent;
                    } else {
                        break;
                    }
                }
                if result != EventResponse::Pass {
                    self.active_widget = Some(id);
                    self.cursor_grabbed = GrabState::GrabbedInside;
                }
                if result == EventResponse::HandledRedraw {
                    self.render_dirty = true;
                }
            }
            _ => {}
        }
    }

    pub fn key_pressed(&mut self, _key: GlutinKey) {}

    pub fn key_released(&mut self, _key: GlutinKey) {}
    pub fn cursor_left(&mut self) {
        self.cursor_moved(Vec2::new(-1.0, -1.0));
    }
    fn point_in_widget(&self, id: usize, p: Vec2) -> bool {
        let scl = self.render_target.gui_scale;
        let pos = self.positions[id].pos.to_pixels(scl);
        let siz = self.widgets[id].size().to_pixels(scl);
        Rect::from_pos_size(pos, siz).contains(p)
    }
    fn fire_enter_leave_event(&mut self, id: usize, enter: bool) {
        if enter {
            if self.widgets[id].on_cursor_enter(&mut self.cb_executor) == EventResponse::HandledRedraw {
                self.render_dirty = true;
            }
        } else {
            if self.widgets[id].on_cursor_leave(&mut self.cb_executor) == EventResponse::HandledRedraw {
                self.render_dirty = true;
            }
        }
    }
    pub fn cursor_moved(&mut self, p: Vec2) {
        self.cursor_pos = p;
        let mut grab = self.cursor_grabbed;
        match self.cursor_grabbed {
            GrabState::NoGrab => {
                self.rebuild_cursor_inside(p);
            }
            GrabState::GrabbedInside => {
                if let Some(i) = self.cursor_hierarchy {
                    if !self.point_in_widget(i, p) {
                        self.fire_enter_leave_event(i, false);
                        grab = GrabState::GrabbedOutside;
                    }
                }
            }
            GrabState::GrabbedOutside => {
                if let Some(i) = self.cursor_hierarchy {
                    if self.point_in_widget(i, p) {
                        self.fire_enter_leave_event(i, true);
                        grab = GrabState::GrabbedInside;
                    }
                }
            }
        };
        self.cursor_grabbed = grab;
    }
    fn pop_cursor_hierarchy(&mut self) {
        if let Some(i) = self.cursor_hierarchy {
            self.cursor_hierarchy = self.parents[i];
        }
    }
    fn complete_cursor_inside(&mut self, i: usize, p: Vec2) {
        for &id in self.widget_graph[i].iter().rev() {
            if self.point_in_widget(id, p) {
                self.cursor_hierarchy = Some(id);
                self.fire_enter_leave_event(id, true);
                self.complete_cursor_inside(id, p);
                break;
            }
        }
    }
    fn rebuild_cursor_inside(&mut self, p: Vec2) {
        let mut reduced = self.cursor_hierarchy.is_none();
        while !reduced {
            reduced = true;
            if let Some(id) = self.cursor_hierarchy {
                if !self.point_in_widget(id, p) {
                    reduced = false;
                    self.fire_enter_leave_event(id, false);
                    self.pop_cursor_hierarchy();
                }
            }
        }
        if self.cursor_hierarchy.is_none() {
            for i in (0..self.widget_count()).rev() {
                if self.parents[i].is_none() && self.point_in_widget(i, p) {
                    self.cursor_hierarchy = Some(i);
                    self.fire_enter_leave_event(i, true);
                    break;
                }
            }
        }
        if let Some(i) = self.cursor_hierarchy {
            self.complete_cursor_inside(i, p);
        }
    }
    pub fn set_profile(&mut self, enabled: bool) {
        self.profiler.set_enabled(enabled);
    }
    fn render(&mut self) {
        crate::tools::gltraits::check_glerr_debug();
        let rseq = self.render_seq.as_ref().unwrap();

        self.profiler.begin_gpu("Draw");
        rseq.execute(&mut self.draw_res);
        self.profiler.end_gpu();
    }
    fn handle_event(&mut self, event: GlutinEvent) {
        match event {
            GlutinEvent::Resized(size) => {
                self.resized(size.into());
            }
            GlutinEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                None => (),
                Some(key) => {
                    if input.state == glutin::event::ElementState::Pressed {
                        self.key_pressed(key);
                    } else {
                        self.key_released(key);
                    }
                }
            },
            GlutinEvent::MouseInput { button, state, .. } => {
                if state == glutin::event::ElementState::Pressed {
                    self.button_pressed(button);
                } else {
                    self.button_released(button);
                }
            }
            GlutinEvent::CursorMoved { position, .. } => {
                self.cursor_moved(position.into());
            }
            GlutinEvent::CursorLeft { .. } => {
                self.cursor_left();
            }
            _ => (),
        }
    }
    fn actualize_data(&mut self) {
        let build_data = self.cb_executor.data.downcast_mut::<D>().unwrap();
        
        if self.build_data != *build_data {
            self.build_data = build_data.clone();
            self.rebuild_gui();
            self.rebuild_cursor_inside(self.cursor_pos);
        }
        if self.render_dirty {
            self.rebuild_render_seq();
        }
    }
}

use std::fs::OpenOptions;

impl<D> Drop for GuiContext<D> {
    fn drop(&mut self) {
        if self.profiler.enabled() {
            let path = Path::new("performance.txt");
            let display = path.display();
            
            let mut file = match OpenOptions::new().append(true).create(true).open(&path) {
                Err(why) => panic!("couldn't create {}: {}", display, why),
                Ok(file) => file,
            };

            self.profiler.print(&mut file).unwrap();
        }
    }
}

pub struct PostBox {
    messages: Vec<AnnotatedMessage>,
}

impl PostBox {
    pub fn new() -> Self {
        PostBox { messages: vec![] }
    }
    pub fn send<T, M>(&mut self, target: T, msg: M)
    where
        T: Into<MessageTarget>,
        M: Message,
    {
        self.messages.push((target, msg).into());
    }
}
