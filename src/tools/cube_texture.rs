extern crate gl;
extern crate image;
extern crate maplit;

use self::gl::types::*;
use self::image::buffer::ConvertBuffer;
use self::image::io::Reader;
use self::image::GenericImageView;
use self::maplit::hashmap;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use tools::texture::alignment_from_format_u8;
use tools::Texture;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CubeFace {
    PositiveX,
    NegativeX,
    PositiveY,
    NegativeY,
    PositiveZ,
    NegativeZ,
}

impl From<CubeFace> for GLenum {
    fn from(val: CubeFace) -> Self {
        match val {
            CubeFace::PositiveX => gl::TEXTURE_CUBE_MAP_POSITIVE_X,
            CubeFace::NegativeX => gl::TEXTURE_CUBE_MAP_NEGATIVE_X,
            CubeFace::PositiveY => gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
            CubeFace::NegativeY => gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
            CubeFace::PositiveZ => gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
            CubeFace::NegativeZ => gl::TEXTURE_CUBE_MAP_NEGATIVE_Z,
        }
    }
}

impl Display for CubeFace {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CubeFace::PositiveX => write!(f, "PositiveX"),
            CubeFace::NegativeX => write!(f, "NegativeX"),
            CubeFace::PositiveY => write!(f, "PositiveY"),
            CubeFace::NegativeY => write!(f, "NegativeY"),
            CubeFace::PositiveZ => write!(f, "PositiveZ"),
            CubeFace::NegativeZ => write!(f, "NegativeZ"),
        }
    }
}

impl CubeFace {
    pub fn iter_faces() -> impl Iterator<Item = &'static CubeFace> {
        [
            CubeFace::PositiveX,
            CubeFace::NegativeX,
            CubeFace::PositiveY,
            CubeFace::NegativeY,
            CubeFace::PositiveZ,
            CubeFace::NegativeZ,
        ]
        .iter()
    }

    pub fn index(self) -> u32 {
        match self {
            CubeFace::PositiveX => 0,
            CubeFace::NegativeX => 1,
            CubeFace::PositiveY => 2,
            CubeFace::NegativeY => 3,
            CubeFace::PositiveZ => 4,
            CubeFace::NegativeZ => 5,
        }
    }
}

#[derive(Debug)]
pub struct CubeTextureFace<'a> {
    cube_tex: &'a CubeTexture,
    face: CubeFace,
}

impl<'a> CubeTextureFace<'a> {
    fn new(cube_tex: &'a CubeTexture, face: CubeFace) -> CubeTextureFace<'a> {
        CubeTextureFace { cube_tex, face }
    }

    pub fn as_image(&self) -> image::RgbaImage {
        let s = self.cube_tex.size;
        let mut img = image::RgbaImage::new(s as u32, s as u32);
        let buf_size = s * s * 4 * std::mem::size_of::<u8>();
        unsafe {
            gl::GetTextureSubImage(
                self.id(),
                0,
                0,
                0,
                self.face.index() as GLint,
                s as GLsizei,
                s as GLsizei,
                1,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                buf_size as GLsizei,
                img.as_mut_ptr() as *mut std::ffi::c_void,
            );
        }
        img
    }
}

impl<'a> Texture for CubeTextureFace<'a> {
    fn id(&self) -> u32 {
        self.cube_tex.id
    }

    fn bind(&self) {
        self.cube_tex.bind();
    }

    fn size_2d(&self) -> (usize, usize) {
        (self.cube_tex.size, self.cube_tex.size)
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

            gl::TextureSubImage3D(
                self.id(),
                0,
                x as GLint,
                y as GLint,
                self.face.index() as _,
                width as GLsizei,
                height as GLsizei,
                1,
                format,
                type_,
                ptr,
            );

            gl::PixelStorei(gl::UNPACK_ROW_LENGTH, 0);
        }
    }
}

#[derive(Debug, Clone)]
pub struct FaceLayout {
    pub face_cells: HashMap<CubeFace, (usize, usize)>,
    pub cell_count: (usize, usize),
}

impl Default for FaceLayout {
    fn default() -> Self {
        FaceLayout {
            face_cells: hashmap! {
                CubeFace::PositiveX => (2,1),
                CubeFace::NegativeX => (0,1),
                CubeFace::PositiveY => (1,0),
                CubeFace::NegativeY => (1,2),
                CubeFace::PositiveZ => (1,1),
                CubeFace::NegativeZ => (3,1),
            },
            cell_count: (4, 3),
        }
    }
}
impl FaceLayout {
    pub fn bottom_in_middle() -> Self {
        FaceLayout {
            face_cells: hashmap! {
                CubeFace::PositiveX => (1,0),
                CubeFace::NegativeX => (1,2),
                CubeFace::PositiveY => (3,1),
                CubeFace::NegativeY => (1,1),
                CubeFace::PositiveZ => (2,1),
                CubeFace::NegativeZ => (0,1),
            },
            cell_count: (4, 3),
        }
    }
}

#[derive(Debug)]
pub struct CubeTexture {
    id: u32,
    size: usize,
}

