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
    pub wireframe: bool,
}

impl RenderCommand {
    pub fn new(vao: VertexArray, mode: DrawMode, shader: DrawShaderSelector) -> RenderCommand {
        RenderCommand {
            vao,
            mode,
            shader,
            uniforms: vec![],
            transparent: false,
            instances: 1,
            wireframe: false,
        }
    }
    pub fn new_uniforms(
        vao: VertexArray,
        mode: DrawMode,
        shader: DrawShaderSelector,
        uniforms: Vec<Uniform>,
    ) -> RenderCommand {
        RenderCommand {
            vao,
            mode,
            shader,
            uniforms,
            transparent: false,
            instances: 1,
            wireframe: false,
        }
    }
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
    fn apply_wireframe(&self) {
        unsafe {
            if self.wireframe {
                gl::PolygonMode(gl::FRONT, gl::LINE);
            } else {
                gl::PolygonMode(gl::FRONT, gl::FILL);
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
        shader.set_uniform("MV", draw_resources.MV());
        shader.set_uniform("VP", draw_resources.VP());
        shader.set_uniform("normal_MV", draw_resources.normal_matrix_MV());
        shader.set_uniform("normal_model", draw_resources.normal_matrix_model());
        shader.set_uniform("time", draw_resources.clock.elapsed().as_secs_f32());
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
            if self.wireframe != previous.wireframe {
                self.apply_wireframe();
            }
        } else {
            self.apply_blend();
            self.bind_shader(draw_resources);
            self.apply_wireframe();
        }

        let shader = draw_resources.get_shader(&self.shader);
        for uniform in &self.uniforms {
            shader.set_uniform_val(uniform.clone());
        }

        self.vao.render(self.instances, self.mode);
    }
}
