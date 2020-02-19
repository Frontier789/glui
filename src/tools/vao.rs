use gl::types::*;
use super::Buffer;
use super::GlNum;

pub struct VertexArray {
    id: u32
}

impl VertexArray {
    pub fn new() -> VertexArray {
        let mut id = 0 as GLuint;
        unsafe { gl::CreateVertexArrays(1,&mut id); }
        
        VertexArray {
            id: id
        }
    }
    
    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id); }
    }
    
    pub fn attrib_divisor(&mut self, index: u32, divisor: u32) {
        self.bind();
        
        unsafe {
            gl::VertexAttribDivisor(index as GLuint, divisor as GLuint);
        }
    }
    
    pub fn attrib_buffer<T: GlNum>(&mut self, index: u32, buf: &Buffer<T>) {
        self.attrib_buffer_offset(index,buf,0)
    }
    
    pub fn attrib_buffer_offset<T: GlNum>(&mut self, index: u32, buf: &Buffer<T>, offset: usize) {
        buf.bind();
        self.bind();
        
        unsafe {
            gl::EnableVertexAttribArray(index);
            gl::VertexAttribPointer(index, T::dim() as GLint, T::gl_type(), gl::TRUE, 0 as GLsizei, std::mem::transmute(offset));
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1,&self.id); }
    }
}