impl CubeTexture {
    pub fn new(size: usize) -> CubeTexture {
        let mut id: GLuint = 0;
        unsafe {
            gl::CreateTextures(gl::TEXTURE_CUBE_MAP, 1, &mut id);
        }

        unsafe {
            gl::TextureParameteri(id, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TextureParameteri(id, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as GLint);
            gl::TextureParameteri(id, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
            gl::TextureParameteri(id, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);

            gl::TextureStorage2D(id, 1, gl::RGBA8 as GLenum, size as GLsizei, size as GLsizei);
        }
        CubeTexture { id, size }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.id);
        }
    }

    pub fn face(&self, face: CubeFace) -> CubeTextureFace {
        CubeTextureFace::new(self, face)
    }

    pub fn from_ptr(
        size: usize,
        layout: FaceLayout,
        format: GLenum,
        type_: GLenum,
        ptr: *const std::ffi::c_void,
        alignment: GLint,
        stride_row: usize,
        bytes_per_pixel: usize,
    ) -> CubeTexture {
        let tex = CubeTexture::new(size);
        for (face, (x, y)) in layout.face_cells {
            let mut tex_face = tex.face(face);
            tex_face.gl_update(
                0,
                0,
                size,
                size,
                format,
                type_,
                unsafe {
                    (ptr as *mut u8).add(bytes_per_pixel * (stride_row * size * y + size * x)) as _
                },
                alignment,
                stride_row,
            );
        }
        tex
    }
    pub fn from_ptr_u8(
        size: usize,
        layout: FaceLayout,
        format: GLenum,
        ptr: *const u8,
        stride_row: usize,
        bytes_per_pixel: usize,
    ) -> CubeTexture {
        CubeTexture::from_ptr(
            size,
            layout,
            format,
            gl::UNSIGNED_BYTE,
            ptr as *const std::ffi::c_void,
            alignment_from_format_u8(format),
            stride_row,
            bytes_per_pixel,
        )
    }
    pub fn from_file(file: &str, layout: FaceLayout) -> image::ImageResult<CubeTexture> {
        let reader = Reader::open(file)?;
        let reader = reader.with_guessed_format()?;
        let img = reader.decode()?;

        let (w, h) = img.dimensions();
        let (w, h) = (w as usize, h as usize);
        let sizew = w / layout.cell_count.0;
        let sizeh = h / layout.cell_count.1;
        let size = sizew.min(sizeh);
        match img {
            image::DynamicImage::ImageLuma8(img) => {
                let rgb_img: image::RgbImage = img.convert();
                Ok(CubeTexture::from_ptr_u8(
                    size,
                    layout,
                    gl::RGB,
                    rgb_img.as_ptr(),
                    w,
                    3,
                ))
            }
            image::DynamicImage::ImageLumaA8(img) => {
                let rgba_img: image::RgbaImage = img.convert();
                Ok(CubeTexture::from_ptr_u8(
                    size,
                    layout,
                    gl::RGBA,
                    rgba_img.as_ptr(),
                    w,
                    4,
                ))
            }
            image::DynamicImage::ImageRgb8(img) => Ok(CubeTexture::from_ptr_u8(
                size,
                layout,
                gl::RGB,
                img.as_ptr(),
                w,
                3,
            )),
            image::DynamicImage::ImageRgba8(img) => Ok(CubeTexture::from_ptr_u8(
                size,
                layout,
                gl::RGBA,
                img.as_ptr(),
                w,
                4,
            )),
            image::DynamicImage::ImageBgr8(img) => Ok(CubeTexture::from_ptr_u8(
                size,
                layout,
                gl::BGR,
                img.as_ptr(),
                w,
                3,
            )),
            image::DynamicImage::ImageBgra8(img) => Ok(CubeTexture::from_ptr_u8(
                size,
                layout,
                gl::BGRA,
                img.as_ptr(),
                w,
                4,
            )),
            image::DynamicImage::ImageLuma16(img) => {
                let rgb_img: image::RgbImage = img.convert();
                Ok(CubeTexture::from_ptr_u8(
                    size,
                    layout,
                    gl::RGB,
                    rgb_img.as_ptr(),
                    w,
                    3,
                ))
            }
            image::DynamicImage::ImageLumaA16(img) => {
                let rgba_img: image::RgbaImage = img.convert();
                Ok(CubeTexture::from_ptr_u8(
                    size,
                    layout,
                    gl::RGBA,
                    rgba_img.as_ptr(),
                    w,
                    4,
                ))
            }
            image::DynamicImage::ImageRgb16(img) => {
                let rgb_img: image::RgbImage = img.convert();
                Ok(CubeTexture::from_ptr_u8(
                    size,
                    layout,
                    gl::RGB,
                    rgb_img.as_ptr(),
                    w,
                    3,
                ))
            }
            image::DynamicImage::ImageRgba16(img) => {
                let rgba_img: image::RgbaImage = img.convert();
                Ok(CubeTexture::from_ptr_u8(
                    size,
                    layout,
                    gl::RGBA,
                    rgba_img.as_ptr(),
                    w,
                    4,
                ))
            }
        }
    }
}
