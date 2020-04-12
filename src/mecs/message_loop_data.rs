use std::sync::mpsc::*;
use super::*;

pub enum MessageLoopData {
    HandRolled(Sender<AnnotatedMessage>, Receiver<AnnotatedMessage>),
    Glutin(GlutinWindow),
    Consumed,
}
