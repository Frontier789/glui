extern crate downcast_rs;

use super::Message;
use super::StaticWorld;

use self::downcast_rs::impl_downcast;
use self::downcast_rs::Downcast;
use mecs::{GlutinDeviceEvent, GlutinWindowEvent};
use std::any::TypeId;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct SystemId {
    pub(super) type_id: TypeId,
    pub(super) number: usize,
}

impl SystemId {
    pub(super) fn from_number<S>(number: usize) -> SystemId
    where
        S: System,
    {
        SystemId {
            type_id: TypeId::of::<S>(),
            number,
        }
    }
}

pub trait System: Downcast {
    fn receive(&mut self, _msg: &Box<dyn Message>, _world: &mut StaticWorld) {}
    fn update(&mut self, _delta_time: Duration, _world: &mut StaticWorld) {}
    fn render(&mut self, _world: &mut StaticWorld) {}
    fn window_event(&mut self, _event: &GlutinWindowEvent, _world: &mut StaticWorld) {}
    fn device_event(&mut self, _event: &GlutinDeviceEvent, _world: &mut StaticWorld) {}
}

impl_downcast!(System);
