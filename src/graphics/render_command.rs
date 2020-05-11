extern crate gl;

use self::gl::types::*;
use tools::{DrawMode, Uniform, VertexArray};

#[derive(Debug)]
pub struct RenderCommand {
    pub vao: VertexArray,
    pub mode: DrawMode,
    pub first: usize,
    pub count: usize,
    pub shader: String,
    pub uniforms: Vec<Uniform>,
    pub transparent: bool,
}

impl RenderCommand {
    fn apply_blend(&self) {
        unsafe {
            if self.transparent {
                gl::Enable(gl::BLEND);
                gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            } else {
                gl::Disable(gl::BLEND);
            }
        }
    }

    pub fn execute(&self) {
        self.apply_blend();

        unsafe {
            gl::DrawArrays(self.mode.into(), self.first as GLint, self.count as GLsizei);
        }
    }

    pub fn execute_prev(&self, previous: &RenderCommand) {
        if self.transparent != previous.transparent {
            self.apply_blend();
        }
        unsafe {
            gl::DrawArrays(self.mode.into(), self.first as GLint, self.count as GLsizei);
        }
    }
}
