use gles_rust_binding::*;
use super::{Color, InputTextureProperties, InputTextureStorageFormat,Framebuffer};
use super::ShaderUniformSettings;
use std::ptr;
use std::rc::Rc;
use std::sync::Arc;
use super::PrimitiveType;
impl Into<GLenum> for PrimitiveType {
    fn into(self) -> GLenum {
        match self {
            PrimitiveType::Point => GL_POINTS,
            PrimitiveType::Line => GL_LINES,
            PrimitiveType::LineStrip => GL_LINE_STRIP,
            PrimitiveType::Triangle => GL_TRIANGLES,
            PrimitiveType::TriangleStrip => GL_TRIANGLE_STRIP
        }
    }
}



pub fn clearFramebufferWithColor(color:Color) {
    unsafe {
        glClearColor(color.redComponent, color.greenComponent, color.blueComponent, color.alphaComponent);
        glClear(GL_COLOR_BUFFER_BIT);
    }
}

pub fn textureUnitForIndex(index: usize) -> GLenum {
    match index {
        0 => GL_TEXTURE0,
        1 => GL_TEXTURE1,
        2 => GL_TEXTURE2,
        3 => GL_TEXTURE3,
        4 => GL_TEXTURE4,
        5 => GL_TEXTURE5,
        6 => GL_TEXTURE6,
        7 => GL_TEXTURE7,
        8 => GL_TEXTURE8,
        _ => unreachable!("Attempted to address too high a texture unit")
    }
}


fn inputTextureProperty(index: usize) -> (String,String) {
    let (inputTextureCoordinateString,inputImageTextureString) = if index == 0 {
        (format!("inputTextureCoordinate"),format!("inputImageTexture"))
    }else{
        (format!("inputTextureCoordinate{}",index+1),format!("inputImageTexture{}",index+1))
    };
    (inputTextureCoordinateString,inputImageTextureString)
}




pub fn enableBlending(sfactor: GLenum, dfactor: GLenum){
    unsafe {
        glEnable(GL_BLEND);
        glBlendFunc(sfactor,dfactor);
    }
}

pub fn disableBlending(){
    unsafe {
        glDisable(GL_BLEND);
    }
}



pub struct Encoder{

}

impl Encoder {

    fn drawPrimitive(&self, mode: PrimitiveType, start: i32, count: i32){
        unsafe {
            glDrawArrays(mode.into(),start,count);
            glFinish();
        }
    }

    fn setVertexBuffer(&self, vertex:&InputTextureStorageFormat, attribute:&GLAttribute){

        unsafe {
            match vertex {
                InputTextureStorageFormat::textureCoordinate(ref vertices) => {
                    glVertexAttribPointer(attribute.location() as u32,2,GL_FLOAT,GL_FALSE,0,vertices.as_ptr() as *const _);
                    glEnableVertexAttribArray(attribute.location() as u32);
                },
                InputTextureStorageFormat::textureVBO(ref boundVBO) => {
                    glBindBuffer(GL_ARRAY_BUFFER,*boundVBO);
                    glVertexAttribPointer(attribute.location() as u32, 2, GL_FLOAT, 0, 0, ptr::null());
                    glEnableVertexAttribArray(attribute.location() as u32);
                    glBindBuffer(GL_ARRAY_BUFFER,0);
                }
            }
        }
    }

    fn setTexture(&self, inputTexture:&InputTextureProperties, uniform:&GLUniform, index: usize){
        unsafe {
            glActiveTexture(textureUnitForIndex(index));
            glBindTexture(uniform.kind().toUniform(),inputTexture.texture);
            glUniform1i(uniform.location() as i32,index as i32);
        }
    }
}

pub struct RenderPipelineState{
    pub framebuffer:Rc<Framebuffer>,
    pub color: Color
}

impl RenderPipelineState {


    pub fn run<T>(self, operation:T) -> Rc<Framebuffer> where T:FnOnce() -> () {
        self.framebuffer.bindFramebufferForRendering();
        clearFramebufferWithColor(self.color);
        operation();
        self.framebuffer.unbindFramebufferForRendering();
        self.framebuffer
    }
    pub fn run_and_then<T>(self, operation: T) -> Rc<Framebuffer> where T:FnOnce() -> () {
        self.framebuffer.bindFramebufferForRendering();
        clearFramebufferWithColor(self.color);
        operation();
        self.framebuffer
    }


}

pub fn renderQuadWithShader(program: &GLProgram, uniformSettings:&ShaderUniformSettings,inputTextures: &Vec<InputTextureProperties>, vertex:InputTextureStorageFormat) {

    unsafe {

        let encoder = Encoder{};

        program.bind();

        uniformSettings.restoreShaderSettings(program);

        let position = program.get_attribute("position").unwrap();

        encoder.setVertexBuffer(&vertex,position);

        for (index,inputTexture) in inputTextures.iter().enumerate(){

            let (inputTextureCoordinateString,inputImageTextureString) = inputTextureProperty(index);

            if let Some(textureCoordinate) = program.get_attribute(&inputTextureCoordinateString) {

                encoder.setVertexBuffer(&inputTexture.textureStorage,textureCoordinate);

            }else if index == 0 {

                panic!("The required attribute named inputTextureCoordinate was missing from the shader program during rendering.");

            }

            let uniform = program.get_uniform(&inputImageTextureString);

            encoder.setTexture(inputTexture,uniform,index);
        }


        encoder.drawPrimitive(PrimitiveType::TriangleStrip,0,4);

        if let InputTextureStorageFormat::textureVBO(_) = vertex {
            glBindBuffer(GL_ARRAY_BUFFER,0);
        }

        for (index,ino) in inputTextures.iter().enumerate() {

            let (_,inputImageTextureString) = inputTextureProperty(index);

            let inputImageTexture = program.get_uniform(&inputImageTextureString);

            glActiveTexture(textureUnitForIndex(index));
            glBindTexture(inputImageTexture.kind().toUniform(),0);
        }

    }
}



