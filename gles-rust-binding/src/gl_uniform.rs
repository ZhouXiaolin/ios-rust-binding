#![allow(unused_imports)]

use gles::es20::ffi::*;
use gles::consts::*;
use gles::types::*;

use super::{GLTexture, UniformKind};

#[derive(Debug, Clone, Hash)]
pub struct GLUniform {
    kind: UniformKind,
    count: usize,
    location: usize,
}

impl GLUniform {
    #[inline(always)]
    pub fn new(kind: UniformKind, count: usize, location: usize) -> Self {
        GLUniform {
            kind: kind,
            count: count,
            location: location,
        }
    }

    #[inline(always)]
    pub fn kind(&self) -> &UniformKind {
        &self.kind
    }
    #[inline(always)]
    pub fn count(&self) -> usize {
        self.count
    }
    #[inline(always)]
    pub fn location(&self) -> usize {
        self.location
    }
}

macro_rules! create_set_uniform {
    ($name: ident, $func: ident, $kind: ident) => (
        #[inline]
        pub fn $name(&self, value: $kind) {
            unsafe {
                $func(self.location as GLint, value);
                // 想调用这个函数  pub fn glUniform1f(location: GLint, x: GLfloat);
                // 这是使用宏的语句 create_set_uniform!(set_1f, Uniform1f, f32);
                // 编译报错 error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `Uniform1i`
            }
        }
    )
}

macro_rules! create_set_vec_uniform {
    ($name: ident, $func: ident, $kind: ident, $item_count: expr) => (
        #[inline]
        pub fn $name(&self, value: &[$kind; $item_count]) {
            unsafe {
                $func(self.location as GLint, 1 as GLint, value.as_ptr());
            }
        }
    )
}

macro_rules! create_set_matrix_uniform {
    ($name: ident, $func: ident, $kind: ident, $item_count: expr) => (
        #[inline]
        pub fn $name(&self, value: &[$kind; $item_count]) {
            unsafe {
                $func(self.location as GLint, 1 as GLint, GL_FALSE, value.as_ptr());
            }
        }
    )
}

impl GLUniform {
    create_set_uniform!(set_1f, glUniform1f, f32);
    create_set_uniform!(set_1i, glUniform1i, i32);

    create_set_vec_uniform!(set_2f, glUniform2fv, f32, 2);
    create_set_vec_uniform!(set_2i, glUniform2iv, i32, 2);

    create_set_vec_uniform!(set_3f, glUniform3fv, f32, 3);
    create_set_vec_uniform!(set_3i, glUniform3iv, i32, 3);

    create_set_vec_uniform!(set_4f, glUniform4fv, f32, 4);
    create_set_vec_uniform!(set_4i, glUniform4iv, i32, 4);

    create_set_matrix_uniform!(set_mat2f, glUniformMatrix2fv, f32, 4);
    create_set_matrix_uniform!(set_mat3f, glUniformMatrix3fv, f32, 9);
    create_set_matrix_uniform!(set_mat4f, glUniformMatrix4fv, f32, 16);
}

macro_rules! create_set_vec_uniform_size {
    ($name: ident, $func: ident, $kind: ident) => (
        #[inline]
        pub fn $name(&self, values: &[$kind]) {
            unsafe {
                $func(self.location as GLint, self.count as GLint, values.as_ptr());
            }
        }
    )
}

macro_rules! create_set_matrix_uniform_size {
    ($name: ident, $func: ident, $kind: ident) => (
        #[inline]
        pub fn $name(&self, values: &[$kind]) {
            unsafe {
                $func(self.location as GLint, self.count as GLint, GL_FALSE, values.as_ptr());
            }
        }
    )
}

impl GLUniform {
    create_set_vec_uniform_size!(set_2fv, glUniform2fv, f32);
    create_set_vec_uniform_size!(set_2iv, glUniform2iv, i32);

    create_set_vec_uniform_size!(set_3fv, glUniform3fv, f32);
    create_set_vec_uniform_size!(set_3iv, glUniform3iv, i32);

    create_set_vec_uniform_size!(set_4fv, glUniform4fv, f32);
    create_set_vec_uniform_size!(set_4iv, glUniform4iv, i32);

    create_set_matrix_uniform_size!(set_mat2fv, glUniformMatrix2fv, f32);
    create_set_matrix_uniform_size!(set_mat3fv, glUniformMatrix3fv, f32);
    create_set_matrix_uniform_size!(set_mat4fv, glUniformMatrix4fv, f32);

    #[inline]
    pub fn set_sampler_2d(&self, texture: &GLTexture, index: usize) {
        unsafe {
            glActiveTexture(GL_TEXTURE0 + index as GLuint);
            glUniform1i(self.location as GLint, index as GLint);
            glBindTexture(texture.kind().into(), texture.id());
        }
    }
}
