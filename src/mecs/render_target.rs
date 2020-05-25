use super::gl::types::*;
use std::fmt;
use tools::*;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GLVerion {
    major: usize,
    minor: usize,
}

impl fmt::Display for GLVerion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct WindowInfo {
    pub size: Vec2,
    pub gui_scale: f32,
    pub gl_verison: GLVerion,
}

impl WindowInfo {
    pub fn logical_size(&self) -> Vec2px {
        Vec2px::from_pixels(self.size, self.gui_scale)
    }
    pub fn fill_from_context(mut self) -> Self {
        let mut major: GLint = 0;
        let mut minor: GLint = 0;
        unsafe {
            gl::GetIntegerv(gl::MAJOR_VERSION, &mut major);
            gl::GetIntegerv(gl::MINOR_VERSION, &mut minor);
        }

        self.gl_verison = GLVerion {
            major: major as usize,
            minor: minor as usize,
        };
        self
    }
}
