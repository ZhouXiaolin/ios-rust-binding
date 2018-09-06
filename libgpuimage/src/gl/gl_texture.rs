#![allow(unused_imports)]

use std::{mem, ptr};

use gles::es20::ffi::*;
use gles::consts::*;
use gles::types::*;

use super::{DataFormat, DataKind, FilterMode, InternalFormat, TextureKind, Wrap};

#[derive(Debug, Hash)]
pub struct GLTexture {
    id: GLuint,
    kind: TextureKind,
    width: usize,
    height: usize,
    internal_format: InternalFormat,
    format: DataFormat,
    data_kind: DataKind,
    filter: FilterMode,
    wrap: Wrap,
    mipmap: bool,
}

impl GLTexture {
    #[inline(always)]
    pub fn new_2d<T>(
        width: usize,
        height: usize,
        internal_format: InternalFormat,
        format: DataFormat,
        data_kind: DataKind,
        filter: FilterMode,
        wrap: Wrap,
        generate_mipmap: bool,
        data: &[T],
    ) -> Self {
        let id = new_gl_texture();
        let mipmap = unsafe {
            glBindTexture(GL_TEXTURE_2D, id);
            Self::gl_2d(width, height, internal_format, format, data_kind, data);
            Self::gl_wrap_filter_mipmap(width, height, filter, wrap, generate_mipmap)
        };

        GLTexture {
            id: id,
            kind: TextureKind::Texture2D,
            width: width,
            height: height,
            internal_format: internal_format,
            format: format,
            data_kind: data_kind,
            filter: filter,
            wrap: wrap,
            mipmap: mipmap,
        }
    }

    #[inline(always)]
    pub fn new_null_2d(
        width: usize,
        height: usize,
        internal_format: InternalFormat,
        format: DataFormat,
        data_kind: DataKind,
        filter: FilterMode,
        wrap: Wrap,
        generate_mipmap: bool,
    ) -> Self {
        let id = new_gl_texture();
        let mipmap = unsafe {
            glBindTexture(GL_TEXTURE_2D, id);
            Self::gl_null_2d(width, height, internal_format, format, data_kind);
            Self::gl_wrap_filter_mipmap(width, height, filter, wrap, generate_mipmap)
        };

        GLTexture {
            id: id,
            kind: TextureKind::Texture2D,
            width: width,
            height: height,
            internal_format: internal_format,
            format: format,
            data_kind: data_kind,
            filter: filter,
            wrap: wrap,
            mipmap: mipmap,
        }
    }

    #[inline(always)]
    pub fn resize_null_2d(&mut self, width: usize, height: usize) -> &Self {
        self.width = width;
        self.height = height;

        unsafe {
            glBindTexture(GL_TEXTURE_2D, self.id);
            Self::gl_null_2d(
                self.width,
                self.height,
                self.internal_format,
                self.format,
                self.data_kind,
            );
            Self::gl_wrap_filter_mipmap(
                self.width,
                self.height,
                self.filter,
                self.wrap,
                self.mipmap,
            );
        }
        self
    }

    #[inline(always)]
    pub fn id(&self) -> GLuint {
        self.id
    }
    #[inline(always)]
    pub fn kind(&self) -> TextureKind {
        self.kind
    }
    #[inline(always)]
    pub fn width(&self) -> usize {
        self.width
    }
    #[inline(always)]
    pub fn height(&self) -> usize {
        self.height
    }
    #[inline(always)]
    pub fn data_kind(&self) -> DataKind {
        self.data_kind
    }
    #[inline(always)]
    pub fn format(&self) -> DataFormat {
        self.format
    }
    #[inline(always)]
    pub fn filter(&self) -> FilterMode {
        self.filter
    }
    #[inline(always)]
    pub fn wrap(&self) -> Wrap {
        self.wrap
    }
    #[inline(always)]
    pub fn mipmap(&self) -> bool {
        self.mipmap
    }

    #[inline]
    pub fn bind(&self) -> &Self {
        unsafe {
            glBindTexture(self.kind.into(), self.id);
        }
        self
    }
    #[inline]
    pub fn unbind(&self) -> &Self {
        unsafe {
            glBindTexture(self.kind.into(), 0);
        }
        self
    }

    #[inline]
    pub fn gl_wrap_filter_mipmap(
        width: usize,
        height: usize,
        filter: FilterMode,
        wrap: Wrap,
        generate_mipmap: bool,
    ) -> bool {
        let is_power_of_two = width.is_power_of_two() && height.is_power_of_two();
        let (mag_filter, min_filter) =
            Self::mag_filter_min_filter(filter, is_power_of_two, generate_mipmap);

        unsafe {
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, mag_filter as GLint);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, min_filter as GLint);

            let gl_wrap = GLenum::from(wrap) as GLint;
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, gl_wrap);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, gl_wrap);

            if generate_mipmap && is_power_of_two {
                glGenerateMipmap(GL_TEXTURE_2D);
            }
        }

        generate_mipmap && is_power_of_two
    }

    #[inline]
    pub fn gl_2d<T>(
        width: usize,
        height: usize,
        internal_format: InternalFormat,
        format: DataFormat,
        data_kind: DataKind,
        data: &[T],
    ) {
        unsafe {
            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                GLenum::from(internal_format) as GLint,
                width as GLsizei,
                height as GLsizei,
                0,
                format.into(),
                data_kind.into(),
                mem::transmute(data.as_ptr()),
            );
        }
    }

    #[inline]
    pub fn gl_null_2d(
        width: usize,
        height: usize,
        internal_format: InternalFormat,
        format: DataFormat,
        data_kind: DataKind,
    ) {
        unsafe {
            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                GLenum::from(internal_format) as GLint,
                width as GLsizei,
                height as GLsizei,
                0,
                format.into(),
                data_kind.into(),
                ptr::null(),
            );
        }
    }

    // (mag_filter, min_filter)
    #[inline]
    pub fn mag_filter_min_filter(
        filter: FilterMode,
        is_power_of_two: bool,
        generate_mipmap: bool,
    ) -> (GLuint, GLuint) {
        match filter {
            FilterMode::None => (
                GL_NEAREST,
                if is_power_of_two && generate_mipmap {
                    GL_LINEAR_MIPMAP_NEAREST
                } else {
                    GL_NEAREST
                },
            ),
            _ => (
                GL_LINEAR,
                if is_power_of_two && generate_mipmap {
                    GL_LINEAR_MIPMAP_LINEAR
                } else {
                    GL_LINEAR
                },
            ),
        }
    }
}

impl Drop for GLTexture {
    #[inline]
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe {
                glDeleteTextures(1, &self.id);
            }
        }
    }
}

#[inline(always)]
fn new_gl_texture() -> GLuint {
    let mut id = 0;
    unsafe {
        glGenTextures(1, &mut id);
    }
    id
}
