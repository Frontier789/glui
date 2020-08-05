extern crate gl;
extern crate image;

use super::gl::types::*;
use super::image::buffer::ConvertBuffer;
use super::image::io::Reader;
use super::image::GenericImageView;
use super::Vec2;
use tools::texture::alignment_from_format_u8;
use tools::Texture;

pub type ImageError = image::ImageError;

#[derive(Debug)]
pub struct RgbaTexture {
    id: u32,
    width: usize,
    height: usize,
}

impl Texture for RgbaTexture {
    fn id(&self) -> u32 {
        self.id
    }

    fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    fn size_2d(&self) -> (usize, usize) {
        (self.width, self.height)
    }

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
    ) {
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, alignment);
            gl::PixelStorei(gl::UNPACK_ROW_LENGTH, stride as GLint);

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

            gl::PixelStorei(gl::UNPACK_ROW_LENGTH, 0);
        }
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
        RgbaTexture { id, width, height }
    }

    pub fn from_ptr(
        width: usize,
        height: usize,
        format: GLenum,
        type_: GLenum,
        ptr: *const std::ffi::c_void,
        alignment: GLint,
        stride: usize,
    ) -> RgbaTexture {
        let mut tex = RgbaTexture::new(width, height);
        tex.gl_update(0, 0, width, height, format, type_, ptr, alignment, stride);
        tex
    }
    pub fn from_ptr_u8(
        width: usize,
        height: usize,
        format: GLenum,
        ptr: *const u8,
        stride: usize,
    ) -> RgbaTexture {
        RgbaTexture::from_ptr(
            width,
            height,
            format,
            gl::UNSIGNED_BYTE,
            ptr as *const std::ffi::c_void,
            alignment_from_format_u8(format),
            stride,
        )
    }
    pub fn from_file(file: &str) -> image::ImageResult<RgbaTexture> {
        let reader = Reader::open(file)?;
        let reader = reader.with_guessed_format()?;
        let img = reader.decode()?;
        // image::imageops::flip_vertical_in_place(&mut img);

        let (w, h) = img.dimensions();
        let w = w as usize;
        let h = h as usize;
        match img {
            image::DynamicImage::ImageLuma8(img) => {
                let rgb_img: image::RgbImage = img.convert();
                Ok(RgbaTexture::from_ptr_u8(w, h, gl::RGB, rgb_img.as_ptr(), 0))
            }
            image::DynamicImage::ImageLumaA8(img) => {
                let rgba_img: image::RgbaImage = img.convert();
                Ok(RgbaTexture::from_ptr_u8(
                    w,
                    h,
                    gl::RGBA,
                    rgba_img.as_ptr(),
                    0,
                ))
            }
            image::DynamicImage::ImageRgb8(img) => {
                Ok(RgbaTexture::from_ptr_u8(w, h, gl::RGB, img.as_ptr(), 0))
            }
            image::DynamicImage::ImageRgba8(img) => {
                Ok(RgbaTexture::from_ptr_u8(w, h, gl::RGBA, img.as_ptr(), 0))
            }
            image::DynamicImage::ImageBgr8(img) => {
                Ok(RgbaTexture::from_ptr_u8(w, h, gl::BGR, img.as_ptr(), 0))
            }
            image::DynamicImage::ImageBgra8(img) => {
                Ok(RgbaTexture::from_ptr_u8(w, h, gl::BGRA, img.as_ptr(), 0))
            }
            image::DynamicImage::ImageLuma16(img) => {
                let rgb_img: image::RgbImage = img.convert();
                Ok(RgbaTexture::from_ptr_u8(w, h, gl::RGB, rgb_img.as_ptr(), 0))
            }
            image::DynamicImage::ImageLumaA16(img) => {
                let rgba_img: image::RgbaImage = img.convert();
                Ok(RgbaTexture::from_ptr_u8(
                    w,
                    h,
                    gl::RGBA,
                    rgba_img.as_ptr(),
                    0,
                ))
            }
            image::DynamicImage::ImageRgb16(img) => {
                let rgb_img: image::RgbImage = img.convert();
                Ok(RgbaTexture::from_ptr_u8(w, h, gl::RGB, rgb_img.as_ptr(), 0))
            }
            image::DynamicImage::ImageRgba16(img) => {
                let rgba_img: image::RgbaImage = img.convert();
                Ok(RgbaTexture::from_ptr_u8(
                    w,
                    h,
                    gl::RGBA,
                    rgba_img.as_ptr(),
                    0,
                ))
            }
        }
    }
    pub fn as_image(&self) -> image::RgbaImage {
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
