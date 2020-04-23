extern crate downcast_rs;
use self::downcast_rs::Downcast;
use self::downcast_rs::impl_downcast;

use super::DrawBuilder;
use super::CallbackExecutor;
use tools::*;

#[derive(Copy, Clone)]
pub enum WidgetSize {
    Default,
    Pixels(Vec2),
    Relative(Vec2),
    Units(Vec2px),
}

impl Default for WidgetSize {
    fn default() -> WidgetSize {
        WidgetSize::Default
    }
}

impl WidgetSize {
    pub fn to_units(&self, container_size: Vec2px) -> Vec2px {
        match *self {
            WidgetSize::Units(s) => s,
            WidgetSize::Relative(r) => {
                container_size * Vec2px::from_pixels(r, 1.0)
            }, 
            _ => container_size,
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct WidgetConstraints {
    pub max_size: Vec2px,
}

#[derive(PartialEq, Eq)]
pub enum EventResponse {
    Pass,
    Handled,
    HandledRedraw,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct WidgetPosition {
    pub pos: Vec2px,
    pub depth: f32,
}

impl From<Vec2px> for WidgetPosition {
    fn from(p: Vec2px) -> WidgetPosition {
        WidgetPosition {
            pos: p,
            depth: 0.01,
        }
    }
}

impl WidgetPosition {
    pub fn new(pos: Vec2px, depth: f32) -> WidgetPosition {
        WidgetPosition {
            pos,
            depth,
        }
    }
    
    pub fn to_pixels(self, gui_scale: f32) -> Vec3 {
        Vec3::from_vec2(
            self.pos.to_pixels(gui_scale),
            self.depth,
        )
    }
}

pub trait Widget: Downcast {
    fn constraint(&mut self, _self_constraint: WidgetConstraints) {}
    
    fn expand(&self) -> Vec<Box<dyn Widget>> {vec![]}

    fn place_child(&mut self, _child_size: Vec2px, _child_descent: f32) -> WidgetPosition {
        Vec2px::zero().into()
    }
    fn child_constraint(&self) -> Option<WidgetConstraints> {
        None
    }

    fn on_press(&mut self, _executor: &mut CallbackExecutor) -> EventResponse {
        EventResponse::Pass
    }
    fn on_release(&mut self, _executor: &mut CallbackExecutor) -> EventResponse {
        EventResponse::Pass
    }
    fn on_cursor_enter(&mut self, _executor: &mut CallbackExecutor) -> EventResponse {
        EventResponse::Pass
    }
    fn on_cursor_leave(&mut self, _executor: &mut CallbackExecutor) -> EventResponse {
        EventResponse::Pass
    }
    
    fn on_draw_build(&self, _builder: &mut DrawBuilder) {}
    fn size(&self) -> Vec2px;
}
impl_downcast!(Widget);
