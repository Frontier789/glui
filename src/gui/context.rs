use super::*;
use super::GlutinEvent;
use crate::ecs::Entity;

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
enum GrabState {
    NoGrab,
    GrabbedInside,
    GrabbedOutside,
}

pub struct GuiContext {
    render_target: RenderTarget,
    widgets: Vec<Box<dyn Widget + 'static>>,
    positions: Vec<Vec2px>,
    
    widget_with_cursor: Option<usize>,
    cursor_grabbed: GrabState,
    
    cursor_pos: Vec2,
    recheck_cursor: bool,
    
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
        
        if self.recheck_cursor {
            self.check_cursor();
        }
        
        if self.render_dirty {
            self.rebuild_render_seq();
        }
    }
    
    fn render(&self) {
        self.render_seq.as_ref().unwrap().execute();
    }
}

impl GuiContext {
    pub fn new(target: RenderTarget) -> GuiContext {
        GuiContext {
            render_target: target,
            widgets: vec![],
            positions: vec![],
            widget_with_cursor: None,
            cursor_grabbed: GrabState::NoGrab,
            cursor_pos: Vec2::new(-1.0,-1.0),
            recheck_cursor: true,
            render_seq: None,
            render_dirty: true,
        }
    }
    
    fn rebuild_render_seq(&mut self) {
        let mut drawer = WidgetDrawBuilder::new();
        
        drawer.build(&self.widgets, &self.positions);
        
        let rs = self.render_seq.take();
        
        self.render_seq = Some(drawer.builder.to_render_sequence(&self.render_target,rs));
        
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
        
        match (self.widget_with_cursor, self.cursor_grabbed) {
            (_, GrabState::GrabbedOutside) => {
                self.recheck_cursor = true;
            },
            (Some(id), GrabState::GrabbedInside) => {
                self.recheck_cursor = true;
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
        
        match self.widget_with_cursor {
            Some(id) => {
                if self.widgets[id].on_press() == EventResponse::HandledRedraw {
                    self.render_dirty = true;
                }
                self.cursor_grabbed = GrabState::GrabbedInside;
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
    
    pub fn cursor_moved(&mut self, p: Vec2) {
        self.cursor_pos = p;
        self.recheck_cursor = true;
    }
    
    fn cursor_hit_test(&self) -> std::option::Option<usize> {
        let p = self.cursor_pos;
        let scl = self.render_target.gui_scale;
        
        for i in (0..self.widget_count()).rev() {
            let pos = self.positions[i].to_pixels(scl);
            let siz = self.widgets[i].size().to_pixels(scl);
            
            if Rect::from_pos_size(pos, siz).contains(p) {
                return Some(i);
            }
        }
        
        None
    }
    
    fn check_cursor(&mut self) {
        
        let i = self.cursor_hit_test();
        
        match self.cursor_grabbed {
            GrabState::NoGrab => {
                if self.widget_with_cursor != i {
                    if let Some(id) = self.widget_with_cursor {
                        if self.widgets[id].on_cursor_leave() == EventResponse::HandledRedraw {
                            self.render_dirty = true;
                        }
                    }
                    
                    if let Some(id) = i {
                        if self.widgets[id].on_cursor_enter() == EventResponse::HandledRedraw {
                            self.render_dirty = true;
                        }
                    }
                    
                    self.widget_with_cursor = i;
                }
            },
            GrabState::GrabbedInside => {
                if self.widget_with_cursor != i {
                    self.cursor_grabbed = GrabState::GrabbedOutside;
                }
            },
            GrabState::GrabbedOutside => {
                if self.widget_with_cursor == i {
                    self.cursor_grabbed = GrabState::GrabbedInside;
                }
            },
        }
        
        self.recheck_cursor = false;
    }
}
