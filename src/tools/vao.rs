use super::gl::types::*;
use super::Buffer;
use super::GlNum;
use super::Indices;
use std::collections::HashMap;
use std::ops::Range;
use tools::{BufferBindTarget, DrawMode};

#[derive(Debug)]
pub struct VertexArray {
    id: u32,
    indices: Indices,
    buf_lens: HashMap<u32, usize>,
    max_vertices: usize,
    index_buffer_size: usize,
}

impl VertexArray {
    pub fn new() -> VertexArray {
        let mut id = 0 as GLuint;
        unsafe {
            gl::CreateVertexArrays(1, &mut id);
        }

        VertexArray {
            id,
            indices: Indices::All,
            buf_lens: Default::default(),
            max_vertices: 0,
            index_buffer_size: 0,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn attrib_divisor(&mut self, index: u32, divisor: u32) {
        self.bind();

        unsafe {
            gl::VertexAttribDivisor(index as GLuint, divisor as GLuint);
        }
    }

    pub fn set_indices_all(&mut self) {
        self.indices = Indices::All;
    }
    pub fn set_indices_range(&mut self, range: Range<usize>) {
        self.indices = Indices::Range(range);
    }
    pub fn set_indices_vec(&mut self, indices: Vec<u32>) {
        self.indices = Indices::Vec(indices);
    }
    pub fn set_indices_buffer(&mut self, index_buffer: &Buffer<u32>) {
        self.indices = Indices::Buffer;
        self.index_buffer_size = index_buffer.len();
        self.bind();
        index_buffer.bind_to_target(BufferBindTarget::ElementArray);
        Self::unbind();
    }
    // pub fn index_buffer<I: GlNum>(&mut self, indices: Indices) {
    //     self.indices = indices;
    // }

    pub fn attrib_buffer<T: GlNum>(&mut self, index: u32, buf: &Buffer<T>) {
        self.attrib_buffer_offset(index, buf, 0)
    }

    pub fn attrib_buffer_offset<T: GlNum>(&mut self, index: u32, buf: &Buffer<T>, offset: usize) {
        if buf.len() <= offset {
            return;
        }

        self.bind();
        buf.bind();

        unsafe {
            gl::EnableVertexAttribArray(index);
            gl::VertexAttribPointer(
                index,
                T::dim() as GLint,
                T::gl_type(),
                gl::TRUE,
                0 as GLsizei,
                std::mem::transmute(offset * T::item_size()),
            );
        }

        self.update_max_vertixes(index, buf.len() - offset);
    }

    fn update_max_vertixes(&mut self, index: u32, length: usize) {
        self.buf_lens.insert(index, length);

        if length >= self.max_vertices {
            self.max_vertices = length;
        } else {
            self.max_vertices = self.buf_lens.values().max().map(|m| *m).unwrap_or(0);
        }
    }

    pub fn max_vertices(&self) -> usize {
        self.max_vertices
    }

    pub fn render(&self, instances: usize, mode: DrawMode) {
        if self.max_vertices == 0 {
            return;
        }

        self.bind();

        match (instances, &self.indices) {
            (0, _) => {}
            (_, Indices::None) => {}
            (1, Indices::Range(range)) => {
                let vertex_count = range.end - range.start;

                unsafe {
                    gl::DrawArrays(mode.into(), range.start as GLint, vertex_count as GLsizei);
                }
            }
            (1, Indices::All) => unsafe {
                gl::DrawArrays(mode.into(), 0, self.max_vertices as GLsizei);
            },
            (1, Indices::Vec(indices)) => unsafe {
                gl::DrawElements(
                    mode.into(),
                    indices.len() as GLsizei,
                    gl::UNSIGNED_INT,
                    indices.as_ptr() as *const _,
                );
            },
            (1, Indices::Buffer) => unsafe {
                gl::DrawElements(
                    mode.into(),
                    self.index_buffer_size as GLsizei,
                    gl::UNSIGNED_INT,
                    0 as *const _,
                );
            },
            (1, Indices::BufferRange(range)) => unsafe {
                gl::DrawElements(
                    mode.into(),
                    (range.end - range.start) as GLsizei,
                    gl::UNSIGNED_INT,
                    (range.start * u32::item_size()) as *const _,
                );
            },
            /////////////////////////// instanced /////////////////////////////////
            (instances, Indices::Range(range)) => {
                let vertex_count = range.end - range.start;

                unsafe {
                    gl::DrawArraysInstanced(
                        mode.into(),
                        range.start as GLint,
                        vertex_count as GLsizei,
                        instances as GLsizei,
                    );
                }
            }
            (instances, Indices::All) => unsafe {
                gl::DrawArraysInstanced(
                    mode.into(),
                    0,
                    self.max_vertices as GLsizei,
                    instances as GLsizei,
                );
            },
            (instances, Indices::Vec(indices)) => unsafe {
                gl::DrawElementsInstanced(
                    mode.into(),
                    indices.len() as GLsizei,
                    gl::UNSIGNED_INT,
                    indices.as_ptr() as *const _,
                    instances as GLsizei,
                );
            },
            (instances, Indices::Buffer) => unsafe {
                gl::DrawElementsInstanced(
                    mode.into(),
                    self.index_buffer_size as GLsizei,
                    gl::UNSIGNED_INT,
                    0 as *const _,
                    instances as GLsizei,
                );
            },
            (instances, Indices::BufferRange(range)) => unsafe {
                gl::DrawElementsInstanced(
                    mode.into(),
                    (range.end - range.start) as GLsizei,
                    gl::UNSIGNED_INT,
                    (range.start * u32::item_size()) as *const _,
                    instances as GLsizei,
                );
            },
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}
