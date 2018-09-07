#![allow(unused_imports)]

use gles::es20::ffi::*;
use gles::consts::*;
use gles::types::*;

use super::{Attachment, InternalFormat};

#[derive(Debug, Hash)]
pub struct GLRenderbuffer {
    id: GLuint,
}

impl GLRenderbuffer {
    #[inline(always)]
    pub fn new(
        internal_format: InternalFormat,
        attachment: Attachment,
        width: usize,
        height: usize,
    ) -> Self {
        let renderbuffer = GLRenderbuffer {
            id: {
                let mut id = 0;
                unsafe {
                    glGenRenderbuffers(1, &mut id);
                }
                id
            },
        };

        renderbuffer.set(internal_format, attachment, width, height);
        renderbuffer
    }

    #[inline(always)]
    pub fn id(&self) -> GLuint {
        self.id
    }

    #[inline]
    pub fn bind(&self) -> &Self {
        unsafe {
            glBindRenderbuffer(GL_RENDERBUFFER, self.id);
        }
        self
    }

    #[inline]
    pub fn unbind(&self) -> &Self {
        unsafe {
            glBindRenderbuffer(GL_RENDERBUFFER, 0);
        }
        self
    }

    #[inline]
    pub fn set(
        &self,
        internal_format: InternalFormat,
        attachment: Attachment,
        width: usize,
        height: usize,
    ) {
        let internal_format: GLenum = internal_format.into();
        let attachment: GLenum = attachment.into();

        unsafe {
            glBindRenderbuffer(GL_RENDERBUFFER, self.id);
            glRenderbufferStorage(
                GL_RENDERBUFFER,
                internal_format,
                width as GLint,
                height as GLint,
            );
            glFramebufferRenderbuffer(GL_FRAMEBUFFER, attachment, GL_RENDERBUFFER, self.id);
        }
    }
}

impl Drop for GLRenderbuffer {
    #[inline]
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe {
                glDeleteRenderbuffers(1, &self.id);
            }
        }
    }
}
