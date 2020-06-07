use graphics::{DrawResources, RenderCommand};
use tools::Buffer;

#[derive(Debug)]
pub struct RenderSequence {
    buffers: Vec<Buffer<f32>>,
    index_buffers: Vec<Buffer<u32>>,
    commands: Vec<RenderCommand>,
}

impl RenderSequence {
    pub fn new() -> RenderSequence {
        RenderSequence {
            buffers: vec![],
            index_buffers: vec![],
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
    pub fn buffer(&self, index: usize) -> &Buffer<f32> {
        &self.buffers[index]
    }
    pub fn buffer_mut(&mut self, index: usize) -> &mut Buffer<f32> {
        &mut self.buffers[index]
    }
    pub fn add_buffer(&mut self, buf: Buffer<f32>) {
        self.buffers.push(buf);
    }
    pub fn add_index_buffer(&mut self, buf: Buffer<u32>) {
        self.index_buffers.push(buf);
    }
    pub fn command(&self, index: usize) -> &RenderCommand {
        &self.commands[index]
    }
    pub fn command_mut(&mut self, index: usize) -> &mut RenderCommand {
        &mut self.commands[index]
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
