use super::gl::types::GLenum;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BufferBindTarget {
    Array,
    AtomicCounter,
    CopyRead,
    CopyWrite,
    DispatchIndirect,
    DrawIndirect,
    ElementArray,
    PixelPack,
    PixelUnpack,
    Query,
    ShaderStorage,
    Texture,
    TransformFeedback,
    Uniform,
}

impl fmt::Display for BufferBindTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Array => "Array",
                Self::AtomicCounter => "AtomicCounter",
                Self::CopyRead => "CopyRead",
                Self::CopyWrite => "CopyWrite",
                Self::DispatchIndirect => "DispatchIndirect",
                Self::DrawIndirect => "DrawIndirect",
                Self::ElementArray => "ElementArray",
                Self::PixelPack => "PixelPack",
                Self::PixelUnpack => "PixelUnpack",
                Self::Query => "Query",
                Self::ShaderStorage => "ShaderStorage",
                Self::Texture => "Texture",
                Self::TransformFeedback => "TransformFeedback",
                Self::Uniform => "Uniform",
            }
        )
    }
}

impl From<BufferBindTarget> for GLenum {
    fn from(value: BufferBindTarget) -> Self {
        match value {
            BufferBindTarget::Array => gl::ARRAY_BUFFER,
            BufferBindTarget::AtomicCounter => gl::ATOMIC_COUNTER_BUFFER,
            BufferBindTarget::CopyRead => gl::COPY_READ_BUFFER,
            BufferBindTarget::CopyWrite => gl::COPY_WRITE_BUFFER,
            BufferBindTarget::DispatchIndirect => gl::DISPATCH_INDIRECT_BUFFER,
            BufferBindTarget::DrawIndirect => gl::DRAW_INDIRECT_BUFFER,
            BufferBindTarget::ElementArray => gl::ELEMENT_ARRAY_BUFFER,
            BufferBindTarget::PixelPack => gl::PIXEL_PACK_BUFFER,
            BufferBindTarget::PixelUnpack => gl::PIXEL_UNPACK_BUFFER,
            BufferBindTarget::Query => gl::QUERY_BUFFER,
            BufferBindTarget::ShaderStorage => gl::SHADER_STORAGE_BUFFER,
            BufferBindTarget::Texture => gl::TEXTURE_BUFFER,
            BufferBindTarget::TransformFeedback => gl::TRANSFORM_FEEDBACK_BUFFER,
            BufferBindTarget::Uniform => gl::UNIFORM_BUFFER,
        }
    }
}
