extern crate gl;
extern crate image;

use image::io::Reader;
use gl::types::*;
use super::Vec2;
use image::GenericImageView;

pub struct Texture {
    id: u32,
    width: usize,
    height: usize
}

use image::ConvertBuffer;

impl Texture {
    fn create_u8(width: usize, height: usize, format: GLenum, ptr: *const u8) -> Texture {
        Texture::create(width, height, format, gl::UNSIGNED_BYTE, ptr as *const std::ffi::c_void)
    }
    
    fn create(width: usize, height: usize, format: GLenum, type_: GLenum, ptr: *const std::ffi::c_void) -> Texture {
        let mut id: GLuint = 0;
        
        unsafe { gl::GenTextures(1, &mut id); }
        
        let tex = Texture {
            id: id,
            width: width,
            height: height
        };
        
        tex.bind();
        
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
            
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as GLint, width as GLsizei, height as GLsizei, 0, format, type_, ptr);
        }
        
        tex
    }
    
    pub fn new(width: usize, height: usize) -> Texture {
        Texture::create(width, height, gl::RGBA, gl::UNSIGNED_BYTE, std::ptr::null())
    }
    
    pub fn from_file(file: &str) -> image::ImageResult<Texture> {
        let reader = Reader::open(file)?;
        let reader = reader.with_guessed_format()?;
        
        let img = reader.decode()?;
        
        let (w,h) = img.dimensions();
        let w = w as usize; 
        let h = h as usize; 
        
        match img {
            image::ImageLuma8(img) => {
                let rgb_img: image::RgbImage = img.convert();
                Ok(Texture::create_u8(w, h, gl::RGB, rgb_img.as_ptr()))
            }
            image::ImageLumaA8(img) => {
                let rgba_img: image::RgbaImage = img.convert();
                Ok(Texture::create_u8(w, h, gl::RGBA, rgba_img.as_ptr()))
            }
            image::ImageRgb8(img) => {
                Ok(Texture::create_u8(w, h, gl::RGB, img.as_ptr()))
            }
            image::ImageRgba8(img) =>  {
                Ok(Texture::create_u8(w, h, gl::RGBA, img.as_ptr()))
            }
            image::ImageBgr8(img) =>  {
                Ok(Texture::create_u8(w, h, gl::BGR, img.as_ptr()))
            }
            image::ImageBgra8(img) =>  {
                Ok(Texture::create_u8(w, h, gl::BGRA, img.as_ptr()))
            }
        }
    }
    
    pub fn into_image(&self) -> image::RgbaImage {
        let mut img = image::RgbaImage::new(self.width as u32, self.height as u32);
        let buf_size = self.width * self.height * 4 * std::mem::size_of::<u8>();
        
        unsafe {
            gl::GetTextureSubImage(
                self.id, 0, 0,0,0, 
                self.width as GLsizei, self.height as GLsizei, 1,
                gl::RGBA, gl::UNSIGNED_BYTE, buf_size as GLsizei, img.as_mut_ptr() as *mut std::ffi::c_void
            );
        }
        
        img
    }
    
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32)
    }
    
    pub fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.id); }
    }
    
    pub fn get_id(&self) -> u32 { self.id }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id); }
    }
}