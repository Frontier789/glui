extern crate downcast_rs;

use tools::*;

use super::CallbackExecutor;
use super::DrawBuilder;

use self::downcast_rs::impl_downcast;
use self::downcast_rs::Downcast;

#[derive(Debug, Copy, Clone, PartialEq)]
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
    pub fn to_units(self, container_dimension: f32) -> f32 {
        match self {
            GuiDimension::Units(s) => s,
            GuiDimension::Relative(r) => container_dimension * r,
            _ => container_dimension,
        }
    }
    pub fn relative(self) -> f32 {
        match self {
            GuiDimension::Units(_) => 0.0,
            GuiDimension::Relative(r) => r,
            GuiDimension::Default => 1.0,
        }
    }
    pub fn absolute(self) -> f32 {
        match self {
            GuiDimension::Units(s) => s,
            _ => 0.0,
        }
    }
    pub fn relative_array(array: Vec<f32>) -> Vec<GuiDimension> {
        array.iter().map(|f| GuiDimension::Relative(*f)).collect()
    }
}

#[derive(Copy, Clone)]
pub struct WidgetSize {
    pub x: GuiDimension,
    pub y: GuiDimension,
}

pub type GuiPoint = WidgetSize;

impl Default for WidgetSize {
    fn default() -> WidgetSize {
        WidgetSize {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl From<Vec2px> for WidgetSize {
    fn from(s: Vec2px) -> Self {
        WidgetSize {
            x: GuiDimension::Units(s.x),
            y: GuiDimension::Units(s.y),
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
    pub fn from_units(units: Vec2) -> Self {
        Self {
            x: GuiDimension::Units(units.x),
            y: GuiDimension::Units(units.y),
        }
    }
    pub fn relative(ratio: Vec2) -> Self {
        Self {
            x: GuiDimension::Relative(ratio.x),
            y: GuiDimension::Relative(ratio.y),
        }
    }
    pub fn fill() -> Self {
        WidgetSize {
            x: GuiDimension::Relative(1.0),
            y: GuiDimension::Relative(1.0),
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct WidgetConstraints {
    pub max_size: Vec2px,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EventResponse {
    Pass,
    Handled,
    HandledRedraw,
    HandledRebuild,
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

    fn place_child(&mut self, _child_size: Vec2px, _child_descent: f32) -> WidgetPosition {
        Vec2px::zero().into()
    }
    fn child_constraint(&self) -> Option<WidgetConstraints> {
        None
    }

    fn on_press(
        &mut self,
        _local_cursor_pos: Vec2px,
        _executor: &mut CallbackExecutor,
    ) -> EventResponse {
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
    fn on_cursor_move(
        &mut self,
        _local_cursor_pos: Vec2px,
        _executor: &mut CallbackExecutor,
    ) -> EventResponse {
        EventResponse::Pass
    }
    fn on_draw_build(&self, _builder: &mut DrawBuilder) {}
    fn size(&self) -> Vec2px;
}

impl_downcast!(Widget);

impl Widget for () {
    fn size(&self) -> Vec2px {
        Vec2px::zero()
    }
}

trait WidgetClone {
    fn clone_widget(&self) -> Box<dyn Widget + 'static>;
}
impl<T> WidgetClone for T
where
    T: Widget + Clone,
{
    fn clone_widget(&self) -> Box<dyn Widget + 'static> {
        Box::new(self.clone())
    }
}
