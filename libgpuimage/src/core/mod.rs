pub mod context;
pub mod framebuffer;
pub mod framebuffercache;


pub use self::framebuffer::*;
pub use self::framebuffercache::*;
pub use self::context::*;
pub use gles_rust_binding::GLProgram as Program;
pub use gles_rust_binding::*;

#[repr(C)]
#[derive(Copy,Clone)]
pub enum NodeType{
    Picture,
    Camera,
    BasicFilter,
    GuassianBlurFilter,
    LookupTableFilter,
    ToneCurveFilter,
    View
}

impl NodeType {
    pub fn get_name(&self) -> &str {
        match self {
            NodeType::Picture => "NodeType::Picture",
            NodeType::Camera => "NodeType::Camera",
            NodeType::BasicFilter => "NodeType::BasicFilter",
            NodeType::GuassianBlurFilter => "NodeType::GuassianBlurFilter",
            NodeType::LookupTableFilter => "NodeType::LookupTableFilter",
            NodeType::ToneCurveFilter => "NodeType::ToneCurveFilter",
            NodeType::View => "NodeType::View"
        }
    }
}

pub trait Node {
    fn get_type_name(&self) -> NodeType;
}
pub struct RenderNode(NodeType);

impl RenderNode {
    pub fn new(_type: NodeType) -> Self {
        RenderNode(_type)
    }
    pub fn name(&self) -> &str {
        self.0.get_name()
    }
}

impl Node for RenderNode {
    fn get_type_name(&self) -> NodeType {
        self.0
    }
}


// 这两个trait描述滤镜链图的关系
// 更确切地说，滤镜关系是一张计算图，渲染方向就是前向计算Forward Compute， Graph = {Node Edge}


pub trait Source<'a>{
    fn addTarget(&self, target: &'a dyn Consumer, _location: u32);
    fn removeAllTargets(&self);
    fn updateTargetsWithFramebuffer(&self, framebuffer:&Framebuffer);
}

pub trait Consumer {
    fn setSource(&self, _source: &dyn Source, _location: u32);
    fn newFramebufferAvailable(&self,framebuffer: &Framebuffer,fromSourceIndex: usize);
}







lazy_static!{
    pub static ref sharedImageProcessingContext : GlContext = GlContext::new();
}



pub mod GLRender {
    use super::Color;
    use super::Framebuffer;
    use gles_rust_binding::*;
    use super::sharedImageProcessingContext;
    use super::{InputTextureProperties,InputTextureStorageFormat};
    use std::ptr;

    pub fn clearFramebufferWithColor(color:Color) {
        unsafe {
            glClearColor(color.redComponent, color.greenComponent, color.blueComponent, color.alphaComponent);
            glClear(GL_COLOR_BUFFER_BIT);
        }
    }



//    pub fn textureUnitForIndex(index: usize) -> GLenum {
//        match index {
//            0 => GL_TEXTURE0,
//            1 => GL_TEXTURE1,
//            2 => GL_TEXTURE2,
//            3 => GL_TEXTURE3,
//            4 => GL_TEXTURE4,
//            5 => GL_TEXTURE5,
//            6 => GL_TEXTURE6,
//            7 => GL_TEXTURE7,
//            8 => GL_TEXTURE8,
//            _ => panic!("Attempted to address too high a texture unit")
//        }
//    }
//
//    pub fn renderQuadWithShader(program: &GLProgram, vertices:Option<[f32;8]>, vertexBufferObject:Option<GLuint>, inputTextures: &Vec<InputTextureProperties>) {
//        sharedImageProcessingContext.makeCurrentContext();
//        unsafe {
//
//            program.bind();
//
//
//            let position = program.get_attribute("position").unwrap();
//            if let Some(boundVBO) = vertexBufferObject {
//                println!("render boundVBO");
//
//                glBindBuffer(GL_ARRAY_BUFFER,boundVBO);
//                glVertexAttribPointer(position.location() as u32,2,GL_FLOAT,0,0,ptr::null());
//                glBindBuffer(GL_ARRAY_BUFFER,0);
//            }else{
//                println!("render vertices");
//
//                glVertexAttribPointer(position.location() as u32,2,GL_FLOAT,GL_FALSE,0,vertices.unwrap().as_ptr() as *const _);
//                glEnableVertexAttribArray(position.location() as u32);
//            }
//
//
//            for (index,inputTexture) in inputTextures.iter().enumerate() {
//
//                let (attribute,inputTextureUniform) = if index == 0 {
//                    (format!("inputTextureCoordinate"),format!("inputImageTexture"))
//                }else{
//                    (format!("inputTextureCoordinate{}",index),format!("inputImageTexture{}",index))
//                };
//
//                if let Some(textureCoordinateAttribute) = program.get_attribute(&attribute) {
//                    match inputTexture.textureStorage {
//                        InputTextureStorageFormat::textureCoordinate(textureCoordinates) => {
//                            println!("view texture coordinate");
//
//                            glVertexAttribPointer(textureCoordinateAttribute.location() as u32,2,GL_FLOAT,0,0,textureCoordinates.as_ptr() as *const _);
//                            glEnableVertexAttribArray(textureCoordinateAttribute.location() as u32);
//
//                        },
//                        InputTextureStorageFormat::textureVBO(textureVBO) => {
//                            println!("view texture vbo");
//                            glBindBuffer(GL_ARRAY_BUFFER,textureVBO);
//                            glVertexAttribPointer(textureCoordinateAttribute.location() as u32,2,GL_FLOAT,0,0,ptr::null());
//                            glEnableVertexAttribArray(textureCoordinateAttribute.location() as u32);
//
//                        }
//                    }
//
//                }else if index == 0 {
//                    panic!("The required attribute named inputTextureCoordinate was missing from the shader program during rendering.");
//
//                }
//
//
//                let inputImageTexture = program.get_uniform(&inputTextureUniform);
//                glActiveTexture(textureUnitForIndex(index));
//                glBindTexture(GL_TEXTURE_2D,inputTexture.texture);
//                glUniform1i(0,inputImageTexture.location() as i32);
//
//            }
//
//
//
//
//
//
//
//            glDrawArrays(GL_TRIANGLE_STRIP,0,4);
//
//
//            if let Some(_) = vertexBufferObject {
//                glBindBuffer(GL_ARRAY_BUFFER,0);
//            }
//
//            for (index,_) in inputTextures.iter().enumerate() {
//                glActiveTexture(textureUnitForIndex(index));
//                glBindTexture(GL_TEXTURE_2D,0);
//            }
//        }
//    }

