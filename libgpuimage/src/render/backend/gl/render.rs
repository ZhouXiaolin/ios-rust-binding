use gles_rust_binding::*;
use super::{Color, InputTextureProperties, InputTextureStorageFormat};
use super::ShaderUniformSettings;
use std::ptr;
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
        }
    }
}


pub fn renderQuadWithShader(program: &GLProgram, uniformSettings:&ShaderUniformSettings,inputTextures: &Vec<InputTextureProperties>, vertex:InputTextureStorageFormat) {


    unsafe {

        let encoder = Encoder{};

        program.bind();

        uniformSettings.restoreShaderSettings(program);

        let position = program.get_attribute("position").unwrap();




        match vertex {
            InputTextureStorageFormat::textureCoordinate(ref vertices) => {
                glVertexAttribPointer(position.location() as u32,2,GL_FLOAT,GL_FALSE,0,vertices.as_ptr() as *const _);
                glEnableVertexAttribArray(position.location() as u32);
            },
            InputTextureStorageFormat::textureVBO(boundVBO) => {
                glBindBuffer(GL_ARRAY_BUFFER,boundVBO);
                glVertexAttribPointer(position.location() as u32, 2, GL_FLOAT, 0, 0, ptr::null());
                glEnableVertexAttribArray(position.location() as u32);
                glBindBuffer(GL_ARRAY_BUFFER,0);
            }
        }





        for (index,inputTexture) in inputTextures.iter().enumerate(){

            let (inputTextureCoordinateString,inputImageTextureString) = inputTextureProperty(index);


            if let Some(textureCoordinate) = program.get_attribute(&inputTextureCoordinateString) {

                match inputTexture.textureStorage {
                    InputTextureStorageFormat::textureVBO(texVBO) => {
                        glBindBuffer(GL_ARRAY_BUFFER,texVBO);
                        glVertexAttribPointer(textureCoordinate.location() as u32, 2, GL_FLOAT, 0, 0, ptr::null());
                        glEnableVertexAttribArray(textureCoordinate.location() as u32);
                        glBindBuffer(GL_ARRAY_BUFFER,0);

                    },
                    InputTextureStorageFormat::textureCoordinate(ref texCoord) => {
                        glVertexAttribPointer(textureCoordinate.location() as u32, 2, GL_FLOAT, GL_FALSE, 0, texCoord.as_ptr() as *const _);
                        glEnableVertexAttribArray(textureCoordinate.location() as u32);
                    }
                }

            }else if index == 0{
                panic!("The required attribute named inputTextureCoordinate was missing from the shader program during rendering.");
            }



            let inputImageTexture = program.get_uniform(&inputImageTextureString);
            glActiveTexture(textureUnitForIndex(index));
            glBindTexture(inputImageTexture.kind().toUniform(),inputTexture.texture);
            glUniform1i(inputImageTexture.location() as i32,index as i32);
        }

//        glDrawArrays(GL_TRIANGLE_STRIP,0,4);

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



