extern crate gl;

use self::gl::types::*;
use graphics::{DrawResources, DrawShaderSelector, Indices};
use tools::{DrawMode, Uniform, VertexArray};

#[derive(Debug)]
pub struct RenderCommand {
    pub vao: VertexArray,
    pub mode: DrawMode,
    pub indices: Indices,
    pub shader: DrawShaderSelector,
    pub uniforms: Vec<Uniform>,
    pub transparent: bool,
    pub instances: usize,
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

    fn bind_shader(&self, draw_resources: &DrawResources) {
        let projection_matrix = draw_resources.projection_matrix;
        let view_matrix = draw_resources.view_matrix;
        let model_matrix = draw_resources.model_matrix;
        let uv_matrix = draw_resources.uv_matrix;
        let shader = draw_resources.get_shader(&self.shader);
        shader.bind();
        shader.set_uniform("projection", projection_matrix);
        shader.set_uniform("view", view_matrix);
        shader.set_uniform("model", model_matrix);
        shader.set_uniform("MVP", projection_matrix * view_matrix * model_matrix);
        shader.set_uniform("uv_matrix", uv_matrix);
    }

    pub fn execute_prev(&self, previous: Option<&RenderCommand>, draw_resources: &DrawResources) {
        if let Some(previous) = previous {
            if self.transparent != previous.transparent {
                self.apply_blend();
            }
            if self.shader != previous.shader {
                self.bind_shader(draw_resources);
            }
        } else {
            self.apply_blend();
            self.bind_shader(draw_resources);
        }

        let shader = draw_resources.get_shader(&self.shader);
        for uniform in &self.uniforms {
            shader.set_uniform_val(uniform.clone());
        }

        self.do_the_draw();
    }

    fn do_the_draw(&self) {
        self.vao.bind();

        match (self.instances, &self.indices) {
            (0, _) => {}
            (_, Indices::None) => {}
            (1, Indices::Range(range)) => {
                let vertex_count = range.end - range.start;

                unsafe {
                    gl::DrawArrays(
                        self.mode.into(),
                        range.start as GLint,
                        vertex_count as GLsizei,
                    );
                }
            }
            (1, Indices::Vec(indices)) => unsafe {
                gl::DrawElements(
                    self.mode.into(),
                    indices.len() as GLsizei,
                    gl::UNSIGNED_INT,
                    indices.as_ptr() as *const _,
                );
            },
            (instances, Indices::Range(range)) => {
                let vertex_count = range.end - range.start;

                unsafe {
                    gl::DrawArraysInstanced(
                        self.mode.into(),
                        range.start as GLint,
                        vertex_count as GLsizei,
                        instances as GLsizei,
                    );
                }
            }
            (instances, Indices::Vec(indices)) => unsafe {
                gl::DrawElementsInstanced(
                    self.mode.into(),
                    indices.len() as GLsizei,
                    gl::UNSIGNED_INT,
                    indices.as_ptr() as *const _,
                    instances as GLsizei,
                );
            },
        }
    }
}
