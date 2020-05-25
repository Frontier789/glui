extern crate gl;

use std::collections::HashMap;

use tools::{Texture, Vec2};

use self::gl::types::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FrameBufferStatus {
    IncompleteAttachment,
    IncompleteMissingAttachment,
    IncompleteDrawBuffer,
    IncompleteReadBuffer,
    Unsupported,
    IncompleteMultisample,
    IncompleteLayerTargets,
    Unknown,
}

impl FrameBufferStatus {
    pub fn from_gl_value(val: GLenum) -> Self {
        match val {
            gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => Self::IncompleteAttachment,
            gl::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => Self::IncompleteMissingAttachment,
            gl::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => Self::IncompleteDrawBuffer,
            gl::FRAMEBUFFER_INCOMPLETE_READ_BUFFER => Self::IncompleteReadBuffer,
            gl::FRAMEBUFFER_UNSUPPORTED => Self::Unsupported,
            gl::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => Self::IncompleteMultisample,
            gl::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => Self::IncompleteLayerTargets,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum FrameBufferAttachment {
    Color(usize),
    Depth,
    Stencil,
    DepthStencil,
}

impl FrameBufferAttachment {
    pub fn color_count() -> usize {
        let mut mx: GLint = 0;
        unsafe {
            gl::GetIntegerv(gl::MAX_COLOR_ATTACHMENTS, &mut mx);
        }
        mx as usize
    }
    pub fn to_gl_enum(self) -> GLenum {
        match self {
            Self::Color(i) => gl::COLOR_ATTACHMENT0 + i as GLenum,
            Self::Depth => gl::DEPTH_ATTACHMENT,
            Self::Stencil => gl::STENCIL_ATTACHMENT,
            Self::DepthStencil => gl::DEPTH_STENCIL_ATTACHMENT,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct AttachmentData {
    id: u32,
    size: (usize, usize),
}

#[derive(Eq)]
pub struct Framebuffer {
    id: u32,
    attachments: HashMap<FrameBufferAttachment, AttachmentData>,
    size: Option<(usize, usize)>,
}

impl PartialEq for Framebuffer {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Framebuffer {
    pub fn new() -> Framebuffer {
        let mut id: GLuint = 0;
        unsafe {
            gl::CreateFramebuffers(1, &mut id);
        }

        Framebuffer {
            id,
            attachments: HashMap::new(),
            size: None,
        }
    }

    pub fn status(&self) -> FrameBufferStatus {
        let stat = unsafe { gl::CheckNamedFramebufferStatus(self.id, gl::FRAMEBUFFER) };
        FrameBufferStatus::from_gl_value(stat)
    }

    pub fn attach_texture<T>(&mut self, attachment_point: FrameBufferAttachment, texture: &T)
    where
        T: Texture,
    {
        let cur = self
            .attachments
            .get(&attachment_point)
            .map(|data| data.id == texture.id());
        if cur != Some(true) {
            unsafe {
                gl::NamedFramebufferTexture(
                    self.id,
                    attachment_point.to_gl_enum(),
                    texture.id(),
                    0,
                );
            }

            self.attachments.insert(
                attachment_point,
                AttachmentData {
                    id: texture.id(),
                    size: texture.size_2d(),
                },
            );

            self.intersect_size(texture.size_2d());
        }
    }
    fn intersect_size(&mut self, ts: (usize, usize)) {
        match self.size {
            None => {
                self.size = Some(ts);
            }
            Some(mut s) => {
                if s.0 < ts.0 {
                    s.0 = ts.0;
                }
                if s.1 < ts.1 {
                    s.1 = ts.1;
                }
            }
        }
    }

    pub fn detach_texture(&mut self, attachment_point: FrameBufferAttachment) {
        if let Some(&attachment) = self.attachments.get(&attachment_point) {
            self.attachments.remove(&attachment_point);

            if attachment.id != 0 {
                unsafe {
                    gl::NamedFramebufferTexture(self.id, attachment_point.to_gl_enum(), 0, 0);
                }

                self.find_size(attachment.size);
            }
        }
    }
    fn find_size(&mut self, removed_size: (usize, usize)) {
        if let Some(mut s) = self.size {
            if s.0 == removed_size.0 || s.1 == removed_size.1 {
                if self.attachments.is_empty() {
                    self.size = None;
                } else {
                    s = (std::usize::MAX, std::usize::MAX);
                    for (_, attachment) in self.attachments.iter() {
                        s.0 = s.0.min(attachment.size.0);
                        s.1 = s.1.min(attachment.size.1);
                    }
                    self.size = Some(s);
                }
            }
        }
    }

    pub fn set_draw_targets(&mut self, targets: Vec<FrameBufferAttachment>) {
        let enums: Vec<GLenum> = targets.iter().map(|a| a.to_gl_enum()).collect();

        unsafe {
            gl::NamedFramebufferDrawBuffers(self.id, enums.len() as GLsizei, enums.as_ptr());
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
            if let Some(s) = self.size {
                gl::Viewport(0, 0, s.0 as GLsizei, s.1 as GLsizei);
            }
        }
    }

    pub fn bind_def_framebuffer(size: Option<Vec2>) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            if let Some(s) = size {
                gl::Viewport(0, 0, s.x as GLsizei, s.y as GLsizei);
            }
        }
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        let mut id = self.id as _;
        unsafe {
            gl::DeleteFramebuffers(1, &mut id);
        }
    }
}
