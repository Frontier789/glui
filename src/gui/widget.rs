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

#[derive(PartialEq,Eq)]
pub enum EventResponse {
    Pass,
    Handled,
    HandledRedraw,
}

pub trait Widget: Downcast {
    fn constraint(&mut self, _self_constraint: WidgetConstraints) {}

    fn place_child(&mut self, _child_size: Vec2px) -> Vec2px {
        Vec2px::zero()
    }
    
    fn child_constraint(&self) -> Option<WidgetConstraints> {
        None
    }
    
    fn on_press(&mut self) -> EventResponse { EventResponse::Pass }
    fn on_release(&mut self) -> EventResponse { EventResponse::Pass }
    fn on_cursor_enter(&mut self) -> EventResponse { EventResponse::Pass }
    fn on_cursor_leave(&mut self) -> EventResponse { EventResponse::Pass }
    
    fn start_layout(&mut self) {}
    fn on_draw_build(&self, _builder: &mut DrawBuilder) {}
    fn size(&self) -> Vec2px;
}
impl_downcast!(Widget);
