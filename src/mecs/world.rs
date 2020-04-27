extern crate gl;
extern crate glutin;

use self::glutin::platform::desktop::EventLoopExtDesktop;
use super::actor::*;
use super::glutin_util::*;
use super::glutin_cont::*;
use super::glutin_win::*;
use super::message;
use super::message::*;
use super::message_channel::*;
use super::message_loop_data::*;
use super::render_target::*;
use super::static_world::*;
use super::system::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::mem::swap;
use std::sync::mpsc::*;
use tools::*;

#[derive(Default)]
pub struct World {
    static_world: StaticWorld,
    systems: HashMap<SystemId, Box<dyn System>>,
    actors: HashMap<ActorId, Box<dyn Actor>>,
    next_actor_id: usize,
    loop_data: MessageLoopData,
    ui_aware_actors: HashSet<ActorId>,
}

fn clone_glutin_event(event: &GlutinEvent<'static>) -> GlutinEvent<'static> {
    let mut cloned = GlutinEvent::<'static>::CloseRequested;
    unsafe {
        std::ptr::copy_nonoverlapping(event, &mut cloned, 1);
    }
    cloned
}

impl World {
    pub fn new() -> World {
        let (sender, receiver) = channel();
        World {
            loop_data: MessageLoopData::HandRolled(sender, receiver),
            ..Default::default()
        }
    }
    pub fn new_win(size: Vec2, title: &str, bgcolor: Vec3) -> World {
        World {
            loop_data: MessageLoopData::GlutinWindowed(GlutinWindowData::new(size, title, bgcolor)),
            ..Default::default()
        }
    }
    pub fn new_winless(bgcolor: Vec3) -> Result<World,glutin::CreationError> {
        Ok(World {
            loop_data: MessageLoopData::GlutinWindowless(GlutinContextData::new(bgcolor)?),
            ..Default::default()
        })
    }
    pub fn render_target(&self) -> Option<RenderTarget> {
        match &self.loop_data {
            MessageLoopData::GlutinWindowed(win) => Some(win.render_target()),
            MessageLoopData::GlutinWindowless(cont) => Some(cont.render_target()),
            _ => None,
        }
    }
    pub fn as_static(&self) -> &StaticWorld {
        &self.static_world
    }
    pub fn as_static_mut(&mut self) -> &mut StaticWorld {
        &mut self.static_world
    }

    pub fn add_system<S>(&mut self, system: S) -> SystemId
    where
        S: System + 'static,
    {
        let id = SystemId::of::<S>();
        self.systems.insert(id, Box::new(system));
        id
    }
    fn next_actor_id(&mut self) -> ActorId {
        let id = ActorId(self.next_actor_id);
        self.next_actor_id += 1;
        id
    }
    pub fn add_actor<A>(&mut self, actor: A) -> ActorId
    where
        A: Actor + 'static,
    {
        let id = self.next_actor_id();
        self.actors.insert(id, Box::new(actor));
        id
    }
    pub fn del_actor(&mut self, id: ActorId) {
        self.actors.remove(&id);
        self.ui_aware_actors.remove(&id);
    }
    pub fn make_actor_ui_aware(&mut self, id: ActorId) {
        self.ui_aware_actors.insert(id);
    }

    pub fn deliver_all_messages(&mut self) {
        while !self.static_world.queued_messages.is_empty() {
            self.deliver_messages();
        }
    }
    fn deliver_messages(&mut self) {
        let mut messages = vec![];
        swap(&mut self.static_world.queued_messages, &mut messages);
        for msg in messages.drain(0..) {
            match msg.target {
                MessageTarget::System(id) => {
                    self.systems
                        .get_mut(&id)
                        .expect(&format!(
                            "Message {:?} sent to non-existing system!",
                            msg.payload
                        ))
                        .receive(msg.payload, &mut self.static_world);
                }
                MessageTarget::Actor(id) => {
                    self.actors
                        .get_mut(&id)
                        .expect(&format!(
                            "Message {:?} sent to non-existing actor!",
                            msg.payload
                        ))
                        .receive(msg.payload, &mut self.static_world);
                }
                MessageTarget::None => {}
            }
        }
    }
    pub fn channel(&mut self) -> MessageChannel {
        match &self.loop_data {
            MessageLoopData::HandRolled(sender, _receiver) => {
                MessageChannel::from_sender(sender.clone())
            }
            MessageLoopData::GlutinWindowed(win) => MessageChannel::from_glutin(win.event_loop_proxy()),
            MessageLoopData::GlutinWindowless(cont) => MessageChannel::from_glutin(cont.event_loop_proxy()),
            MessageLoopData::Consumed => {
                panic!("Cannot give channel to consumed message loop data!")
            }
        }
    }
    pub fn run(self) {
        match self.loop_data {
            MessageLoopData::HandRolled(_, _) => {
                self.owned_msg_loop();
            }
            MessageLoopData::GlutinWindowed(_) => {
                self.glutin_win_msg_loop();
            }
            MessageLoopData::GlutinWindowless(_) => {
                self.glutin_cont_msg_loop();
            }
            MessageLoopData::Consumed => {}
        }
    }
    fn owned_msg_loop(mut self) {
        let mut loop_data = MessageLoopData::Consumed;
        std::mem::swap(&mut loop_data, &mut self.loop_data);
        if let MessageLoopData::HandRolled(_, receiver) = loop_data {
            let mut finished = false;
            while !finished {
                let msg = receiver.recv().unwrap();
                println!("msg loop got: {:?}", msg);
                if msg.target == MessageTarget::None {
                    if msg.payload.is::<message::Exit>() {
                        finished = true;
                    }
                } else {
                    self.static_world.send_annotated(msg);
                    self.deliver_all_messages();
                }
            }
        }
    }
    fn send_event_to_ui_aware<'a>(&mut self, event: GlutinEvent<'a>) {
        if let Some(event) = event.to_static() {
            for &id in &self.ui_aware_actors {
                self.static_world
                    .send(id, UiEvent::GlutinEvent(clone_glutin_event(&event)));
            }
        }
    }
    fn send_to_ui_aware<'a>(&mut self, event: ClonableUiEvent) {
        for &id in &self.ui_aware_actors {
            self.static_world.send(id, UiEvent::from(event));
        }
    }
    fn glutin_cont_msg_loop(mut self) {
        use self::glutin::event_loop::ControlFlow;
        let mut loop_data = MessageLoopData::Consumed;
        std::mem::swap(&mut loop_data, &mut self.loop_data);
        
        if let MessageLoopData::GlutinWindowless(data) = loop_data {
            let (mut event_loop, _context, _bgcolor) = data.unpack();
            
            event_loop.run_return(move |event, _, control_flow| {
                *control_flow = ControlFlow::Wait;
                match event {
                    glutin::event::Event::UserEvent(msg) => {
                        if msg.target == MessageTarget::None {
                            if msg.payload.is::<message::Exit>() {
                                *control_flow = ControlFlow::Exit;
                            }
                        } else {
                            self.static_world.send_annotated(msg);
                        }
                    }
                    glutin::event::Event::MainEventsCleared => {
                        self.send_to_ui_aware(ClonableUiEvent::EventsCleared);
                        self.deliver_all_messages();
                    }
                    _ => (),
                }
            });
        }
    }
    fn glutin_win_msg_loop(mut self) {
        use self::glutin::event_loop::ControlFlow;
        let mut loop_data = MessageLoopData::Consumed;
        std::mem::swap(&mut loop_data, &mut self.loop_data);
        if let MessageLoopData::GlutinWindowed(win) = loop_data {
            let (mut event_loop, win, bgcolor) = win.unpack();
            event_loop.run_return(move |event, _, control_flow| {
                // println!("Event received {:?}", event);
                *control_flow = ControlFlow::Wait;
                match event {
                    glutin::event::Event::WindowEvent { event, .. } => {
                        match event {
                            glutin::event::WindowEvent::CloseRequested => {
                                *control_flow = ControlFlow::Exit;
                            }
                            glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                                match input.virtual_keycode {
                                    None => (),
                                    Some(glutin::event::VirtualKeyCode::Escape) => {
                                        *control_flow = ControlFlow::Exit;
                                    }
                                    _ => {}
                                }
                            }
                            glutin::event::WindowEvent::Resized(size) => unsafe {
                                gl::Viewport(0, 0, size.width as i32, size.height as i32);
                            },
                            _ => (),
                        }
                        self.send_event_to_ui_aware(event);
                        self.deliver_all_messages();
                        win.window().request_redraw();
                    }
                    glutin::event::Event::RedrawRequested(..) => {
                        unsafe {
                            gl::ClearColor(bgcolor.x, bgcolor.y, bgcolor.z, 1.0);
                            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                        }
                        self.send_to_ui_aware(ClonableUiEvent::Redraw);
                        self.deliver_all_messages(); // FIXME: don't deliver all
                        win.swap_buffers().unwrap();
                    }
                    glutin::event::Event::UserEvent(msg) => {
                        if msg.target == MessageTarget::None {
                            if msg.payload.is::<message::Exit>() {
                                *control_flow = ControlFlow::Exit;
                            }
                        } else {
                            self.static_world.send_annotated(msg);
                        }
                    }
                    glutin::event::Event::MainEventsCleared => {
                        self.send_to_ui_aware(ClonableUiEvent::EventsCleared);
                        self.deliver_all_messages();
                    }
                    // glutin::event::Event::RedrawEventsCleared => {
                    //     win.window().request_redraw();
                    // }
                    _ => (),
                }
            });
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum ClonableUiEvent {
    Redraw,
    EventsCleared,
}

impl From<ClonableUiEvent> for UiEvent {
    fn from(cuiev: ClonableUiEvent) -> UiEvent {
        match cuiev {
            ClonableUiEvent::Redraw => UiEvent::Redraw,
            ClonableUiEvent::EventsCleared => UiEvent::EventsCleared,
        }
    }
}

#[derive(Debug)]
pub enum UiEvent {
    GlutinEvent(GlutinEvent<'static>),
    Redraw,
    EventsCleared,
}
impl Message for UiEvent {}
