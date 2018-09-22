use super::*;
use gles_rust_binding::*;
use std::ptr;

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
        _ => panic!("Attempted to address too high a texture unit")
    }
}


pub fn renderQuadWithShader(program: &GLProgram, inputTextures: &Vec<InputTextureProperties>, vertex:InputTextureStorageFormat) {

    sharedImageProcessingContext.makeCurrentContext();
    unsafe {

        program.bind();

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

            let (inputTextureCoordinateString,inputImageTextureString) = if index == 0 {
                (format!("inputTextureCoordinate"),format!("inputImageTexture"))
            }else{
                (format!("inputTextureCoordinate{}",index),format!("inputImageTexture{}",index))
            };


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
            glBindTexture(GL_TEXTURE_2D,inputTexture.texture);
            glUniform1i(inputImageTexture.location() as i32,index as i32);
        }

        glDrawArrays(GL_TRIANGLE_STRIP,0,4);


        for (index,_) in inputTextures.iter().enumerate() {
            glActiveTexture(textureUnitForIndex(index));
            glBindTexture(GL_TEXTURE_2D,0);
        }

    }
}


#[derive(Copy,Clone,Debug,Default)]
pub struct GLSize {
    pub width : i32,
    pub height: i32
}
impl GLSize {
    pub fn new(width: i32, height: i32) -> Self {
        GLSize{width:width,height:height}
    }
}


#[derive(Copy, Clone,Debug,Default)]
pub struct Size {
    pub width: f32,
    pub height: f32
}