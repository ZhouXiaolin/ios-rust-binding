#![allow(unused_imports)]

use std::mem;

use gles::consts::*;
use gles::es20::ffi::*;
use gles::types::*;

use super::DataKind;

#[derive(Debug, Hash)]
pub struct GLVertex {
    location: usize,
    item_count: usize,
    kind: DataKind,
    stride: usize,
    offset: usize,
}

impl GLVertex {
    #[inline(always)]
    pub fn new(
        location: usize,
        item_count: usize,
        kind: DataKind,
        stride: usize,
        offset: usize,
    ) -> Self {
        GLVertex {
            location,
            item_count,
            kind,
            stride,
            offset,
        }
    }

    #[inline(always)]
    pub fn location(&self) -> usize {
        self.location
    }
    #[inline(always)]
    pub fn item_count(&self) -> usize {
        self.item_count
    }
    #[inline(always)]
    pub fn kind(&self) -> DataKind {
        self.kind
    }
    #[inline(always)]
    pub fn stride(&self) -> usize {
        self.stride
    }
    #[inline(always)]
    pub fn offset(&self) -> usize {
        self.offset
    }

    #[inline(always)]
    pub fn enable(&self) -> &Self {
        unsafe {
            glEnableVertexAttribArray(self.location() as GLuint);
        }
        self
    }
    #[inline(always)]
    pub fn disable(&self) -> &Self {
        unsafe {
            glDisableVertexAttribArray(self.location() as GLuint);
        }
        self
    }

    #[inline]
    pub fn bind(&self) -> &Self {
        unsafe {
            glVertexAttribPointer(
                self.location as GLuint,
                self.item_count as GLint,
                self.kind.into(),
                GL_FALSE,
                self.stride as GLsizei,
                mem::transmute(self.offset),
            );
        }
        self
    }
}
