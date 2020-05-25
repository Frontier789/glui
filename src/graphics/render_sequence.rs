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
    pub fn has_transparent(&self) -> bool {
        for cmd in &self.commands {
            if cmd.transparent {
                return true;
            }
        }
        false
    }
    pub fn add_buffer(&mut self, buf: Buffer<f32>) {
        self.buffers.push(buf);
    }
    pub fn add_command(&mut self, cmd: RenderCommand) {
        self.commands.push(cmd);
    }
    pub fn execute(&self, resources: &DrawResources) {
        if self.commands.is_empty() {
            return;
        }

        self.commands[0].execute_prev(None, resources);
        for i in 1..self.commands.len() {
            self.commands[i].execute_prev(Some(&self.commands[i - 1]), resources);
        }
    }
}
