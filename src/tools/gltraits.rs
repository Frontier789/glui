extern crate gl;

use super::matrix4::Mat4;
use super::tex2d::Texture;
use super::vector2::Vec2;
use super::vector3::Vec3;
use super::vector4::Vec4;
use std::collections::HashMap;

use super::gl::types::*;

/// Trait for types that can be stored in a buffer
pub trait GlNum: Clone + Copy {
    type BaseType;
    fn dim() -> u32;
    fn gl_type() -> u32;
    fn base_ptr(self) -> *const Self::BaseType;
}

impl GlNum for f32 {
    type BaseType = f32;
    fn dim() -> u32 {
        1
    }
    fn gl_type() -> u32 {
        gl::FLOAT as u32
    }
    fn base_ptr(self) -> *const f32 {
        &self
    }
}
impl GlNum for i32 {
    type BaseType = i32;
    fn dim() -> u32 {
        1
    }
    fn gl_type() -> u32 {
        gl::INT as u32
    }
    fn base_ptr(self) -> *const i32 {
        &self
    }
}
impl GlNum for Vec2 {
    type BaseType = f32;
    fn dim() -> u32 {
        2
    }
    fn gl_type() -> u32 {
        gl::FLOAT as u32
    }
    fn base_ptr(self) -> *const f32 {
        &self.x
    }
}
impl GlNum for Vec3 {
    type BaseType = f32;
    fn dim() -> u32 {
        3
    }
    fn gl_type() -> u32 {
        gl::FLOAT as u32
    }
    fn base_ptr(self) -> *const f32 {
        &self.x
    }
}
impl GlNum for Vec4 {
    type BaseType = f32;
    fn dim() -> u32 {
        4
    }
    fn gl_type() -> u32 {
        gl::FLOAT as u32
    }
    fn base_ptr(self) -> *const f32 {
        &self.x
    }
}
impl GlNum for Mat4 {
    type BaseType = f32;
    fn dim() -> u32 {
        16
    }
    fn gl_type() -> u32 {
        gl::FLOAT as u32
    }
    fn base_ptr(self) -> *const f32 {
        self.as_ptr()
    }
}

/// Types that can be set as uniforms
pub trait GlUniform {
    fn set_uniform_impl(val: Self, prog: GLuint, loc: GLint, map: &mut HashMap<GLint, u32>);
}

impl GlUniform for f32 {
    fn set_uniform_impl(
        val: Self,
        prog: GLuint,
        loc: GLint,
        _map: &mut HashMap<GLint, u32>,
    ) {
        unsafe {
            gl::ProgramUniform1f(prog, loc, val);
        }
    }
}

impl GlUniform for i32 {
    fn set_uniform_impl(
        val: Self,
        prog: GLuint,
        loc: GLint,
        _map: &mut HashMap<GLint, u32>,
    ) {
        unsafe {
            gl::ProgramUniform1i(prog, loc, val);
        }
    }
}

impl GlUniform for Vec2 {
    fn set_uniform_impl(
        val: Self,
        prog: GLuint,
        loc: GLint,
        _map: &mut HashMap<GLint, u32>,
    ) {
        unsafe {
            gl::ProgramUniform2f(prog, loc, val.x, val.y);
        }
    }
}

impl GlUniform for Vec3 {
    fn set_uniform_impl(
        val: Self,
        prog: GLuint,
        loc: GLint,
        _map: &mut HashMap<GLint, u32>,
    ) {
        unsafe {
            gl::ProgramUniform3f(prog, loc, val.x, val.y, val.z);
        }
    }
}

impl GlUniform for Vec4 {
    fn set_uniform_impl(
        val: Self,
        prog: GLuint,
        loc: GLint,
        _map: &mut HashMap<GLint, u32>,
    ) {
        unsafe {
            gl::ProgramUniform4f(prog, loc, val.x, val.y, val.z, val.w);
        }
    }
}

impl GlUniform for Mat4 {
    fn set_uniform_impl(
        val: Self,
        prog: GLuint,
        loc: GLint,
        _map: &mut HashMap<GLint, u32>,
    ) {
        unsafe {
            gl::ProgramUniformMatrix4fv(prog, loc, 1, gl::FALSE, val.as_ptr());
        }
    }
}

impl<T> GlUniform for &T
where
    T: Texture,
{
    fn set_uniform_impl(val: Self, prog: GLuint, loc: GLint, map: &mut HashMap<GLint, u32>) {
        if !map.contains_key(&loc) {
            let slot = map.len() as u32;

            unsafe {
                gl::ProgramUniform1i(prog, loc, slot as GLint);
            }
            map.insert(loc, slot);
        }
        
        let slot = map[&loc];
        
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + slot);
            val.bind();
        }
    }
}

#[cfg(debug_assertions)]
pub fn check_glerr_debug() {
    match unsafe { gl::GetError() } {
        gl::NO_ERROR => {},
        e => {
            panic!("GL error detected: {}", gl_error_to_str(e))
        }
    }
}

#[cfg(not(debug_assertions))]
pub fn check_glerr_debug() {}

pub fn gl_error_to_str(err: GLenum) -> &'static str {
    match err {
        gl::NO_ERROR => "No_error",
        gl::INVALID_ENUM => "Invalid_enum",
        gl::INVALID_VALUE => "Invalid_value",
        gl::INVALID_OPERATION => "Invalid_operation",
        gl::INVALID_FRAMEBUFFER_OPERATION => "Invalid_framebuffer_operation",
        gl::OUT_OF_MEMORY => "Out_of_memory",
        gl::STACK_UNDERFLOW => "Stack_underflow",
        gl::STACK_OVERFLOW => "Stack_overflow",
        _ => "Unknown",
    }
}

pub fn gl_get_string(val: gl::types::GLenum) -> String {
    unsafe {
        std::str::from_utf8(std::ffi::CStr::from_ptr(gl::GetString(val) as *const i8).to_bytes())
            .unwrap()
            .to_owned()
    }
}