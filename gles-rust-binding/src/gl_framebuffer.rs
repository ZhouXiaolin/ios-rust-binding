#![allow(unused_imports,)]

use gles::consts::*;
use gles::es20::ffi::*;
use gles::types::*;

use super::{Attachment, GLTexture};

#[derive(Debug, Hash)]
pub struct GLFramebuffer {
    id: GLuint,
}

impl GLFramebuffer {
    #[inline(always)]
    pub fn new(gl_texture: &GLTexture, buffers: &[Attachment], level: isize) -> Self {
        let framebuffer = GLFramebuffer {
            id: {
                let mut id = 0;
                unsafe {
                    glGenFramebuffers(1, &mut id);
                }
                id
            },
        };

        framebuffer.set(gl_texture, buffers, level);
        framebuffer
    }

    #[inline(always)]
    pub fn id(&self) -> GLuint {
        self.id
    }

    #[inline]
    pub fn bind(&self) -> &Self {
        unsafe {
            glBindFramebuffer(GL_FRAMEBUFFER, self.id);
        }
        self
    }
    #[inline]
    pub fn unbind(&self) -> &Self {
        unsafe {
            glBindFramebuffer(GL_FRAMEBUFFER, 0);
        }
        self
    }

    #[inline]
    pub fn set(&self, gl_texture: &GLTexture, buffers: &[Attachment], level: isize) {
        let gl_texture_id = gl_texture.id();

        let mut gl_enums = Vec::with_capacity(buffers.len());
        for i in 0..buffers.len() {
            gl_enums.push(buffers[i].into());
        }

        unsafe {
            glBindFramebuffer(GL_FRAMEBUFFER, self.id);

            glBindTexture(gl_texture.kind().into(), gl_texture_id);

            for i in 0..gl_enums.len() {
                glFramebufferTexture2D(
                    GL_FRAMEBUFFER,
                    gl_enums[i],
                    GL_TEXTURE_2D,
                    gl_texture_id,
                    level as GLint,
                );
            }
            //            glDrawBuffers(buffers.len() as GLint, gl_enums.as_ptr()); // es 3.0

            match glCheckFramebufferStatus(GL_FRAMEBUFFER) {
                GL_FRAMEBUFFER_UNDEFINED => panic!("Check framebuffer status failed undefined"),
                GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT => {
                    panic!("Check framebuffer status failed incomplete attachment")
                }
                GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {
                    panic!("Check framebuffer status failed incomplete missing attachment")
                }
                GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => {
                    panic!("Check framebuffer status failed incomplete draw buffer")
                }
                GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER => {
                    panic!("Check framebuffer status failed incomplete read buffer")
                }
                GL_FRAMEBUFFER_UNSUPPORTED => panic!("Check framebuffer status failed unsupported"),
                GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => {
                    panic!("Check framebuffer status failed incomplete multisample")
                }
                GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => {
                    panic!("Check framebuffer status failed incomplete layer targets")
                }
                _ => (),
            }
        }
    }
}

impl Drop for GLFramebuffer {
    #[inline]
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe {
                glDeleteFramebuffers(1, &self.id);
            }
        }
    }
}
