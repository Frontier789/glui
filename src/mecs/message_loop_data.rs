use std::sync::mpsc::*;
use super::message::*;
use super::glutin_win::*;
use super::glutin_cont::*;

pub enum MessageLoopData {
    HandRolled(Sender<AnnotatedMessage>, Receiver<AnnotatedMessage>),
    GlutinWindowed(GlutinWindowData),
    GlutinWindowless(GlutinContextData),
    Consumed,
}

impl Default for MessageLoopData {
    fn default() -> MessageLoopData {
        MessageLoopData::Consumed
    }
}