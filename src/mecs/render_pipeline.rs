use mecs::{DrawSystem, GlutinEvent, StaticWorld, SystemSet};
use tools::Vec3;

pub trait RenderPipeline {
    fn render(&mut self, world: &mut StaticWorld, all_systems: &mut SystemSet);
    fn event(&mut self, world: &mut StaticWorld, all_systems: &mut SystemSet, event: &GlutinEvent);
}

pub struct DefaultPipeline {
    pub bgcolor: Vec3,
}

impl RenderPipeline for DefaultPipeline {
    fn render(&mut self, world: &mut StaticWorld, all_systems: &mut SystemSet) {
        unsafe {
            gl::ClearColor(self.bgcolor.x, self.bgcolor.y, self.bgcolor.z, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // Draw draw_systems first
        for (id, sys) in all_systems.all_systems_mut() {
            if id.is_type::<DrawSystem>() {
                sys.render(world);
            }
        }

        // Then the rest
        for (id, sys) in all_systems.all_systems_mut() {
            if !id.is_type::<DrawSystem>() {
                sys.render(world);
            }
        }
    }

    fn event(&mut self, world: &mut StaticWorld, all_systems: &mut SystemSet, event: &GlutinEvent) {
        // Everything first
        for (id, sys) in all_systems.all_systems_mut() {
            if !id.is_type::<DrawSystem>() {
                if sys.event(event, world) {
                    return;
                }
            }
        }

        // Send to draw_systems last
        for (id, sys) in all_systems.all_systems_mut() {
            if id.is_type::<DrawSystem>() {
                if sys.event(event, world) {
                    return;
                }
            }
        }
    }
}
