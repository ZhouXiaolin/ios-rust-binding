#![allow(unused_imports)]

use std::ptr;
use std::str::{self, Utf8Error};

use fnv::FnvHashMap;

use gles::consts::*;
use gles::es20::ffi::*;
use gles::types::*;

use regex::Regex;

use super::{GLAttribute, GLUniform};

#[derive(Debug)]
pub struct GLProgram {
    id: GLuint,
    uniforms: FnvHashMap<String, GLUniform>,
    attributes: FnvHashMap<String, GLAttribute>,
}

impl GLProgram {
    #[inline]
    pub fn new(vertex: &str, fragment: &str) -> Self {
        let mut program = GLProgram {
            id: 0,
            uniforms: FnvHashMap::default(),
            attributes: FnvHashMap::default(),
        };

        program.set(vertex, fragment);


        program
    }

    #[inline]
    pub fn new_mutiple(vertex: &[&str], fragment: &[&str]) -> Self {
        let mut program = GLProgram {
            id: 0,
            uniforms: FnvHashMap::default(),
            attributes: FnvHashMap::default(),
        };
        program.set_mutiple(vertex, fragment);
        program
    }

    #[inline(always)]
    pub fn id(&self) -> GLuint {
        self.id
    }

    #[inline]
    pub fn bind(&self) -> &Self {
        unsafe {
            glUseProgram(self.id);
        }
        self
    }

    #[inline]
    pub fn unbind(&self) -> &Self {
        unsafe {
            glUseProgram(0);
        }

        self
    }

    #[inline(always)]
    pub fn has_uniform(&self, name: &str) -> bool {
        self.uniforms.contains_key(name)
    }

    #[inline(always)]
    pub fn get_uniform(&self, name: &str) -> &GLUniform {
        match self.uniforms.get(name) {
            Some(ref uniform) => uniform,
            None => {

                panic!("No uniform named {:?} found", name)
            },
        }
    }

    #[inline(always)]
    pub fn uniforms(&self) -> &FnvHashMap<String, GLUniform> {
        &self.uniforms
    }

    #[inline(always)]
    pub fn uniforms_mut(&mut self) -> &mut FnvHashMap<String, GLUniform> {
        &mut self.uniforms
    }

    #[inline(always)]
    pub fn has_attribute(&self, name: &str) -> bool {
        self.attributes.contains_key(name)
    }

    #[inline(always)]
    pub fn get_attribute(&self, name: &str) ->Option<&GLAttribute> {
        let attributes = self.attributes.get(name);
        attributes
//        match self.attributes.get(name) {
//            Some(ref attribute) => attribute,
//            None => panic!("No attribute named {:?} found", name),
//        }
    }

    #[inline(always)]
    pub fn attributes(&self) -> &FnvHashMap<String, GLAttribute> {
        &self.attributes
    }

    #[inline(always)]
    pub fn attributes_mut(&mut self) -> &mut FnvHashMap<String, GLAttribute> {
        &mut self.attributes
    }

    #[inline]
    pub fn set(&mut self, vertex: &str, fragment: &str) -> &mut Self {

        let vs = compile_shader(vertex, GL_VERTEX_SHADER);

        let fs = compile_shader(fragment, GL_FRAGMENT_SHADER);

        let id = link_program(vs, fs);

        self.set_program_id(id)
    }

    #[inline]
    pub fn set_mutiple(&mut self, vertex: &[&str], fragment: &[&str]) -> &mut Self {
        let vs = compile_shaders(vertex, GL_VERTEX_SHADER);
        let fs = compile_shaders(fragment, GL_FRAGMENT_SHADER);
        let id = link_program(vs, fs);
        self.set_program_id(id)
    }

    #[inline]
    fn set_program_id(&mut self, id: GLuint) -> &mut Self {
        {
            let ref mut uniforms = self.uniforms;
            let ref mut attributes = self.attributes;

            if self.id != 0 {
                uniforms.clear();
                attributes.clear();
                unsafe {
                    glDeleteProgram(self.id);
                }
            }

            self.id = id;

            parse_attributes(id, attributes);
            parse_uniforms(id, uniforms);


        }
        self
    }
}

impl Drop for GLProgram {
    #[inline]
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe {
                glDeleteProgram(self.id);
            }
        }
    }
}

#[inline]
fn parse_uniforms(program: GLuint, uniforms: &mut FnvHashMap<String, GLUniform>) {
    let mut max_length = 0;
    let mut active_length = 0;

    unsafe {
        glGetProgramiv(program, GL_ACTIVE_UNIFORM_MAX_LENGTH, &mut max_length);
        glGetProgramiv(program, GL_ACTIVE_UNIFORMS, &mut active_length);
    }


    for i in 0..active_length {
        let mut length = 0;
        let mut count = 0;
        let mut kind = 0;

        let mut buf = Vec::with_capacity(max_length as usize);
        let location;

        unsafe {
            let buf_ptr = buf.as_mut_ptr() as *mut GLchar;
            glGetActiveUniform(
                program,
                i as GLuint,
                max_length,
                &mut length,
                &mut count,
                &mut kind,
                buf_ptr,
            );
            buf.set_len(length as usize);

            location = glGetUniformLocation(program, buf_ptr);
        };

        let mut name = match string_from_utf8(&buf) {
            Ok(string) => {
                string
            },
            Err(vec) => {

                panic!("Could not convert uniform name from buffer: {:?}", vec)
            },
        };

        if name
            .chars()
            .nth(name.len() - 1)
            .expect("Unexpected empty uniform name") == ']'
        {
            let new_name = match Regex::new(r"(.*)\[\d+\]")
                .expect("regex failed to compile")
                .captures(&name)
            {
                Some(cap) => match cap.get(1) {
                    Some(name) => Some(String::from(name.as_str())),
                    None => None,
                },
                None => None,
            };

            if let Some(value) = new_name {
                name = value;
            };
        };

        let u = GLUniform::new(kind.into(), count as usize, location as usize);

        uniforms.insert(
            name,
            u
        );

    }
}

