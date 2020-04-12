use super::*;
use std::collections::HashMap;
use std::mem::swap;
use std::sync::mpsc::*;

pub struct World {
    static_world: StaticWorld,
    systems: HashMap<SystemId, Box<dyn System>>,
    actors: HashMap<ActorId, Box<dyn Actor>>,
    next_actor_id: usize,
    loop_data: MessageLoopData,
}

impl World {
    pub fn new() -> World {
        let (sender, receiver) = channel();
        World {
            static_world: Default::default(),
            systems: Default::default(),
            actors: Default::default(),
            next_actor_id: 0,
            loop_data: MessageLoopData::HandRolled(sender, receiver),
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

    // pub fn system<S>(&mut self) -> Option<&mut S>
    // where
    //     S: System + 'static,
    // {
    //     self.systems.get_mut(&SystemId::of::<S>())
    // }
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
                        .expect(&format!("Message {:?} sent to non-existing system!", msg.payload))
                        .receive(msg.payload, &mut self.static_world);
                }
                MessageTarget::Actor(id) => {
                    self.actors
                        .get_mut(&id)
                        .expect(&format!("Message {:?} sent to non-existing actor!", msg.payload))
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
            MessageLoopData::Glutin(win) => MessageChannel::from_window(&win),
            MessageLoopData::Consumed => panic!("Cannot give channel to consumed message loop data!"),
        }
    }
    pub fn run(self) {
        match self.loop_data {
            MessageLoopData::HandRolled(_, _) => {
                self.run_msg_loop();
            }
            MessageLoopData::Glutin(win) => {
                win.run(GuiWinProps::tester());
            }
            MessageLoopData::Consumed => {}
        }
    }
    fn run_msg_loop(mut self) {
        
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
}
