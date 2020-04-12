use super::Message;
use super::StaticWorld;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct ActorId(pub(super) usize);

pub trait Actor {
    fn receive(&mut self, _msg: Box<dyn Message>, _world: &mut StaticWorld) {}
}
