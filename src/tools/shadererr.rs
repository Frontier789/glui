use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ShaderCompileErr {
    details: String
}

impl ShaderCompileErr {
    pub fn new(compile_msg: String) -> ShaderCompileErr {
        ShaderCompileErr{details: compile_msg}
    }
    
    pub fn details(&self) -> &String {
        &self.details
    }
}

impl fmt::Display for ShaderCompileErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for ShaderCompileErr {
    fn description(&self) -> &str {
        &self.details
    }
}