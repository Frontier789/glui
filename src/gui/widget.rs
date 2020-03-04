use crate::downcast_rs::Downcast;

use super::DrawBuilder;
use tools::*;

#[derive(Copy, Clone)]
pub enum WidgetSize {
    Pixels(Vec2),
    Relative(Vec2),
    Units(Vec2px),
}

#[derive(Copy, Clone, Default, Debug)]
pub struct WidgetConstraints {
    pub max_size: Vec2px,
}

pub trait Widget: Downcast {
    fn constraint(&mut self, _self_constraint: WidgetConstraints) {}

    fn place_child(&mut self, _child_size: Vec2px) -> Vec2px {
        Vec2px::zero()
    }
    fn child_constraint(&self) -> Option<WidgetConstraints> {
        None
    }
    fn start_layout(&mut self) {}
    fn on_draw_build(&self, _builder: &mut DrawBuilder) {}
    fn size(&self) -> Vec2px;
}
impl_downcast!(Widget);
