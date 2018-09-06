#![allow(unused_imports)]

use std::mem;

use gles::consts::*;
use gles::es20::ffi::*;
use gles::types::*;

use super::{BufferTarget, Usage};

#[derive(Debug, Hash)]
pub struct GLBuffer {
    id: GLuint,

    stride: usize,
    kind: BufferTarget,
    usage: Usage,

    size: usize,
    kind_size: usize,
    length: usize,
}

impl GLBuffer {
    #[inline]
    pub fn new<T>(kind: BufferTarget, stride: usize, usage: Usage, data: &[T]) -> Self {
        let mut id = 0;

        let length = data.len();
        let kind_size = mem::size_of::<T>();
        let size = kind_size * length;
        let gl_kind = kind.into();
        let gl_usage = usage.into();

        unsafe {
            glGenBuffers(1, &mut id);

            glBindBuffer(gl_kind, id);

            glBufferData(
                gl_kind,
                size as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                gl_usage,
            );
        };

        GLBuffer {
            id,

            stride: stride * kind_size,
            kind,
            usage,

            size,
            kind_size,
            length,
        }
    }

    #[inline(always)]
    pub fn id(&self) -> GLuint {
        self.id
    }

    #[inline(always)]
    pub fn stride(&self) -> usize {
        self.stride
    }
    #[inline(always)]
    pub fn kind(&self) -> &BufferTarget {
        &self.kind
    }
    #[inline(always)]
    pub fn usage(&self) -> &Usage {
        &self.usage
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        self.size
    }
    #[inline(always)]
    pub fn kind_size(&self) -> usize {
        self.kind_size
    }
    #[inline(always)]
    pub fn length(&self) -> usize {
        self.length
    }

    #[inline]
    pub fn bind(&self) -> &Self {
        unsafe {
            glBindBuffer(self.kind.into(), self.id);
        }
        self
    }
    #[inline]
    pub fn unbind(&self) -> &Self {
        unsafe {
            glBindBuffer(self.kind.into(), 0);
        }
        self
    }

    #[inline]
    pub fn update<T>(&mut self, data: &[T]) -> &mut Self {
        let length = data.len();
        let kind_size = mem::size_of::<T>();
        let size = kind_size * length;
        let gl_kind = self.kind.into();
        let gl_usage = self.usage.into();

        unsafe {
            glBindBuffer(gl_kind, self.id);

            glBufferData(
                gl_kind,
                size as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                gl_usage,
            );
        };

        self.size = size;
        self.kind_size = kind_size;
        self.length = length;

        self
    }
}

impl Drop for GLBuffer {
    #[inline]
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe {
                glDeleteBuffers(1, &self.id);
            }
        }
    }
}