    pub fn renderQuadWithShaderF(program: &GLProgram, framebuffer: &Framebuffer, vertices:[f32;8]) {
        sharedImageProcessingContext.makeCurrentContext();
        unsafe {

            program.bind();

            let position = program.get_attribute("position").unwrap();
            let textureCoordinate = program.get_attribute("inputTextureCoordinate").unwrap();
            let inputTexture = program.get_uniform("inputImageTexture");


//            let vertices:[f32;8] = [-1.0,1.0,1.0,1.0,-1.0,-1.0,1.0,-1.0];

            let textureCoordinates:[f32;8] = [1.0,1.0, 1.0,0.0, 0.0,1.0, 0.0,0.0];

            glClearColor(1.0,0.0,0.0,1.0);
            glClear(GL_COLOR_BUFFER_BIT);


            glVertexAttribPointer(position.location() as u32,2,GL_FLOAT,GL_FALSE,0,vertices.as_ptr() as *const _);
            glEnableVertexAttribArray(position.location() as u32);

            glVertexAttribPointer(textureCoordinate.location() as u32,2,GL_FLOAT,GL_FALSE,0,textureCoordinates.as_ptr() as *const _);
            glEnableVertexAttribArray(textureCoordinate.location() as u32);

            glActiveTexture(GL_TEXTURE0);
            glBindTexture(GL_TEXTURE_2D,framebuffer.texture);
            glUniform1i(0,inputTexture.location() as i32);

            glDrawArrays(GL_TRIANGLE_STRIP,0,4);


        }
    }
}


pub struct Color {
    pub redComponent: f32,
    pub greenComponent: f32,
    pub blueComponent: f32,
    pub alphaComponent: f32
}

impl Color{
    pub fn new(redComponent: f32, greenComponent: f32, blueComponent: f32, alphaComponent: f32) -> Self{
        Color{redComponent:redComponent,greenComponent:greenComponent,blueComponent:blueComponent,alphaComponent:alphaComponent}
    }

    pub fn black() -> Self {
        Color::new(0.0,0.0,0.0,1.0)
    }

    pub fn white() -> Self {
        Color::new(1.0,1.0,1.0,1.0)
    }

    pub fn red() -> Self {
        Color::new(1.0, 0.0, 0.0,1.0)
    }

    pub fn green() -> Self {
        Color::new(0.0,1.0,0.0,1.0)
    }

    pub fn blue() -> Self {
        Color::new(0.0,0.0,1.0,1.0)
    }

    pub fn transparent() -> Self {
        Color::new(0.0,0.0,0.0,0.0)
    }


    pub fn toGLArray(&self) -> [GLfloat;3] {
        [self.redComponent as GLfloat,self.greenComponent as GLfloat,self.blueComponent as GLfloat]
    }

    pub fn toGLArrayWithAlpha(&self) -> [GLfloat;4] {
        [self.redComponent as GLfloat,self.greenComponent as GLfloat,self.blueComponent as GLfloat, self.alphaComponent as GLfloat]
    }

}


