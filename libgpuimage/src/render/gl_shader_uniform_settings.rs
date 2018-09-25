use super::{Color,Position,Size};
use fnv::FnvHashMap;
use gles_rust_binding::*;
pub enum Uniform{
    Float(f32),
    Int(i32),
    Color(Color),
    Position(Position),
    Size(Size),
    Matrix4x4(),
    Matrix3x3()
}
#[derive(Default)]
pub struct ShaderUniformSettings{
    uniformValues: FnvHashMap<String,Uniform>
}

impl ShaderUniformSettings {

    pub fn setValue(&mut self, key:&str, value:Uniform) {
        let uniformValues = &mut self.uniformValues;
        uniformValues.insert(String::from(key),value);
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
                Uniform::Matrix3x3() => {

                },
                Uniform::Matrix4x4() => {

                }
            }
        }
    }
}
