use mecs::{DrawSystem, GlutinEvent, StaticWorld, SystemId, SystemSet};
use std::collections::HashSet;
use tools::Vec3;

pub trait RenderPipeline {
    fn render(
        &mut self,
        world: &mut StaticWorld,
        all_systems: &mut SystemSet,
        system_ids: &HashSet<SystemId>,
    );
    fn event(
        &mut self,
        world: &mut StaticWorld,
        all_systems: &mut SystemSet,
        system_ids: &HashSet<SystemId>,
        event: &GlutinEvent,
    );
}

pub struct DefaultPipeline {
    pub bgcolor: Vec3,
}

impl RenderPipeline for DefaultPipeline {
    fn render(
        &mut self,
        world: &mut StaticWorld,
        all_systems: &mut SystemSet,
        system_ids: &HashSet<SystemId>,
    ) {
        unsafe {
            gl::ClearColor(self.bgcolor.x, self.bgcolor.y, self.bgcolor.z, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // Draw draw_systems first
        for id in system_ids {
            if id.is_type::<DrawSystem>() {
                let system = all_systems.system_boxed_mut(*id).unwrap();

                system.render(world);
            }
        }

        // Then the rest
        for id in system_ids {
            if !id.is_type::<DrawSystem>() {
                let system = all_systems.system_boxed_mut(*id).unwrap();

                system.render(world);
            }
        }
    }

    fn event(
        &mut self,
        world: &mut StaticWorld,
        all_systems: &mut SystemSet,
        system_ids: &HashSet<SystemId>,
        event: &GlutinEvent,
    ) {
        // Everything first
        for id in system_ids {
            if !id.is_type::<DrawSystem>() {
                let system = all_systems.system_boxed_mut(*id).unwrap();

                if system.event(event, world) {
                    return;
                }
            }
        }

        // Send to draw_systems last
        for id in system_ids {
            if id.is_type::<DrawSystem>() {
                let system = all_systems.system_boxed_mut(*id).unwrap();

                if system.event(event, world) {
                    return;
                }
            }
        }
    }
}
