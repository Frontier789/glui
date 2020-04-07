use super::Message;
use super::World;

pub trait System {
    fn receive(&mut self, _msg: Box<dyn Message>, _world: &mut World) {}
}
