use graphics::{DrawResources, RenderCommand};
use tools::Buffer;

#[derive(Debug)]
pub struct RenderSequence {
    buffers: Vec<Buffer<f32>>,
    commands: Vec<RenderCommand>,
}

impl RenderSequence {
    pub fn new() -> RenderSequence {
        RenderSequence {
            buffers: vec![],
            commands: vec![],
        }
    }
    pub fn add_buffer(&mut self, buf: Buffer<f32>) {
        self.buffers.push(buf);
    }
    pub fn add_command(&mut self, cmd: RenderCommand) {
        self.commands.push(cmd);
    }
    pub fn execute(&self, resources: &mut DrawResources) {
        for cmd in &self.commands {
            cmd.vao.bind();
            let shader = resources.shader(&cmd.shader).unwrap();
            shader.bind();
            for uniform in &cmd.uniforms {
                shader.set_uniform_val(uniform.clone());
            }
            cmd.execute();
        }
    }
}
