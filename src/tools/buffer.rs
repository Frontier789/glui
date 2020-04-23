use super::gltraits::GlNum;
use super::gl::types::*;
use std::ffi::c_void;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Buffer<T: GlNum> {
    id: u32,
    size: usize,
    _data_type: PhantomData<T>,
}

impl<T: GlNum> Buffer<T> {
    pub fn new() -> Buffer<T> {
        let mut id: GLuint = 0;
        unsafe {
            gl::CreateBuffers(1, &mut id);
        }
        Buffer {
            id: id,
            size: 0,
            _data_type: PhantomData,
        }
    }
    pub fn from_vec(v: Vec<T>) -> Buffer<T> {
        let mut buf = Buffer::<T>::new();
        buf.set_data_reinterpret(v);
        buf
    }
    pub fn from_base_vec(v: Vec<T::BaseType>) -> Buffer<T> {
        let mut buf = Buffer::<T>::new();
        buf.set_data_reinterpret(v);
        buf
    }
    pub fn len(&self) -> usize {
        self.size
    }

    pub fn set_data(&mut self, v: Vec<T>) {
        self.set_data_reinterpret(v)
    }

    pub fn set_data_reinterpret<Q>(&mut self, v: Vec<Q>) {
        let size = std::mem::size_of::<Q>() * v.len();
        let ptr = v.as_ptr() as *const c_void;
        unsafe {
            gl::NamedBufferData(self.id, size as GLsizeiptr, ptr, gl::DYNAMIC_DRAW);
        }
        self.size = size / std::mem::size_of::<T>();
    }

    pub fn update(&mut self, v: Vec<T>, offset: usize) {
        if offset >= self.size {
            return ();
        }
        let mut elems = v.len();
        if offset + elems > self.size {
            elems = self.size - offset;
        }
        let size = std::mem::size_of::<T>() * elems;
        let offs = std::mem::size_of::<T>() * offset;
        let ptr = v.as_ptr() as *const c_void;
        unsafe {
            gl::NamedBufferSubData(self.id, offs as GLintptr, size as GLsizeiptr, ptr);
        }
    }
    pub fn data(&self) -> Vec<T> {
        let mut v = Vec::<T>::with_capacity(self.size);
        unsafe {
            let ptr = gl::MapNamedBuffer(self.id, gl::READ_ONLY);
            std::ptr::copy_nonoverlapping(ptr as *const T, v.as_mut_ptr(), self.size);
            gl::UnmapNamedBuffer(self.id);
        }
        let ptr = v.as_mut_ptr();
        let cap = v.capacity();
        std::mem::forget(v);
        unsafe { Vec::from_raw_parts(ptr, cap, cap) }
    }
    pub fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.id) };
    }

    pub fn as_base_type(mut self) -> Buffer<T::BaseType>
    where
        T::BaseType: GlNum,
    {
        let id = self.id;
        self.id = 0;
        Buffer {
            id,
            size: self.size * T::dim() as usize,
            _data_type: PhantomData,
        }
    }

    pub fn as_type<U>(mut self) -> Buffer<U>
    where
        U: GlNum<BaseType = T>,
    {
        let id = self.id;
        self.id = 0;
        Buffer {
            id,
            size: self.size / U::dim() as usize,
            _data_type: PhantomData,
        }
    }
}

impl<T: GlNum> Drop for Buffer<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
