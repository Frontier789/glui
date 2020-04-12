use super::Message;
use super::StaticWorld;

use std::any::TypeId;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct SystemId(TypeId);

impl SystemId {
    pub fn of<S>() -> SystemId
    where
        S: 'static,
    {
        SystemId(TypeId::of::<S>())
    }
}

pub trait System {
    fn receive(&mut self, _msg: Box<dyn Message>, _world: &mut StaticWorld) {}
}
