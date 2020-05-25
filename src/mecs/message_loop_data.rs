use std::sync::mpsc::*;
use std::time::Duration;

use super::glutin_cont::*;
use super::glutin_win::*;
use super::message::*;

pub struct HandrolledMsgLoopData {
	pub sender: Sender<AnnotatedMessage>,
	pub receiver: Receiver<AnnotatedMessage>,
	pub update_interval: Duration,
}

pub enum MessageLoopData {
	HandRolled(HandrolledMsgLoopData),
	GlutinWindowed(GlutinWindowData),
	GlutinWindowless(GlutinContextData),
	Consumed,
}

impl Default for MessageLoopData {
	fn default() -> MessageLoopData {
		MessageLoopData::Consumed
	}
}
