extern crate gl;
extern crate image;

use super::Vec2;
use gl::types::*;
use image::io::Reader;
use image::GenericImageView;

pub trait Texture {
    fn id(&self) -> u32;
    fn bind(&self);
}

#[derive(Debug)]
pub struct RgbaTexture {
    id: u32,
    width: usize,
    height: usize,
}

use image::ConvertBuffer;

impl Texture for RgbaTexture {
    fn id(&self) -> u32 {
        self.id
    }
    
    fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

fn alignment_from_format_u8(format: GLenum) -> GLint {
    match format {
        gl::RED => 1,
        gl::RG => 2,
        gl::RGB | gl::BGR => 1,
        gl::RGBA | gl::BGRA => 4,
        _ => panic!("Invalid Image Format")
    }
}

impl RgbaTexture {
    pub fn new(width: usize, height: usize) -> RgbaTexture {
        let mut id: GLuint = 0;
        unsafe {
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut id);
        }

        unsafe {
            gl::TextureParameteri(id, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TextureParameteri(id, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl::TextureParameteri(id, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
            gl::TextureParameteri(id, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);

            gl::TextureStorage2D(
                id,
                1,
                gl::RGBA8 as GLenum,
                width as GLsizei,
                height as GLsizei,
            );
        }
        RgbaTexture {
            id: id,
            width: width,
            height: height,
        }
    }

    fn update(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        format: GLenum,
        type_: GLenum,
        ptr: *const std::ffi::c_void,
        alignment: GLint,
    ) {
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, alignment);
                
            gl::TextureSubImage2D(
                self.id(),
                0,
                x as GLint,
                y as GLint,
                width as GLsizei,
                height as GLsizei,
                format,
                type_,
                ptr,
            );
        }
    }

    pub fn update_u8(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        format: GLenum,
        ptr: *const u8,
    ) {
        self.update(
            x,
            y,
            width,
            height,
            format,
            gl::UNSIGNED_BYTE,
            ptr as *const std::ffi::c_void,
            alignment_from_format_u8(format),
        );
    }

    pub fn from_ptr(
        width: usize,
        height: usize,
        format: GLenum,
        type_: GLenum,
        ptr: *const std::ffi::c_void,
        alignment: GLint,
    ) -> RgbaTexture {
        let mut tex = RgbaTexture::new(width, height);
        tex.update(0, 0, width, height, format, type_, ptr, alignment);
        tex
    }
    pub fn from_ptr_u8(width: usize, height: usize, format: GLenum, ptr: *const u8) -> RgbaTexture {
        RgbaTexture::from_ptr(
            width,
            height,
            format,
            gl::UNSIGNED_BYTE,
            ptr as *const std::ffi::c_void,
            alignment_from_format_u8(format),
        )
    }
    pub fn from_file(file: &str) -> image::ImageResult<RgbaTexture> {
        let reader = Reader::open(file)?;
        let reader = reader.with_guessed_format()?;
        let img = reader.decode()?;

        let (w, h) = img.dimensions();
        let w = w as usize;
        let h = h as usize;
        match img {
            image::DynamicImage::ImageLuma8(img) => {
                let rgb_img: image::RgbImage = img.convert();
                Ok(RgbaTexture::from_ptr_u8(w, h, gl::RGB, rgb_img.as_ptr()))
            }
            image::DynamicImage::ImageLumaA8(img) => {
                let rgba_img: image::RgbaImage = img.convert();
                Ok(RgbaTexture::from_ptr_u8(w, h, gl::RGBA, rgba_img.as_ptr()))
            }
            image::DynamicImage::ImageRgb8(img) => Ok(RgbaTexture::from_ptr_u8(w, h, gl::RGB, img.as_ptr())),
            image::DynamicImage::ImageRgba8(img) => Ok(RgbaTexture::from_ptr_u8(w, h, gl::RGBA, img.as_ptr())),
            image::DynamicImage::ImageBgr8(img) => Ok(RgbaTexture::from_ptr_u8(w, h, gl::BGR, img.as_ptr())),
            image::DynamicImage::ImageBgra8(img) => Ok(RgbaTexture::from_ptr_u8(w, h, gl::BGRA, img.as_ptr())),
            image::DynamicImage::ImageLuma16(img) => {
                let rgb_img: image::RgbImage = img.convert();
                Ok(RgbaTexture::from_ptr_u8(w, h, gl::RGB, rgb_img.as_ptr()))
            }
            image::DynamicImage::ImageLumaA16(img) => {
                let rgba_img: image::RgbaImage = img.convert();
                Ok(RgbaTexture::from_ptr_u8(w, h, gl::RGBA, rgba_img.as_ptr()))
            }
            image::DynamicImage::ImageRgb16(img) => {
                let rgb_img: image::RgbImage = img.convert();
                Ok(RgbaTexture::from_ptr_u8(w, h, gl::RGB, rgb_img.as_ptr()))
            }
            image::DynamicImage::ImageRgba16(img) => {
                let rgba_img: image::RgbaImage = img.convert();
                Ok(RgbaTexture::from_ptr_u8(w, h, gl::RGBA, rgba_img.as_ptr()))
            }
        }
    }
    pub fn into_image(&self) -> image::RgbaImage {
        let mut img = image::RgbaImage::new(self.width as u32, self.height as u32);
        let buf_size = self.width * self.height * 4 * std::mem::size_of::<u8>();
        unsafe {
            gl::GetTextureSubImage(
                self.id,
                0,
                0,
                0,
                0,
                self.width as GLsizei,
                self.height as GLsizei,
                1,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                buf_size as GLsizei,
                img.as_mut_ptr() as *mut std::ffi::c_void,
            );
        }
        img
    }
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32)
    }
}

impl Drop for RgbaTexture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