#[inline]
fn string_from_utf8(vec: &Vec<u8>) -> Result<String, Utf8Error> {
    match str::from_utf8(vec) {
        Ok(s) => Ok(String::from(s)),
        Err(e) => Err(e),
    }
}

#[inline]
fn parse_attributes(program: GLuint, attributes: &mut FnvHashMap<String, GLAttribute>) {
    let mut max_length = 0;
    let mut active_length = 0;

    unsafe {
        glGetProgramiv(program, GL_ACTIVE_ATTRIBUTE_MAX_LENGTH, &mut max_length);
        glGetProgramiv(program, GL_ACTIVE_ATTRIBUTES, &mut active_length);
    }

    for i in 0..active_length {
        let mut length = 0;
        let mut count = 0;
        let mut kind = 0;

        let mut buf = Vec::with_capacity(max_length as usize);
        let buf_ptr = buf.as_mut_ptr() as *mut GLchar;
        let location;

        unsafe {
            glGetActiveAttrib(
                program,
                i as u32,
                max_length,
                &mut length,
                &mut count,
                &mut kind,
                buf_ptr,
            );
            buf.set_len(length as usize);
            location = glGetAttribLocation(program, buf_ptr);
        }

        let name = match string_from_utf8(&buf) {
            Ok(string) => string,
            Err(vec) => {
                info!("Could not convert attribute name from buffer: {:?}", vec);

                panic!("Could not convert attribute name from buffer: {:?}", vec)
            },
        };

        attributes.insert(
            name,
            GLAttribute::new(kind.into(), count as usize, location as usize),
        );
    }
}

#[inline]
pub fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    let mut program = 0;

    unsafe {
        program = glCreateProgram();

        glAttachShader(program, vertex_shader);
        glDeleteShader(vertex_shader);

        glAttachShader(program, fragment_shader);
        glDeleteShader(fragment_shader);

        glLinkProgram(program);
        glValidateProgram(program);
        glUseProgram(program);
    }
    check_program_status(program)
}

#[inline]
pub fn check_program_status(program: GLuint) -> GLuint {
    let mut status = 0;

    unsafe { glGetProgramiv(program, GL_LINK_STATUS, &mut status) };

    if status != (GL_TRUE as GLint) {
        let mut len: GLint = 0;
        unsafe {
            glGetProgramiv(program, GL_INFO_LOG_LENGTH, &mut len);
        }
        let mut buf = Vec::with_capacity(len as usize);
        unsafe {
            buf.set_len(len as usize);
            glGetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
        }

        panic!(
            "{}",
            str::from_utf8(&buf)
                .ok()
                .expect("ProgramInfoLog not valid utf8")
        );
    }
    program
}

#[inline]
pub fn compile_shader(source: &str, kind: GLenum) -> GLuint {
    let shader = unsafe { glCreateShader(kind) };

    unsafe {
        let ptr: *const GLchar = source.as_bytes().as_ptr() as *const GLchar;

        let len = source.len() as GLint;

        glShaderSource(shader, 1, &ptr, &len);

        glCompileShader(shader);

    }
    check_shader_status(shader)

}

#[inline]
pub fn compile_shaders(sources: &[&str], kind: GLenum) -> GLuint {
    let shader = unsafe { glCreateShader(kind) };

    unsafe {
        for source in sources.iter() {
            let ptr: *const GLchar = source.as_bytes().as_ptr() as *const GLchar;
            let len = source.len() as GLint;
            glShaderSource(shader, 1, &ptr, &len);
            glCompileShader(shader);
        }
    }
    check_shader_status(shader)
}

#[inline]
pub fn check_shader_status(shader: GLuint) -> GLuint {
    let mut status = 0;
    unsafe { glGetShaderiv(shader, GL_COMPILE_STATUS, &mut status) };

    if status != (GL_TRUE as GLint) {
        let mut len = 0;
        unsafe {
            glGetShaderiv(shader, GL_INFO_LOG_LENGTH, &mut len);
        }

        let mut buf = Vec::with_capacity(len as usize);
        unsafe {
            buf.set_len(len as usize);
            glGetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
        }

        let uf = str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8");


        panic!(
            "{}",
            str::from_utf8(&buf)
                .ok()
                .expect("ShaderInfoLog not valid utf8")
        );
    }
    shader
}
