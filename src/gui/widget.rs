extern crate downcast_rs;
use self::downcast_rs::impl_downcast;
use self::downcast_rs::Downcast;

use super::CallbackExecutor;
use super::DrawBuilder;
use tools::*;

#[derive(Copy, Clone)]
pub enum GuiDimension {
    Default,
    Relative(f32),
    Units(f32),
}

impl Default for GuiDimension {
    fn default() -> GuiDimension {
        GuiDimension::Default
    }
}

impl GuiDimension {
    pub fn to_units(&self, container_dimension: f32) -> f32 {
        match *self {
            GuiDimension::Units(s) => s,
            GuiDimension::Relative(r) => container_dimension * r,
            _ => container_dimension,
        }
    }
}

#[derive(Copy, Clone)]
pub struct WidgetSize {
    pub x: GuiDimension,
    pub y: GuiDimension,
}

impl Default for WidgetSize {
    fn default() -> WidgetSize {
        WidgetSize {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl WidgetSize {
    pub fn to_units(&self, container_size: Vec2px) -> Vec2px {
        Vec2px::new(
            self.x.to_units(container_size.x),
            self.y.to_units(container_size.y),
        )
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
        WidgetPosition { pos, depth }
    }
    pub fn to_pixels(self, gui_scale: f32) -> Vec3 {
        Vec3::from_vec2(self.pos.to_pixels(gui_scale), self.depth)
    }
}

pub trait Widget: Downcast {
    fn constraint(&mut self, _self_constraint: WidgetConstraints) {}

    fn expand(&self) -> Vec<Box<dyn Widget>> {
        vec![]
    }
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
