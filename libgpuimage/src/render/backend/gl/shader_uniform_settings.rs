use super::{Color,Position,Size, Matrix3x3, Matrix4x4};
use fnv::FnvHashMap;
use gles_rust_binding::*;

#[derive(Debug,Clone)]
pub enum Uniform{
    Float(f32),
    Int(i32),
    Color(Color),
    Position(Position),
    Size(Size),
    Matrix4x4(Matrix4x4),
    Matrix3x3(Matrix3x3)
}
#[derive(Default,Debug)]
pub struct ShaderUniformSettings{
    pub uniformValues: FnvHashMap<String,Uniform>
}

impl ShaderUniformSettings {

    pub fn setValue(&mut self, key:&str, value:Uniform) {


        let uniformValues = &mut self.uniformValues;

        let key = String::from(key);


        uniformValues.insert(key,value);

    }

    pub fn restoreShaderSettings(&self, shader: &GLProgram){
        for (key,value) in self.uniformValues.iter() {
            let uniform = shader.get_uniform(key);
            match value {
                Uniform::Float(f) => {
                    unsafe {glUniform1f(uniform.location() as i32, f.clone())};
                },
                Uniform::Int(i) => {

                },
                Uniform::Color(c) => {

                },
                Uniform::Position(p) => {

                },
                Uniform::Size(s) => {

                },
                Uniform::Matrix3x3(m) => {
                    unsafe {glUniformMatrix3fv(uniform.location() as i32, 1, GL_FALSE, m.toRowMajorGLArray().as_ptr())};
                },
                Uniform::Matrix4x4(m) => {
                    unsafe {glUniformMatrix4fv(uniform.location() as i32, 1, GL_FALSE, m.toRowMajorGLArray().as_ptr())};
                }
            }
        }
    }
}
