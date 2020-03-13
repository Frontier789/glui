pub use super::tools::*;
pub use super::gui::RenderTarget;
pub use super::gui::GlutinEvent;

impl From<glutin::dpi::PhysicalSize<u32>> for Vec2 {
    fn from(s: glutin::dpi::PhysicalSize<u32>) -> Vec2 {
        Vec2::new(s.width as f32, s.height as f32)
    }
}

impl From<glutin::dpi::PhysicalPosition<f64>> for Vec2 {
    fn from(p: glutin::dpi::PhysicalPosition<f64>) -> Vec2 {
        Vec2::new(p.x as f32, p.y as f32)
    }
}

impl From<glutin::dpi::PhysicalPosition<i32>> for Vec2 {
    fn from(p: glutin::dpi::PhysicalPosition<i32>) -> Vec2 {
        Vec2::new(p.x as f32, p.y as f32)
    }
}

// TODO: move methods to componenets
pub trait Entity {
    fn handle_event(&mut self, e: &GlutinEvent);
    
    fn render(&mut self);
}

pub struct World {
    pub entities: Vec<Box<dyn Entity>>,
}

impl World {
    pub fn handle_event(&mut self, ev: GlutinEvent) {
        for e in self.entities.iter_mut() {
            e.handle_event(&ev);
        }
    }
    
    pub fn render(&mut self) {
        for e in self.entities.iter_mut() {
            e.render();
        }
    }
    
    pub fn add_entity(&mut self, entity: Box<dyn Entity>) {
        self.entities.push(entity);
    }
}
