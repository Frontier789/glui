extern crate gl;

use self::gl::types::*;

pub(super) fn alignment_from_format_u8(format: GLenum) -> GLint {
    match format {
        gl::RED => 1,
        gl::RG => 2,
        gl::RGB | gl::BGR => 1,
        gl::RGBA | gl::BGRA => 4,
        _ => panic!("Invalid Image Format"),
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TextureFiltering {
    Linear,
    Nearest,
    Trilinear,
}

impl From<TextureFiltering> for GLint {
    fn from(filter: TextureFiltering) -> Self {
        (match filter {
            TextureFiltering::Linear => gl::LINEAR,
            TextureFiltering::Nearest => gl::NEAREST,
            TextureFiltering::Trilinear => gl::LINEAR_MIPMAP_LINEAR,
        }) as GLint
    }
}

pub trait Texture {
    fn id(&self) -> u32;
    fn bind(&self);
    fn size_2d(&self) -> (usize, usize);
    fn gl_update(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        format: GLenum,
        type_: GLenum,
        ptr: *const std::ffi::c_void,
        alignment: GLint,
        stride: usize,
    );

    fn update_u8(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        format: GLenum,
        ptr: *const u8,
        stride: usize,
    ) {
        self.gl_update(
            x,
            y,
            width,
            height,
            format,
            gl::UNSIGNED_BYTE,
            ptr as *const std::ffi::c_void,
            alignment_from_format_u8(format),
            stride,
        );
    }

    fn generate_mipmaps(&self) {
        unsafe {
            gl::GenerateTextureMipmap(self.id());
        }
    }

    fn set_filtering(&self, filtering: TextureFiltering) {
        unsafe {
            gl::TextureParameteri(self.id(), gl::TEXTURE_MIN_FILTER, filtering.into());
        }
    }
}
