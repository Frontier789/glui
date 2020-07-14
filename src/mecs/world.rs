extern crate gl;
extern crate glutin;

use std::mem::swap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::*;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use tools::*;

use super::glutin_cont::*;
use super::glutin_util::*;
use super::glutin_win::*;
use super::message;
use super::message::*;
use super::message_channel::*;
use super::message_loop_data::*;
use super::render_target::*;
use super::static_world::*;
use super::system::*;

use self::glutin::event::*;
use self::glutin::platform::desktop::EventLoopExtDesktop;
use gui::{GuiBuilder, GuiContext};
use mecs::render_pipeline::DefaultPipeline;
use mecs::SystemSet;
use mecs::{Component, Entity, RenderPipeline};

pub struct World {
    static_world: StaticWorld,
    systems: SystemSet,
    loop_data: MessageLoopData,
    render_pipeline: Box<dyn RenderPipeline>,
    running: bool,
}

impl Default for World {
    fn default() -> Self {
        World {
            static_world: Default::default(),
            systems: Default::default(),
            loop_data: Default::default(),
            render_pipeline: Box::new(DefaultPipeline {
                bgcolor: Vec3::new(0.3, 0.3, 0.3),
            }),
            running: Default::default(),
        }
    }
}

impl World {
    pub fn default_update_interval() -> Duration {
        Duration::from_millis(20)
    }
    pub fn entity(&mut self) -> Entity {
        self.as_static_mut().entity()
    }
    pub fn delete_entity(&mut self, entity: Entity) {
        self.as_static_mut().delete_entity(entity);
    }

    pub fn component<C>(&self, entity: Entity) -> Option<&C>
    where
        C: Component,
    {
        self.as_static().component::<C>(entity)
    }

    pub fn component_mut<C>(&mut self, entity: Entity) -> Option<&mut C>
    where
        C: Component,
    {
        self.as_static_mut().component_mut::<C>(entity)
    }
    pub fn add_component<C>(&mut self, entity: Entity, component: C)
    where
        C: Component,
    {
        self.as_static_mut().add_component(entity, component);
    }
    pub fn entities_with_component<C>(&self) -> Vec<(Entity, &C)>
    where
        C: Component,
    {
        self.as_static().entities_with_component::<C>()
    }
    pub fn entities_with_component_mut<C>(&mut self) -> Vec<(Entity, &mut C)>
    where
        C: Component,
    {
        self.as_static_mut().entities_with_component_mut::<C>()
    }

