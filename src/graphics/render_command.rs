extern crate gl;

use graphics::{DrawResources, DrawShaderSelector};
use tools::{DrawMode, Uniform, Vec4, VertexArray};

#[derive(Debug)]
pub struct RenderCommand {
    pub vao: VertexArray,
    pub mode: DrawMode,
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
        let shader = draw_resources.get_shader(&self.shader);
        shader.bind();
        shader.set_uniform("projection", draw_resources.projection_matrix);
        shader.set_uniform("inv_projection", draw_resources.inv_projection_matrix);
        shader.set_uniform("view", draw_resources.view_matrix);
        shader.set_uniform("inv_view", draw_resources.inv_view_matrix);
        shader.set_uniform(
            "cam_pos",
            (draw_resources.inv_view_matrix * Vec4::new(0.0, 0.0, 0.0, 1.0)).xyz(),
        );
        shader.set_uniform("model", draw_resources.model_matrix);
        shader.set_uniform("MVP", draw_resources.MVP());
        shader.set_uniform("uv_matrix", draw_resources.uv_matrix);
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

        self.vao.render(self.instances, self.mode);
    }
}
