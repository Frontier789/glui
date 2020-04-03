use gl::types::*;
use std::cell::Cell;
use std::time::Duration;

#[derive(Copy,Clone,Eq,PartialEq)]
enum State {
    Running,
    Stopped,
    HasTime(Duration),
}

pub struct GPUClock {
    id: u32,
    state: Cell<State>,
}

impl GPUClock {
    pub fn new() -> GPUClock {
        let mut id: GLuint = 0; 
        unsafe {
            gl::GenQueries(1, &mut id);
        }
        
        GPUClock {
            id,
            state: Cell::new(State::Stopped),
        }
    }
    pub fn new_start() -> GPUClock {
        let mut clk = Self::new();
        clk.start();
        clk
    }
    pub fn start(&mut self) {
        unsafe {
            gl::BeginQuery(gl::TIME_ELAPSED, self.id);
        }
        self.state = Cell::new(State::Running);
    }
    fn impl_stop(&self) {
        if self.state.get() == State::Running {
            unsafe {
                gl::EndQuery(gl::TIME_ELAPSED);
            }
        }
        self.state.replace(State::Stopped);
    }
    pub fn stop(&mut self) {
        self.impl_stop();
    }
    pub fn ready(&self) -> bool {
        if let State::HasTime(_) = self.state.get() {
            return true;
        }
        
        let mut available: GLint = 0;
        unsafe {
            gl::GetQueryObjectiv(self.id, gl::QUERY_RESULT_AVAILABLE, &mut available);
        }
        available > 0
    }
    pub fn time(&self) -> Duration {
        if let State::HasTime(t) = self.state.get() {
            return t;
        }
        
        if let State::Running = self.state.get() {
            self.impl_stop();
        }
        
        let mut result: GLuint64 = 0;
        unsafe {
            gl::GetQueryObjectui64v(self.id, gl::QUERY_RESULT, &mut result);
        }
        
        let t = Duration::from_nanos(result as u64);
        
        self.state.replace(State::HasTime(t));
        
        t
    }
}
