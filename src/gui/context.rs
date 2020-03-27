use super::*;
use super::GlutinEvent;
use crate::ecs::Entity;
use std::time::Instant;

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
enum GrabState {
    NoGrab,
    GrabbedInside,
    GrabbedOutside,
}

pub struct GuiContext {
    render_target: RenderTarget,
    draw_res: DrawResources,
    widgets: Vec<Box<dyn Widget + 'static>>,
    parents: Vec<Option<usize>>,
    child_count: Vec<usize>,
    widget_depth: Vec<usize>,
    positions: Vec<Vec2px>,
    
    active_widget: Option<usize>,
    cursor_hierarchy: Option<usize>,
    cursor_grabbed: GrabState,
    
    cursor_pos: Vec2,
    
    render_seq: Option<RenderSequence>,
    
    render_dirty: bool,
}

impl Entity for GuiContext {
    fn handle_event(&mut self, event: &GlutinEvent) {
        match *event {
            glutin::event::WindowEvent::Resized(size) => {
                self.resized(size.into());
            },
            glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                match input.virtual_keycode {
                    None => (),
                    Some(key) => {
                        if input.state == glutin::event::ElementState::Pressed {
                            self.key_pressed(key);
                        } else {
                            self.key_released(key);
                        }
                    }
                }
            },
            glutin::event::WindowEvent::MouseInput { button, state, .. } => {
                if state == glutin::event::ElementState::Pressed {
                    self.button_pressed(button);
                } else {
                    self.button_released(button);
                }
            },
            glutin::event::WindowEvent::CursorMoved { position, .. } => {
                self.cursor_moved(position.into());
            },
            glutin::event::WindowEvent::CursorLeft { .. } => {
                self.cursor_left();
            },
            _ => (),
        }
        
        if self.render_dirty {
            let i = Instant::now();
            self.rebuild_render_seq();
            let d = i.elapsed();
            unsafe {
                gl::Flush();
                gl::Finish();
            }
            println!("render: {}",d.as_secs_f64());
        }
    }
    
    fn render(&mut self) {
        let rseq = self.render_seq.as_ref().unwrap();
        rseq.execute(&mut self.draw_res);
    }
}

impl GuiContext {
    pub fn new(target: RenderTarget) -> GuiContext {
        GuiContext {
            render_target: target,
            widgets: vec![],
            parents: vec![],
            child_count: vec![],
            widget_depth: vec![],
            positions: vec![],
            cursor_hierarchy: None,
            active_widget: None,
            cursor_grabbed: GrabState::NoGrab,
            cursor_pos: Vec2::new(-1.0,-1.0),
            render_seq: None,
            render_dirty: true,
            draw_res: DrawResources::new(),
        }
    }
    
    pub fn init_gl_res(&mut self) {
        self.draw_res.create_defaults().unwrap();
    }
    
    fn rebuild_render_seq(&mut self) {
        let mut builder = DrawBuilder::new(&mut self.draw_res);
        
        let n = self.widgets.len();
        for i in 0..n {
            
            builder.offset = Vec3::from_vec2(self.positions[i].to_pixels(1.0), self.widget_depth[i] as f32 * 0.01);
            self.widgets[i].on_draw_build(&mut builder);
        }
    
        self.render_seq = Some(builder.to_render_sequence(&self.render_target));
        
        self.render_dirty = false;
    }
    
    pub fn build_gui<F,D>(&mut self, parse_fun: F, parse_data: D)
        where F: Fn(&mut WidgetTreeToList, D)
    {
        let mut parser = WidgetTreeToList::new();
        parse_fun(&mut parser, parse_data);

        let mut layout_builder =
            WidgetLayoutBuilder::new(parser.widgets, parser.postorder, parser.child_count);
        
        layout_builder.build(self.render_target.logical_size());
        layout_builder.make_pos_abs(0, Vec2px::origin());
        
        self.child_count = layout_builder.child_count;
        self.parents = parser.parents;
        self.widget_depth = parser.widget_depth;
        self.widgets = layout_builder.widgets;
        self.positions = layout_builder.positions;
        
        self.rebuild_render_seq();
    }
    
