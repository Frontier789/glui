extern crate downcast_rs;
use self::downcast_rs::impl_downcast;
use self::downcast_rs::Downcast;
use super::actor::*;
use super::system::*;
use std::fmt::Debug;

pub trait Message: Downcast + Debug + Send {}
impl_downcast!(Message);

#[derive(Debug)]
pub struct AnnotatedMessage {
    pub target: MessageTarget,
    pub payload: Box<dyn Message>,
}

impl<T, M> From<(T, M)> for AnnotatedMessage
where
    T: Into<MessageTarget>,
    M: Message,
{
    fn from(pair: (T, M)) -> AnnotatedMessage {
        AnnotatedMessage {
            target: pair.0.into(),
            payload: Box::new(pair.1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum MessageTarget {
    System(SystemId),
    Actor(ActorId),
    None,
}

impl From<SystemId> for MessageTarget {
    fn from(id: SystemId) -> MessageTarget {
        MessageTarget::System(id)
    }
}

impl From<ActorId> for MessageTarget {
    fn from(id: ActorId) -> MessageTarget {
        MessageTarget::Actor(id)
    }
}

#[derive(Debug)]
pub struct Exit {}
impl Message for Exit {}
