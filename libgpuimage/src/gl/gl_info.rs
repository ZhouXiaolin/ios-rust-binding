#![allow(unused_imports)]

use std::sync::atomic::{AtomicUsize, Ordering};

use gles::consts::*;
use gles::es20::ffi::*;
use gles::types::*;

use regex::Regex;

static MAJOR: AtomicUsize = AtomicUsize::new(3);
static MINOR: AtomicUsize = AtomicUsize::new(0);

static GLSL_MAJOR: AtomicUsize = AtomicUsize::new(3);
static GLSL_MINOR: AtomicUsize = AtomicUsize::new(0);

static HIGHP: &'static str = "highp";
static MEDIUMP: &'static str = "mediump";
static LOWP: &'static str = "lowp";

#[inline]
pub fn gl_major() -> usize {
    MAJOR.load(Ordering::Relaxed)
}
#[inline]
pub fn gl_minor() -> usize {
    MINOR.load(Ordering::Relaxed)
}

#[inline]
pub fn glsl_major() -> usize {
    GLSL_MAJOR.load(Ordering::Relaxed)
}
#[inline]
pub fn glsl_minor() -> usize {
    GLSL_MINOR.load(Ordering::Relaxed)
}

#[derive(Debug, Clone, Hash)]
pub struct GLInfo {
    version: String,

    major: usize,
    minor: usize,
    glsl_major: usize,
    glsl_minor: usize,

    extenstions: Vec<String>,

    max_anisotropy: usize,
    max_textures: usize,
    max_vertex_textures: usize,
    max_texture_size: usize,
    max_cube_texture_size: usize,
    max_render_buffer_size: usize,

    max_uniforms: usize,
    max_varyings: usize,
    max_attributes: usize,

    precision: &'static str,
}

impl GLInfo {
    #[inline(always)]
    pub fn new() -> Self {
        let mut info = GLInfo {
            version: String::new(),

            major: 0,
            minor: 0,
            glsl_major: 0,
            glsl_minor: 0,

            extenstions: Vec::new(),

            max_anisotropy: 0,
            max_textures: 0,
            max_vertex_textures: 0,
            max_texture_size: 0,
            max_cube_texture_size: 0,
            max_render_buffer_size: 0,

            max_uniforms: 0,
            max_varyings: 0,
            max_attributes: 0,

            precision: HIGHP,
        };

        info.gl_info();
        info
    }

    #[inline(always)]
    pub fn version(&self) -> &String {
        &self.version
    }

    #[inline(always)]
    pub fn major(&self) -> usize {
        self.major
    }
    #[inline(always)]
    pub fn minor(&self) -> usize {
        self.minor
    }
    #[inline(always)]
    pub fn glsl_major(&self) -> usize {
        self.glsl_major
    }
    #[inline(always)]
    pub fn glsl_minor(&self) -> usize {
        self.glsl_minor
    }
    #[inline(always)]
    #[inline(always)]
    pub fn extenstions(&self) -> &[String] {
        &*self.extenstions
    }

    #[inline(always)]
    pub fn max_anisotropy(&self) -> usize {
        self.max_anisotropy
    }
    #[inline(always)]
    pub fn max_textures(&self) -> usize {
        self.max_textures
    }
    #[inline(always)]
    pub fn max_vertex_textures(&self) -> usize {
        self.max_vertex_textures
    }
    #[inline(always)]
    pub fn max_texture_size(&self) -> usize {
        self.max_texture_size
    }
    #[inline(always)]
    pub fn max_cube_texture_size(&self) -> usize {
        self.max_cube_texture_size
    }
    #[inline(always)]
    pub fn max_render_buffer_size(&self) -> usize {
        self.max_render_buffer_size
    }

    #[inline(always)]
    pub fn max_uniforms(&self) -> usize {
        self.max_uniforms
    }
    #[inline(always)]
    pub fn max_varyings(&self) -> usize {
        self.max_varyings
    }
    #[inline(always)]
    pub fn max_attributes(&self) -> usize {
        self.max_attributes
    }

