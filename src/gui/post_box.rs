use mecs::{AnnotatedMessage, MessageTarget, Message};
use std::vec::Drain;

pub struct PostBox {
    messages: Vec<AnnotatedMessage>,
}

impl PostBox {
    pub fn new() -> Self {
        PostBox { messages: vec![] }
    }
    pub fn send<T, M>(&mut self, target: T, msg: M)
    where
        T: Into<MessageTarget>,
        M: Message,
    { 
        self.messages.push((target, msg).into());
    }
    pub fn send_root<M>(&mut self, msg: M)
    where
        M: Message,
    {
        self.messages.push((MessageTarget::Root, msg).into());
    }
    pub fn drain_messages(&mut self) -> Drain<AnnotatedMessage> {
        self.messages.drain(0..)
    }
}
