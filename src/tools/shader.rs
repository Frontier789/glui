use super::gl::types::*;
use super::gltraits::GlUniform;
use super::shader_error::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use tools::Uniform;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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

fn compile_subshader(source: &str, t: ShaderType) -> Result<GLuint, ShaderCompileError> {
    let sid = unsafe { gl::CreateShader(t.into()) };
    if sid == 0 {
        return Err(ShaderCompileError::new(
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

            Err(ShaderCompileError::new(format!(
                "Failed to compile {} err: {}",
                t, text
            )))
        } else {
            Ok(sid)
        }
    }
}

fn compile(sources: Vec<(&str, ShaderType)>) -> Result<(GLuint, Vec<GLuint>), ShaderCompileError> {
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

        return Err(ShaderCompileError::new(format!(
            "Failed to link shader err: {}",
            text
        )));
    }

    Ok((id, sids))
}

#[derive(Debug)]
struct ShaderData {
    id: u32,
    sids: Vec<u32>,
    tex_uniforms: RefCell<HashMap<GLint, u32>>,
}

#[derive(Debug, Clone)]
pub struct DrawShader {
    data: Rc<ShaderData>,
}

impl DrawShader {
    pub fn compile(vert_source: &str, frag_source: &str) -> Result<DrawShader, ShaderCompileError> {
        let (id, sids) = compile(vec![
            (vert_source, ShaderType::Vertex),
            (frag_source, ShaderType::Fragment),
        ])?;
        Ok(DrawShader {
            data: Rc::new(ShaderData {
                id,
                sids,
                tex_uniforms: RefCell::new(HashMap::new()),
            }),
        })
    }

    pub fn id(&self) -> GLuint {
        self.data.id
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id());
        }
    }

    pub fn set_uniform<T>(&self, name: &str, val: T)
    where
        T: GlUniform,
    {
        let name_ptr = std::ffi::CString::new(name).unwrap().into_raw();
        // TODO: cache GetUniformLocation
        let loc: GLint = unsafe { gl::GetUniformLocation(self.id(), name_ptr as *const i8) };

        if loc > -1 {
            T::set_uniform_impl(val, self.id(), loc, self.data.tex_uniforms.borrow_mut());
        }
    }

    pub fn set_uniform_val(&self, uniform: Uniform) {
        match uniform {
            Uniform::Float(id, value) => {
                self.set_uniform(&id, value);
            }
            Uniform::Vector2(id, value) => {
                self.set_uniform(&id, value);
            }
            Uniform::Vector3(id, value) => {
                self.set_uniform(&id, value);
            }
            Uniform::Vector4(id, value) => {
                self.set_uniform(&id, value);
            }
            Uniform::Matrix4(id, value) => {
                self.set_uniform(&id, value);
            }
            Uniform::Texture2D(id, value) => {
                self.uniform_tex2d_id(&id, value);
            }
            Uniform::TextureCube(id, value) => {
                self.uniform_tex_cube_id(&id, value);
            }
        }
    }

    pub fn uniform_tex2d_id(&self, name: &str, id: u32) {
        self.uniform_texture_id(name, gl::TEXTURE_2D, id);
    }

    pub fn uniform_tex_cube_id(&self, name: &str, id: u32) {
        self.uniform_texture_id(name, gl::TEXTURE_CUBE_MAP, id);
    }

    fn uniform_texture_id(&self, name: &str, target: GLenum, id: u32) {
        let name_ptr = std::ffi::CString::new(name).unwrap().into_raw();
        let loc: GLint = unsafe { gl::GetUniformLocation(self.id(), name_ptr as *const i8) };

        let mut tex_uniforms = self.data.tex_uniforms.borrow_mut();

        if !tex_uniforms.contains_key(&loc) {
            let slot = tex_uniforms.len() as u32;

            unsafe {
                gl::ProgramUniform1i(self.id(), loc, slot as GLint);
            }
            tex_uniforms.insert(loc, slot);
        }

        let slot = tex_uniforms[&loc];

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + slot);
            gl::BindTexture(target, id);
        }
    }

    pub fn from_files(vert: &str, frag: &str) -> Result<DrawShader, ShaderLoadError> {
        let vert_source = std::fs::read_to_string(vert)?;
        let frag_source = std::fs::read_to_string(frag)?;

        Ok(Self::compile(&vert_source, &frag_source)?)
    }
}

impl Drop for DrawShader {
    fn drop(&mut self) {
        if Rc::strong_count(&self.data) == 1 {
            unsafe {
                gl::DeleteProgram(self.id());
            }
        }
    }
}

impl PartialEq for DrawShader {
    fn eq(&self, s: &Self) -> bool {
        self.id() == s.id()
    }
}
