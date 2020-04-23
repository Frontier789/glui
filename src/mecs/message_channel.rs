use std::sync::mpsc::*;
use super::*;
type GlutinEventLoopProxy = glutin::event_loop::EventLoopProxy<AnnotatedMessage>;

#[derive(Clone)]
enum ChannelImpl {
    HandRolled(Sender<AnnotatedMessage>),
    Glutin(GlutinEventLoopProxy),
}

#[derive(Clone)]
pub struct MessageChannel {
    implementation: ChannelImpl,
}

impl MessageChannel {
    pub fn from_sender(sender: Sender<AnnotatedMessage>) -> MessageChannel {
        MessageChannel {
            implementation: ChannelImpl::HandRolled(sender)
        }
    }
    
    pub fn from_window(win: &GlutinWindowData) -> MessageChannel {
        MessageChannel {
            implementation: ChannelImpl::Glutin(win.event_loop_proxy())
        }
    }
    
    pub fn send<T,M>(&self, target: T, message: M) -> Result<(),AnnotatedMessage>
    where
        T: Into<MessageTarget>,
        M: Message,
    {
        match &self.implementation {
            ChannelImpl::HandRolled(sender) => {
                match sender.send((target, message).into()) {
                    Ok(_) => Ok(()),
                    Err(SendError(msg)) => Err(msg),
                }
            },
            ChannelImpl::Glutin(proxy) => {
                match proxy.send_event((target, message).into()) {
                    Ok(_) => Ok(()),
                    Err(glutin::event_loop::EventLoopClosed(msg)) => Err(msg),
                }
            }
        }
    }
}
