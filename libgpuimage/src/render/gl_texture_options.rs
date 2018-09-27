use gles_rust_binding::*;


#[derive(Debug, Copy, Clone)]
pub struct GPUTextureOptions {
    pub minFilter : GLenum,
    pub magFilter : GLenum,
    pub wrapS : GLenum,
    pub wrapT : GLenum,
    pub internalFormat : GLenum,
    pub format : GLenum,
    pub _type : GLenum
}

impl Default for GPUTextureOptions {
    fn default() -> Self {
        GPUTextureOptions {
            minFilter: GL_LINEAR,
            magFilter: GL_LINEAR,
            wrapS: GL_CLAMP_TO_EDGE,
            wrapT: GL_CLAMP_TO_EDGE,
            internalFormat: GL_RGBA,
            format: GL_BGRA,
            _type: GL_UNSIGNED_BYTE
        }
    }
}