    pub fn resized(&mut self, s: Vec2) {
        self.render_target.size = s;
        
        self.rebuild_render_seq();
    }
    
    pub fn widget_count(&self) -> usize {
        self.widgets.len()
    }
    
    pub fn button_released(&mut self, button: GlutinButton) {
        if button != GlutinButton::Left { return; }
        
        match (self.active_widget, self.cursor_grabbed) {
            (_, GrabState::GrabbedOutside) => {
                self.rebuild_cursor_inside(self.cursor_pos);
            },
            (Some(id), GrabState::GrabbedInside) => {
                if self.widgets[id].on_release() == EventResponse::HandledRedraw {
                    self.render_dirty = true;
                }
            },
            _ => {}
        }
        
        self.cursor_grabbed = GrabState::NoGrab;
    }
    
    pub fn button_pressed(&mut self, button: GlutinButton) {
        if button != GlutinButton::Left { return; }
        
        match self.cursor_hierarchy {
            Some(mut id) => {
                let mut result = self.widgets[id].on_press();
                
                while result == EventResponse::Pass {
                    if let Some(parent) = self.parents[id] {
                        result = self.widgets[parent].on_press();
                        id = parent;
                    } else {
                        break;
                    }
                }
                
                if result != EventResponse::Pass {
                    self.active_widget  = Some(id);
                    self.cursor_grabbed = GrabState::GrabbedInside;
                }
                
                if result == EventResponse::HandledRedraw {
                    self.render_dirty = true;
                }
            },
            _ => {}
        }
    }
    
    pub fn key_pressed(&mut self, _key: GlutinKey) {
        
    }
    
    pub fn key_released(&mut self, _key: GlutinKey) {
        
    }
    
    pub fn cursor_left(&mut self) {
        self.cursor_moved(Vec2::new(-1.0, -1.0));
    }
    
    fn point_in_widget(&self, id: usize, p: Vec2) -> bool {
        let scl = self.render_target.gui_scale;
        let pos = self.positions[id].to_pixels(scl);
        let siz = self.widgets[id].size().to_pixels(scl);
        
        Rect::from_pos_size(pos, siz).contains(p)
    }
    
    fn fire_enter_leave_event(&mut self, id: usize, enter: bool) {
        if enter {
            if self.widgets[id].on_cursor_enter() == EventResponse::HandledRedraw {
                self.render_dirty = true;
            }
        } else {
            if self.widgets[id].on_cursor_leave() == EventResponse::HandledRedraw {
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
            },
            GrabState::GrabbedInside => {
                if let Some(i) = self.cursor_hierarchy {
                    if !self.point_in_widget(i, p) {
                        self.fire_enter_leave_event(i, false);
                        grab = GrabState::GrabbedOutside;
                    }
                }
            },
            GrabState::GrabbedOutside => {
                if let Some(i) = self.cursor_hierarchy {
                    if self.point_in_widget(i, p) {
                        self.fire_enter_leave_event(i, true);
                        grab = GrabState::GrabbedInside;
                    }
                }
            },
        };
        
        self.cursor_grabbed = grab;
    }
    
    fn pop_cursor_hierarchy(&mut self) {
        if let Some(i) = self.cursor_hierarchy {
            self.cursor_hierarchy = self.parents[i];
        }
    }
    
    fn complete_cursor_inside(&mut self, mut i: usize, p: Vec2) {
        let c = self.child_count[i];
        
        i += 1;
        
        for _ in 0..c {
            if self.point_in_widget(i, p) {
                self.cursor_hierarchy = Some(i);
                self.fire_enter_leave_event(i, true);
                self.complete_cursor_inside(i, p);
                break;
            }
            i += self.child_count[i] + 1;
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
            let mut i = 0;
            while i < self.widget_count() {
                if self.point_in_widget(i, p) {
                    self.cursor_hierarchy = Some(i);
                    self.fire_enter_leave_event(i, true);
                    break;
                }
                i += self.child_count[i] + 1;
            }
        }
        
        if let Some(i) = self.cursor_hierarchy {
            self.complete_cursor_inside(i,p);
        }
    }
}