    #[inline(always)]
    pub fn precision(&self) -> &'static str {
        self.precision
    }

    #[inline(always)]
    pub fn has_extenstion(&self, string: &str) -> bool {
        match self.extenstions.iter().position(|e| e == string) {
            Some(_) => true,
            None => false,
        }
    }

    #[inline]
    fn gl_info(&mut self) {
        //        let (vs_high_float_precision, vs_high_float_range) = get_shader_precision_format(
        //            GL_VERTEX_SHADER,
        //            GL_HIGH_FLOAT);
        let mut vs_high_float_precision: GLint = 0;
        let mut vs_high_float_range: GLint = 0;
        unsafe {
            glGetShaderPrecisionFormat(
                GL_VERTEX_SHADER,
                GL_HIGH_FLOAT,
                &mut vs_high_float_range,
                &mut vs_high_float_precision,
            );
        }

        //        let (vs_mediump_float_precision, vs_mediump_float_range) = get_shader_precision_format(
        //            GL_VERTEX_SHADER,
        //            GL_MEDIUM_FLOAT);

        let mut vs_mediump_float_precision: GLint = 0;
        let mut vs_mediump_float_range: GLint = 0;
        unsafe {
            glGetShaderPrecisionFormat(
                GL_VERTEX_SHADER,
                GL_MEDIUM_FLOAT,
                &mut vs_mediump_float_range,
                &mut vs_mediump_float_precision,
            );
        }

        //        let (fs_high_float_precision, fs_high_float_range) = get_shader_precision_format(
        //            GL_FRAGMENT_SHADER,
        //            GL_HIGH_FLOAT);
        let mut fs_high_float_precision: GLint = 0;
        let mut fs_high_float_range: GLint = 0;
        unsafe {
            glGetShaderPrecisionFormat(
                GL_FRAGMENT_SHADER,
                GL_HIGH_FLOAT,
                &mut fs_high_float_range,
                &mut fs_high_float_precision,
            );
        }

        //        let (fs_mediump_float_precision, fs_mediump_float_range) = get_shader_precision_format(
        //            GL_FRAGMENT_SHADER,
        //            GL_MEDIUM_FLOAT);
        let mut fs_mediump_float_precision: GLint = 0;
        let mut fs_mediump_float_range: GLint = 0;
        unsafe {
            glGetShaderPrecisionFormat(
                GL_FRAGMENT_SHADER,
                GL_MEDIUM_FLOAT,
                &mut fs_mediump_float_range,
                &mut fs_mediump_float_precision,
            );
        }

        let highp_available = vs_high_float_precision > 0 && fs_high_float_precision > 0;
        let mediump_available = vs_mediump_float_precision > 0 && fs_mediump_float_precision > 0;

        self.precision = if !highp_available {
            if mediump_available {
                MEDIUMP
            } else {
                LOWP
            }
        } else {
            HIGHP
        };

        unsafe {
            let ptr = glGetString(GL_VERSION);
            string_from_ptr(ptr, &mut self.version);

            let (mut major, mut minor) = match Regex::new(r"(\d+).(\d+)")
                .expect("regex failed to compile")
                .captures(&self.version)
            {
                Some(cap) => (
                    match cap.get(1) {
                        Some(major) => major.as_str().parse::<i32>().unwrap(),
                        None => 3,
                    },
                    match cap.get(2) {
                        Some(minor) => minor.as_str().parse::<i32>().unwrap(),
                        None => 1,
                    },
                ),
                None => (3, 1),
            };

            if major > 2 {
                glGetIntegerv(GL_MAJOR_VERSION, &mut major);
                self.major = major as usize;
                glGetIntegerv(GL_MINOR_VERSION, &mut minor);
                self.minor = minor as usize;
            } else {
                self.major = 2;
                self.minor = 0;
            }

            glsl_version(
                self.major,
                self.minor,
                &mut self.glsl_major,
                &mut self.glsl_minor,
            );
            parse_extenstions(&mut self.extenstions, self.major);
        }

        MAJOR.store(self.major, Ordering::SeqCst);
        MINOR.store(self.minor, Ordering::SeqCst);

        GLSL_MAJOR.store(self.glsl_major, Ordering::SeqCst);
        GLSL_MINOR.store(self.glsl_minor, Ordering::SeqCst);

        unsafe {
            let mut max_textures = 0;
            glGetIntegerv(GL_MAX_TEXTURE_IMAGE_UNITS, &mut max_textures);
            self.max_textures = max_textures as usize;

            let mut max_vertex_textures = 0;
            glGetIntegerv(GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS, &mut max_vertex_textures);
            self.max_vertex_textures = max_vertex_textures as usize;

            let mut max_texture_size = 0;
            glGetIntegerv(GL_MAX_TEXTURE_SIZE, &mut max_texture_size);
            self.max_texture_size = max_texture_size as usize;

            let mut max_cube_texture_size = 0;
            glGetIntegerv(GL_MAX_CUBE_MAP_TEXTURE_SIZE, &mut max_cube_texture_size);
            self.max_cube_texture_size = max_cube_texture_size as usize;

            let mut max_render_buffer_size = 0;
            glGetIntegerv(GL_MAX_RENDERBUFFER_SIZE, &mut max_render_buffer_size);
            self.max_render_buffer_size = max_render_buffer_size as usize;

            let mut vs_max_uniforms = 0;
            let mut fs_max_uniforms = 0;
            glGetIntegerv(GL_MAX_VERTEX_UNIFORM_VECTORS, &mut vs_max_uniforms);
            glGetIntegerv(GL_MAX_FRAGMENT_UNIFORM_VECTORS, &mut fs_max_uniforms);
            self.max_uniforms = if vs_max_uniforms < fs_max_uniforms {
                vs_max_uniforms
            } else {
                fs_max_uniforms
            } as usize * 4;

            let mut max_varyings = 0;
            glGetIntegerv(GL_MAX_VARYING_VECTORS, &mut max_varyings);
            self.max_varyings = max_varyings as usize * 4;

            let mut max_attributes = 0;
            glGetIntegerv(GL_MAX_VERTEX_ATTRIBS, &mut max_attributes);
            self.max_attributes = max_attributes as usize;
        }
    }
}

