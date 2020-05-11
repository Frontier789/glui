use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ShaderCompileError {
    details: String,
}

impl ShaderCompileError {
    pub fn new(compile_msg: String) -> ShaderCompileError {
        ShaderCompileError {
            details: compile_msg,
        }
    }

    pub fn details(&self) -> &str {
        &self.details
    }
}

impl fmt::Display for ShaderCompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ShaderCompileError {
    fn description(&self) -> &str {
        &self.details
    }
}
