use super::gltraits::GlUniform;
use super::shadererr::*;
use std::collections::HashMap;
use std::fmt;

#[derive(Copy, Clone)]
pub enum ShaderType {
    Compute,
    Vertex,
    TessControl,
    TessEvaluation,
    Geometry,
    Fragment,
}

impl fmt::Display for ShaderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}Shader",
            match self {
                ShaderType::Compute => "Compute",
                ShaderType::Vertex => "Vertex",
                ShaderType::TessControl => "TessControl",
                ShaderType::TessEvaluation => "TessEvaluation",
                ShaderType::Geometry => "Geometry",
                ShaderType::Fragment => "Fragment",
            }
        )
    }
}

impl From<ShaderType> for GLenum {
    fn from(t: ShaderType) -> Self {
        match t {
            ShaderType::Compute => gl::COMPUTE_SHADER,
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::TessControl => gl::TESS_CONTROL_SHADER,
            ShaderType::TessEvaluation => gl::TESS_EVALUATION_SHADER,
            ShaderType::Geometry => gl::GEOMETRY_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
        }
    }
}

fn shader_status(sid: u32) -> GLint {
    let mut success = 0 as GLint;

    unsafe {
        gl::GetShaderiv(sid, gl::COMPILE_STATUS, &mut success);
    }
    success
}

fn program_status(id: u32) -> GLint {
    let mut success = 0 as GLint;

    unsafe {
        gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
    }
    success
}

fn shader_info_log(sid: u32) -> String {
    let mut log_size = 0 as GLint;
    unsafe {
        gl::GetShaderiv(sid, gl::INFO_LOG_LENGTH, &mut log_size);
    }

    let mut text = String::new();
    let tv = unsafe { text.as_mut_vec() };
    tv.resize(log_size as usize, 0 as u8);

    unsafe {
        gl::GetShaderInfoLog(
            sid,
            log_size,
            std::ptr::null_mut(),
            tv.as_mut_ptr() as *mut i8,
        );
    }
    text
}

fn program_info_log(sid: u32) -> String {
    let mut log_size = 0 as GLint;
    unsafe {
        gl::GetProgramiv(sid, gl::INFO_LOG_LENGTH, &mut log_size);
    }

    let mut text = String::new();
    let tv = unsafe { text.as_mut_vec() };
    tv.resize(log_size as usize, 0 as u8);

    unsafe {
        gl::GetProgramInfoLog(
            sid,
            log_size,
            std::ptr::null_mut(),
            tv.as_mut_ptr() as *mut i8,
        );
    }
    text
}

fn compile_subshader(source: &str, t: ShaderType) -> Result<GLuint, ShaderCompileErr> {
    let sid = unsafe { gl::CreateShader(t.into()) };
    if sid == 0 {
        return Err(ShaderCompileErr::new(
            "Failed to allocate shader.".to_string(),
        ));
    }
    unsafe {
        let cptr = source.as_ptr() as *const GLchar;
        let ccnt = source.len() as GLint;
        gl::ShaderSource(sid, 1, &cptr, &ccnt);
        gl::CompileShader(sid);
        if shader_status(sid) == 0 {
            let text = shader_info_log(sid);

            Err(ShaderCompileErr::new(format!(
                "Failed to compile {} err: {}",
                t, text
            )))
        } else {
            Ok(sid)
        }
    }
}

fn compile(sources: Vec<(&str, ShaderType)>) -> Result<(GLuint, Vec<GLuint>), ShaderCompileErr> {
    let id: GLuint = unsafe { gl::CreateProgram() };
    let mut sids = Vec::with_capacity(sources.len());

    for (source, t) in sources {
        match compile_subshader(source, t) {
            Err(e) => return Err(e),
            Ok(sid) => {
                unsafe {
                    gl::AttachShader(id, sid);
                    gl::DeleteShader(sid);
                }
                sids.push(sid)
            }
        }
    }
    unsafe {
        gl::LinkProgram(id);
    }
    if program_status(id) == 0 {
        let text = program_info_log(id);

        return Err(ShaderCompileErr::new(format!(
            "Failed to link shader err: {}",
            text
        )));
    }

    Ok((id, sids))
}

use gl::types::*;

pub struct DrawShader {
    id: u32,
    sids: Vec<u32>,
    tex_uniforms: HashMap<GLint, u32>,
}

impl DrawShader {
    pub fn compile(vert_source: &str, frag_source: &str) -> Result<DrawShader, ShaderCompileErr> {
        let (id, sids) = compile(vec![
            (vert_source, ShaderType::Vertex),
            (frag_source, ShaderType::Fragment),
        ])?;
        Ok(DrawShader {
            id: id,
            sids: sids,
            tex_uniforms: HashMap::new(),
        })
    }
    
    pub fn id(&self) -> GLuint {
        self.id
    }
    
    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id());
        }
    }

    pub fn set_uniform<T>(&mut self, name: &str, val: T)
    where
        T: GlUniform,
    {
        let name_ptr = std::ffi::CString::new(name).unwrap().into_raw();
        let loc: GLint = unsafe { gl::GetUniformLocation(self.id(), name_ptr as *const i8) };
        
        if loc > -1 {
            T::set_uniform_impl(val, self.id, loc, &mut self.tex_uniforms);
        }
    }
    
    pub fn uniform_tex2d_id(&mut self, name: &str, id: u32) {
        let name_ptr = std::ffi::CString::new(name).unwrap().into_raw();
        let loc: GLint = unsafe { gl::GetUniformLocation(self.id(), name_ptr as *const i8) };
        
        if !self.tex_uniforms.contains_key(&loc) {
            let slot = self.tex_uniforms.len() as u32;

            unsafe {
                gl::ProgramUniform1i(self.id(), loc, slot as GLint);
            }
            self.tex_uniforms.insert(loc, slot);
        }
        
        let slot = self.tex_uniforms[&loc];
        
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + slot);
            gl::BindTexture(gl::TEXTURE_2D, id);
        }
    }
}

impl Drop for DrawShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl PartialEq for DrawShader {
    fn eq(&self, s: &Self) -> bool {
        self.id() == s.id()
    }
}