    pub fn new() -> World {
        let (sender, receiver) = channel();
        World {
            loop_data: MessageLoopData::HandRolled(HandrolledMsgLoopData {
                sender,
                receiver,
                update_interval: Self::default_update_interval(),
            }),
            ..Default::default()
        }
    }
    pub fn new_win(size: Vec2, title: &str, bgcolor: Vec3) -> World {
        World {
            loop_data: MessageLoopData::GlutinWindowed(GlutinWindowData::new(
                size,
                title,
                bgcolor,
                Self::default_update_interval(),
            )),
            render_pipeline: Box::new(DefaultPipeline { bgcolor }),
            ..Default::default()
        }
    }
    pub fn new_winless(bgcolor: Vec3) -> Result<World, glutin::CreationError> {
        Ok(World {
            loop_data: MessageLoopData::GlutinWindowless(GlutinContextData::new(
                bgcolor,
                Self::default_update_interval(),
            )?),
            ..Default::default()
        })
    }
    pub fn window_info(&self) -> Option<WindowInfo> {
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

    pub fn system<S>(&self, id: SystemId) -> Option<&S>
    where
        S: System + 'static,
    {
        self.systems.system::<S>(id)
    }
    pub fn system_mut<S>(&mut self, id: SystemId) -> Option<&mut S>
    where
        S: System + 'static,
    {
        self.systems.system_mut::<S>(id)
    }
    pub fn system_boxed(&self, id: SystemId) -> Option<&Box<dyn System>> {
        self.systems.system_boxed(id)
    }
    pub fn system_boxed_mut(&mut self, id: SystemId) -> Option<&mut Box<dyn System>> {
        self.systems.system_boxed_mut(id)
    }

    pub fn add_system<S>(&mut self, system: S) -> SystemId
    where
        S: System + 'static,
    {
        self.systems.add_system(system)
    }
    pub fn del_system(&mut self, id: SystemId) {
        self.systems.del_system(id);
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
            // println!("delivering {:?}", msg);
            match msg.target {
                MessageTarget::Broadcast => {
                    for (_, system) in self.systems.all_systems_mut() {
                        system.receive(&msg.payload, &mut self.static_world);
                    }
                }
                MessageTarget::System(id) => {
                    self.systems
                        .system_boxed_mut(id)
                        .expect(&format!(
                            "Message {:?} sent to non-existing system!",
                            msg.payload
                        ))
                        .receive(&msg.payload, &mut self.static_world);
                }
                MessageTarget::SystemOfType(sys_type) => {
                    self.systems
                        .first_of_type_mut(sys_type)
                        .expect(&format!(
                            "Message {:?} sent to non-existing system!",
                            msg.payload
                        ))
                        .receive(&msg.payload, &mut self.static_world);
                }
                MessageTarget::Root => {
                    if msg.payload.is::<message::Exit>() {
                        self.running = false;
                    } else {
                        if let Some(Update(dt)) = msg.payload.downcast_ref() {
                            self.update_systems(*dt);
                        } else {
                            eprintln!("Root got message {:?}", msg.payload);
                        }
                    }
                }
            }
        }
    }
    fn update_systems(&mut self, delta_time: Duration) {
        for (_id, system) in self.systems.all_systems_mut() {
            system.update(delta_time, &mut self.static_world);
        }
    }
    pub fn channel(&mut self) -> MessageChannel {
        match &self.loop_data {
            MessageLoopData::HandRolled(HandrolledMsgLoopData { sender, .. }) => {
                MessageChannel::from_sender(sender.clone())
            }
            MessageLoopData::GlutinWindowed(win) => {
                MessageChannel::from_glutin(win.event_loop_proxy())
            }
            MessageLoopData::GlutinWindowless(cont) => {
                MessageChannel::from_glutin(cont.event_loop_proxy())
            }
            MessageLoopData::Consumed => {
                panic!("Cannot give channel to consumed message loop data!")
            }
        }
    }
    pub fn run(mut self) {
        if self.running {
            return;
        }
        self.running = true;

        match self.loop_data {
            MessageLoopData::HandRolled(_) => {
                self.owned_msg_loop();
            }
            MessageLoopData::GlutinWindowed(_) => {
                self.glutin_win_msg_loop();
            }
            MessageLoopData::GlutinWindowless(_) => {
                self.glutin_cont_msg_loop();
            }
            MessageLoopData::Consumed => {
                self.running = false;
            }
        }
    }
    fn msg_loop_updater(update_interval: Duration, sender: MessageChannel) -> Arc<AtomicBool> {
        let updates_running = Arc::new(AtomicBool::new(true));
        let updates_running_clone = updates_running.clone();

        thread::spawn(move || {
            let mut sleep_until = Instant::now() + update_interval;

            while updates_running.load(Ordering::Relaxed) {
                if let Err(_) = sender.send_to_root(Update(update_interval)) {
                    break;
                };
                thread::sleep(sleep_until - Instant::now());
                sleep_until += update_interval;
            }
        });

        updates_running_clone
    }
    fn owned_msg_loop(mut self) {
        let mut loop_data = MessageLoopData::Consumed;
        std::mem::swap(&mut loop_data, &mut self.loop_data);
        if let MessageLoopData::HandRolled(HandrolledMsgLoopData {
            sender,
            receiver,
            update_interval,
        }) = loop_data
        {
            let updates_running =
                Self::msg_loop_updater(update_interval, MessageChannel::from_sender(sender));

            while self.running {
                let msg = receiver.recv().unwrap();
                self.static_world.send_annotated(msg);
                self.deliver_all_messages();
            }
            updates_running.store(false, Ordering::Relaxed);
        }
    }
    fn glutin_update_control(
        &mut self,
        event: &Event<AnnotatedMessage>,
        control_flow: &mut GlutinControlFlow,
        update_interval: Duration,
    ) -> bool {
        match event {
            Event::NewEvents(StartCause::Init) => {
                *control_flow = GlutinControlFlow::WaitUntil(Instant::now() + update_interval);
            }
            Event::NewEvents(StartCause::ResumeTimeReached {
                requested_resume, ..
            }) => {
                *control_flow = GlutinControlFlow::WaitUntil(*requested_resume + update_interval);
                self.update_systems(update_interval);
                return true;
            }
            _ => {}
        }
        false
    }
    fn glutin_cont_msg_loop(mut self) {
        let mut loop_data = MessageLoopData::Consumed;
        std::mem::swap(&mut loop_data, &mut self.loop_data);

        if let MessageLoopData::GlutinWindowless(data) = loop_data {
            let GlutinContextData {
                mut event_loop,
                update_interval,
                ..
            } = data;

            event_loop.run_return(move |event, _, mut control_flow| {
                self.glutin_update_control(&event, &mut control_flow, update_interval);
                match event {
                    Event::UserEvent(msg) => {
                        self.static_world.send_annotated(msg);
                    }
                    Event::MainEventsCleared => {
                        self.deliver_all_messages();
                    }
                    _ => (),
                }
                if !self.running {
                    *control_flow = GlutinControlFlow::Exit;
                }
            });
        }
    }

