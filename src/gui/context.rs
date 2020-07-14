use std::fs::OpenOptions;
use std::path::Path;

use graphics::*;
use gui::{CallbackExecutor, GuiBuilder, WidgetParser};
use mecs::*;
use tools::*;

use super::draw::*;
use super::widget::*;
use super::widget_layout_builder::*;
use std::time::Duration;

pub struct GuiContext<D>
where
    D: GuiBuilder + 'static,
{
    draw_res: DrawResources,
    widgets: Vec<Box<dyn Widget + 'static>>,
    parents: Vec<Option<usize>>,
    widget_graph: Vec<Vec<usize>>,
    widget_depth: Vec<usize>,
    positions: Vec<WidgetPosition>,
    active_widget: Option<usize>,
    cursor_hierarchy: Option<usize>,
    cursor_grabbed: bool,
    cursor_pos: Vec2px,
    render_seq: Option<RenderSequence>,
    render_dirty: bool,
    build_dirty: bool,
    profiler: Profiler,
    gui_builder: D,
    gui_builder_new: D,
}

impl<D> System for GuiContext<D>
where
    D: GuiBuilder + 'static,
{
    fn receive(&mut self, msg: &Box<dyn Message>, world: &mut StaticWorld) {
        self.gui_builder_new.receive(msg, world);
    }

    fn update(&mut self, delta_time: Duration, world: &mut StaticWorld) {
        self.gui_builder_new.update(delta_time, world);
    }

    fn render(&mut self, world: &mut StaticWorld) {
        self.actualize_data(world); // FIXME: actualize after events only

        crate::tools::gltraits::check_glerr_debug();
        let rseq = self.render_seq.as_mut().unwrap();

        self.profiler.begin_gpu("Draw");
        rseq.execute(&mut self.draw_res);
        self.profiler.end_gpu();
    }
    fn window_event(&mut self, event: &GlutinWindowEvent, world: &mut StaticWorld) -> bool {
        let handled = match event {
            GlutinWindowEvent::Resized(size) => {
                self.resized(size.into(), world);
                false
            }
            GlutinWindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                None => false,
                Some(key) => {
                    if input.state == glutin::event::ElementState::Pressed {
                        self.key_pressed(key)
                    } else {
                        self.key_released(key)
                    }
                }
            },
            GlutinWindowEvent::MouseInput { button, state, .. } => {
                if *state == glutin::event::ElementState::Pressed {
                    self.button_pressed(*button, world)
                } else {
                    self.button_released(*button, world)
                }
            }
            GlutinWindowEvent::CursorMoved { position, .. } => {
                let scl = self.draw_res.window_info.gui_scale;
                self.cursor_moved(Vec2px::from_pixels(position.into(), scl), world)
            }
            GlutinWindowEvent::CursorLeft { .. } => self.cursor_left(world),
            _ => false,
        };

        handled
    }
}

