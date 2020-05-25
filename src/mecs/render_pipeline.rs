use mecs::world::UiEvent;
use mecs::{StaticWorld, SystemId};
use std::collections::HashSet;
use tools::Vec3;

pub trait RenderPipeline {
    fn render(&mut self, world: &mut StaticWorld, system_ids: &HashSet<SystemId>);
}

pub struct DefaultPipeline {
    pub bgcolor: Vec3,
}

impl RenderPipeline for DefaultPipeline {
    fn render(&mut self, world: &mut StaticWorld, system_ids: &HashSet<SystemId>) {
        unsafe {
            gl::ClearColor(self.bgcolor.x, self.bgcolor.y, self.bgcolor.z, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        for id in system_ids {
            world.send(*id, UiEvent::Redraw);
        }
    }
}