    fn glutin_win_msg_loop(mut self) {
        let mut loop_data = MessageLoopData::Consumed;
        std::mem::swap(&mut loop_data, &mut self.loop_data);
        if let MessageLoopData::GlutinWindowed(data) = loop_data {
            let GlutinWindowData {
                mut event_loop,
                gl_window: win,
                bgcolor: _,
                update_interval,
            } = data;

            event_loop.run_return(move |event, _, mut control_flow| {
                self.glutin_update_control(&event, &mut control_flow, update_interval);

                match event {
                    Event::WindowEvent { event, .. } => {
                        if self.glutin_window_event(&event) {
                            self.render_pipeline.event(
                                &mut self.static_world,
                                &mut self.systems,
                                &GlutinEvent::WindowEvent(event),
                            );
                        }

                        self.deliver_all_messages();
                    }
                    Event::DeviceEvent { event, .. } => {
                        self.render_pipeline.event(
                            &mut self.static_world,
                            &mut self.systems,
                            &GlutinEvent::DeviceEvent(event),
                        );

                        self.deliver_all_messages();
                    }
                    Event::RedrawRequested(..) => {
                        self.render_pipeline
                            .render(&mut self.static_world, &mut self.systems);
                        win.swap_buffers().unwrap();
                    }
                    Event::UserEvent(msg) => {
                        self.static_world.send_annotated(msg);
                    }
                    Event::MainEventsCleared => {
                        self.deliver_all_messages();
                        win.window().request_redraw();
                    }
                    _ => (),
                }
                if !self.running {
                    *control_flow = GlutinControlFlow::Exit;
                }
            });
        }
    }

    fn glutin_window_event(&mut self, event: &glutin::event::WindowEvent) -> bool {
        match event {
            glutin::event::WindowEvent::CloseRequested => {
                self.running = false;
            }
            glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                if let Some(key) = input.virtual_keycode {
                    if key == glutin::event::VirtualKeyCode::Escape {
                        self.running = false;
                    }

                    if input.state == GlutinElementState::Pressed
                        && self.static_world.is_key_pressed(key)
                    {
                        return false;
                    }

                    self.static_world.key_states.insert(key, input.state);
                }
            }
            glutin::event::WindowEvent::Resized(size) => unsafe {
                gl::Viewport(0, 0, size.width as i32, size.height as i32);
            },
            _ => (),
        }
        true
    }

    pub fn add_gui<T>(&mut self, gui_builder: T)
    where
        T: GuiBuilder + 'static,
    {
        let gui_context = GuiContext::new(
            self.window_info().unwrap(),
            false,
            gui_builder,
            &mut self.static_world,
        );

        self.add_system(gui_context);
    }
}

#[derive(Debug)]
struct Update(pub Duration);
impl Message for Update {}