impl<D> GuiContext<D>
where
    D: GuiBuilder + 'static,
{
    pub fn new(
        target: WindowInfo,
        profile: bool,
        gui_builder: D,
        world: &mut StaticWorld,
    ) -> GuiContext<D> {
        let mut gui_context = GuiContext {
            widgets: vec![],
            parents: vec![],
            widget_graph: vec![],
            widget_depth: vec![],
            positions: vec![],
            cursor_hierarchy: None,
            active_widget: None,
            cursor_grabbed: false,
            cursor_pos: Vec2px::new(-1.0, -1.0),
            render_seq: None,
            render_dirty: true,
            build_dirty: true,
            draw_res: DrawResources::new(target).unwrap(),
            profiler: Profiler::new(profile),
            gui_builder_new: gui_builder.clone(),
            gui_builder,
        };
        gui_context.update_projection_matrix();
        gui_context.rebuild_gui(world);
        gui_context
    }
    fn update_projection_matrix(&mut self) {
        self.draw_res.projection_matrix = Mat4::ortho(
            0.0,
            self.draw_res.window_info.size.y,
            self.draw_res.window_info.size.x,
            0.0,
            1.0,
            -1.0,
        );
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
        self.render_seq = Some(builder.into_render_sequence());
        self.render_dirty = false;
        self.profiler.end();
    }

    pub fn rebuild_gui(&mut self, world: &mut StaticWorld) {
        crate::tools::gltraits::check_glerr_debug();
        self.profiler.begin("Rebuild_Gui");
        let widget_list = WidgetParser::produce_list(&self.gui_builder);
        // println!("cache_details is {:?}",widget_list.cache_details);
        // println!("cache_loc is {:?}",widget_list.cache_loc);

        let mut layout_builder = WidgetLayoutBuilder::new(
            widget_list.widgets,
            widget_list.postorder,
            widget_list.widget_graph,
        );
        layout_builder.build(self.draw_res.window_info.logical_size());

        self.widgets = layout_builder.widgets;
        self.cursor_hierarchy = None;
        self.active_widget = None;

        self.build_dirty = false;
        self.widget_graph = layout_builder.widget_graph;
        self.parents = widget_list.parents;
        self.widget_depth = widget_list.widget_depth;
        self.positions = layout_builder.positions;
        self.profiler.end();
        self.rebuild_render_seq();

        self.rebuild_cursor_inside(world);
    }
    pub fn resized(&mut self, s: Vec2, world: &mut StaticWorld) {
        self.draw_res.window_info.size = s;
        self.update_projection_matrix();
        self.rebuild_gui(world);
    }
    pub fn widget_count(&self) -> usize {
        self.widgets.len()
    }
    fn handle_event_response(&mut self, response: EventResponse) {
        match response {
            EventResponse::HandledRedraw => {
                self.render_dirty = true;
            }
            EventResponse::HandledRebuild => {
                self.build_dirty = true;
            }
            EventResponse::Handled => {}
            EventResponse::Pass => {}
        }
    }
    pub fn button_released(&mut self, button: GlutinButton, world: &mut StaticWorld) -> bool {
        if button != GlutinButton::Left {
            return false;
        }
        self.rebuild_cursor_inside(world);

        if self.cursor_grabbed {
            self.cursor_grabbed = false;

            if let Some(id) = self.active_widget {
                let response =
                    self.widgets[id].on_release(&mut (&mut self.gui_builder_new, world).into());
                self.handle_event_response(response);
                response != EventResponse::Pass
            } else {
                false
            }
        } else {
            false
        }
    }
    pub fn button_pressed(&mut self, button: GlutinButton, world: &mut StaticWorld) -> bool {
        if button != GlutinButton::Left {
            return false;
        }
        match self.cursor_hierarchy {
            Some(mut id) => {
                let wpos = self.positions[id].pos;
                let mut cb_exec: CallbackExecutor = (&mut self.gui_builder_new, world).into();
                let mut result = self.widgets[id].on_press(self.cursor_pos - wpos, &mut cb_exec);
                while result == EventResponse::Pass {
                    if let Some(parent) = self.parents[id] {
                        let wpos = self.positions[parent].pos;
                        result =
                            self.widgets[parent].on_press(self.cursor_pos - wpos, &mut cb_exec);
                        id = parent;
                    } else {
                        break;
                    }
                }
                if result != EventResponse::Pass {
                    self.active_widget = Some(id);
                    self.cursor_grabbed = true;
                }
                self.handle_event_response(result);

                result != EventResponse::Pass
            }
            None => false,
        }
    }

    pub fn key_pressed(&mut self, _key: GlutinKey) -> bool {
        false
    }

    pub fn key_released(&mut self, _key: GlutinKey) -> bool {
        false
    }
    pub fn cursor_left(&mut self, world: &mut StaticWorld) -> bool {
        self.cursor_moved(Vec2px::new(-1.0, -1.0), world)
    }
    fn point_in_widget(&self, id: usize, p: Vec2px) -> bool {
        let pos = self.positions[id].pos;
        let siz = self.widgets[id].size();
        Rect::from_pos_size(pos.as_vec2(), siz.as_vec2()).contains(p.as_vec2())
    }
    fn fire_enter_event(&mut self, id: usize, world: &mut StaticWorld) {
        let response =
            self.widgets[id].on_cursor_enter(&mut (&mut self.gui_builder_new, world).into());
        self.handle_event_response(response);
    }
    fn fire_leave_event(&mut self, id: usize, world: &mut StaticWorld) {
        let response =
            self.widgets[id].on_cursor_leave(&mut (&mut self.gui_builder_new, world).into());
        self.handle_event_response(response);
    }
    fn fire_move_event(&mut self, id: usize, pos: Vec2px, world: &mut StaticWorld) {
        let widget_pos = self.positions[id].pos;
        let response = self.widgets[id].on_cursor_move(
            pos - widget_pos,
            &mut (&mut self.gui_builder_new, world).into(),
        );
        self.handle_event_response(response);
    }
    pub fn cursor_moved(&mut self, p: Vec2px, world: &mut StaticWorld) -> bool {
        self.cursor_pos = p;
        if !self.cursor_grabbed {
            self.rebuild_cursor_inside(world);
            false
        } else if let Some(i) = self.cursor_hierarchy {
            self.fire_move_event(i, p, world);
            true
        } else {
            false
        }
    }
    fn pop_cursor_hierarchy(&mut self) {
        if let Some(i) = self.cursor_hierarchy {
            self.cursor_hierarchy = self.parents[i];
        }
    }
    fn complete_cursor_inside(&mut self, i: usize, world: &mut StaticWorld) {
        for &id in self.widget_graph[i].iter().rev() {
            if self.point_in_widget(id, self.cursor_pos) {
                self.cursor_hierarchy = Some(id);
                self.fire_enter_event(id, world);
                self.complete_cursor_inside(id, world);
                break;
            }
        }
    }
    fn rebuild_cursor_inside(&mut self, world: &mut StaticWorld) {
        let mut reduced = self.cursor_hierarchy.is_none();
        while !reduced {
            reduced = true;
            if let Some(id) = self.cursor_hierarchy {
                if !self.point_in_widget(id, self.cursor_pos) {
                    reduced = false;
                    self.fire_leave_event(id, world);
                    self.pop_cursor_hierarchy();
                }
            }
        }
        if self.cursor_hierarchy.is_none() {
            for i in (0..self.widget_count()).rev() {
                if self.parents[i].is_none() && self.point_in_widget(i, self.cursor_pos) {
                    self.cursor_hierarchy = Some(i);
                    self.fire_enter_event(i, world);
                    break;
                }
            }
        }
        if let Some(i) = self.cursor_hierarchy {
            self.complete_cursor_inside(i, world);
        }
    }

    pub fn set_profile(&mut self, enabled: bool) {
        self.profiler.set_enabled(enabled);
    }

    fn actualize_data(&mut self, world: &mut StaticWorld) {
        if (self.gui_builder != self.gui_builder_new || self.build_dirty) && !self.cursor_grabbed {
            self.gui_builder = self.gui_builder_new.clone();
            self.rebuild_gui(world);
            self.rebuild_cursor_inside(world);
        }
        if self.render_dirty {
            self.rebuild_render_seq();
        }
    }
}

impl<D> Drop for GuiContext<D>
where
    D: GuiBuilder,
{
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
