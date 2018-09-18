use gles_rust_binding::*;
use std::ptr;
use std::ffi::CStr;
use fnv::FnvHashMap;
// program的功能完全由GLProgram代替
pub struct ShaderProgram{
    program: GLuint,
    vertexShader: GLuint,
    fragmentShader: GLuint,
    attributeAddresses:FnvHashMap<String,GLuint>,
    uniformAddresses:FnvHashMap<String,GLuint>

}
enum ShaderType{
    vertex,
    fragment
}

fn compileShader(shaderString: &str, _type: ShaderType) -> GLuint {

    unsafe {
        let shaderHandle  = match _type {
        ShaderType::vertex => {
            glCreateShader(GL_VERTEX_SHADER)
        },
        ShaderType::fragment => {
            glCreateShader(GL_FRAGMENT_SHADER)
        }
    };


        glShaderSource(shaderHandle,1,shaderString.as_ptr() as *const _ ,ptr::null());
        glCompileShader(shaderHandle);


        let mut compileStatus: GLint = 1;
        glGetShaderiv(shaderHandle,GL_COMPILE_STATUS,&mut compileStatus);
        if compileStatus != 1 {
            let mut logLength : GLint = 0;
            glGetShaderiv(shaderHandle,GL_INFO_LOG_LENGTH,&mut logLength);
            if logLength > 0 {
//            let mut compileLog = [i8;logLength];
//            glGetShaderInfoLog(shaderHandle,logLength,&mut logLength,&mut compileLog);

            }
        }

        shaderHandle
    }

}


impl ShaderProgram {
//    pub fn new(vertexString: &str, fragmentString: &str) -> Self {
//        Program{}
//
//
//    }

    pub fn addAttribute(&self, name: &str) {

    }

    pub fn attributeIndex(name: &str){

    }
}