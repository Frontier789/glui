extern crate gl;
extern crate image;

use super::gl::types::*;
use super::image::buffer::ConvertBuffer;
use super::image::io::Reader;
use super::image::GenericImageView;
use super::Vec2;
use tools::texture::alignment_from_format_u8;
use tools::{Texture, Vec3, Vec4};

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
    pub fn unit() -> RgbaTexture {
        Self::from_vec_u8(1, 1, &vec![255, 255, 255, 255])
    }
    pub fn new_color(width: usize, height: usize, col: Vec4) -> RgbaTexture {
        let mut v = Vec::with_capacity(width * height * 4);
        let r = (col.x * 255.0) as u8;
        let g = (col.y * 255.0) as u8;
        let b = (col.z * 255.0) as u8;
        let a = (col.w * 255.0) as u8;
        for _ in 0..width * height {
            v.push(r);
            v.push(g);
            v.push(b);
            v.push(a);
        }
        Self::from_vec_u8(width, height, &v)
    }
    pub fn from_vec_u8(width: usize, height: usize, vec: &Vec<u8>) -> RgbaTexture {
        RgbaTexture::from_ptr(
            width,
            height,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            vec.as_ptr() as *const std::ffi::c_void,
            4,
            0,
        )
    }
    pub fn from_vec_v3_rescale(width: usize, height: usize, vec: &Vec<Vec3>) -> RgbaTexture {
        let mut data = Vec::with_capacity(width * height * 4);
        for n in vec.into_iter() {
            data.push(((n.x + 1.0) / 2.0 * 255.0) as u8);
            data.push(((n.y + 1.0) / 2.0 * 255.0) as u8);
            data.push(((n.z + 1.0) / 2.0 * 255.0) as u8);
            data.push(0);
        }
        Self::from_vec_u8(width, height, &data)
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
    pub fn load_rgba_image(file: &str) -> image::ImageResult<image::RgbaImage> {
        let reader = Reader::open(file)?;
        let reader = reader.with_guessed_format()?;
        Ok(reader.decode()?.to_rgba())
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

#[derive(Debug)]
pub struct FloatTexture {
    id: u32,
    width: usize,
    height: usize,
}

impl Texture for FloatTexture {
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

impl FloatTexture {
    pub fn new(width: usize, height: usize) -> FloatTexture {
        let mut id: GLuint = 0;
        unsafe {
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut id);
        }

        unsafe {
            gl::TextureParameteri(id, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TextureParameteri(id, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TextureParameteri(id, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);

            gl::TextureStorage2D(
                id,
                1,
                gl::R32F as GLenum,
                width as GLsizei,
                height as GLsizei,
            );
        }
        FloatTexture { id, width, height }
    }

    pub fn from_ptr(
        width: usize,
        height: usize,
        format: GLenum,
        type_: GLenum,
        ptr: *const std::ffi::c_void,
        stride: usize,
    ) -> FloatTexture {
        let mut tex = FloatTexture::new(width, height);
        tex.gl_update(0, 0, width, height, format, type_, ptr, 4, stride);
        tex
    }
    pub fn from_ptr_f32(
        width: usize,
        height: usize,
        ptr: *const f32,
        stride: usize,
    ) -> FloatTexture {
        FloatTexture::from_ptr(
            width,
            height,
            gl::RED,
            gl::FLOAT,
            ptr as *const std::ffi::c_void,
            stride,
        )
    }
    pub fn from_vec(width: usize, height: usize, vec: &Vec<f32>) -> FloatTexture {
        FloatTexture::from_ptr_f32(width, height, vec.as_ptr(), 0)
    }
    pub fn as_vec(&self) -> Vec<f32> {
        let mut buf = vec![0.0; self.width * self.height];
        let buf_size = self.width * self.height * std::mem::size_of::<f32>();
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
                gl::RED,
                gl::FLOAT,
                buf_size as GLsizei,
                buf.as_mut_ptr() as *mut std::ffi::c_void,
            );
        }
        buf
    }
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32)
    }
}

impl Drop for FloatTexture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