#[inline]
unsafe fn string_from_ptr(ptr: *const u8, string: &mut String) {
    let mut i = 0isize;
    loop {
        let ch = *ptr.offset(i);

        if ch != 0u8 {
            string.push(ch as char);
            i = i + 1isize;
        } else {
            break;
        }
    }
}

#[inline]
unsafe fn parse_extenstions(extenstions: &mut Vec<String>, major_version: usize) {
    //    if major_version > 2 {
    //        let mut count = 0;
    //        glGetIntegerv(GL_NUM_EXTENSIONS, &mut count);
    //
    //        for i in 0..(count as u32) {
    //            let mut string = String::new();
    //            string_from_ptr(glGetStringi(GL_EXTENSIONS, i), &mut string);
    //            extenstions.push(string);
    //        }
    //    } else {
    let mut string = String::new();
    string_from_ptr(glGetString(GL_EXTENSIONS), &mut string);

    for extenstion in string.split_whitespace() {
        extenstions.push(String::from(extenstion));
    }
    //    }
}

#[inline]
fn glsl_version(major: usize, minor: usize, glsl_major: &mut usize, glsl_minor: &mut usize) {
    if major <= 3 && minor <= 2 {
        *glsl_major = 1;
        *glsl_minor = if major == 3 && minor == 2 {
            5
        } else if major == 3 && minor == 1 {
            4
        } else if major == 3 && minor == 0 {
            3
        } else if major == 2 && minor == 1 {
            2
        } else {
            1
        }
    } else {
        *glsl_major = major;
        *glsl_minor = minor;
    }
}
