use std::error::Error;
use std::{fmt, io};

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub enum ShaderLoadError {
    IoError(io::Error),
    CompileError(ShaderCompileError),
}

impl From<io::Error> for ShaderLoadError {
    fn from(e: io::Error) -> Self {
        ShaderLoadError::IoError(e)
    }
}

impl From<ShaderCompileError> for ShaderLoadError {
    fn from(e: ShaderCompileError) -> Self {
        ShaderLoadError::CompileError(e)
    }
}
