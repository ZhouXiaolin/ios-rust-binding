use gles_rust_binding::*;
use std::mem;
use super::{Rotation, FramebufferCache};

#[repr(C)]
#[derive(Debug)]
pub struct GlContext{
    pub standardImageVBO: GLuint,
    pub passthroughShader: GLProgram,
    pub framebufferCache: FramebufferCache,
    pub textureVBOs: Vec<GLuint>

}

impl Drop for GlContext {
    fn drop(&mut self){
        info!("Drop GlContext");
    }
}

const vertexStr: &str = r#"
 attribute vec4 position;
 attribute vec2 inputTextureCoordinate;

 varying vec2 textureCoordinate;

 void main()
 {
     gl_Position = position;
     textureCoordinate = inputTextureCoordinate.xy;
 }
    "#;

const fragmentStr: &str = r#"
 precision mediump float;

 varying highp vec2 textureCoordinate;
 uniform sampler2D inputImageTexture;

 void main()
 {
     gl_FragColor = texture2D(inputImageTexture, textureCoordinate);
 }
    "#;

impl GlContext {
    pub fn new() -> Self{

        let standardImageVertices:[f32;8] = [-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0];
        let standardImageVBO = generateVBO(&standardImageVertices);

        let program = GLProgram::new(vertexStr,fragmentStr);
        let textureVBOs = generateTextureVBOs();

        Self{
            standardImageVBO,
            passthroughShader:program,
            framebufferCache: FramebufferCache::default(),
            textureVBOs
        }
    }


    pub fn textureVBO(&self, rotation: Rotation) -> GLuint {
        let textureVBO = self.textureVBOs[rotation.toRawValue()];
        textureVBO
    }
}



fn generateTextureVBOs() -> Vec<GLuint> {
    let mut textureVBOs = Vec::with_capacity(8);

    textureVBOs.push(generateVBO(&Rotation::noRotation.textureCoordinates()));
    textureVBOs.push(generateVBO(&Rotation::rotateCounterclockwise.textureCoordinates()));
    textureVBOs.push(generateVBO(&Rotation::rotateClockwise.textureCoordinates()));
    textureVBOs.push(generateVBO(&Rotation::rotate180.textureCoordinates()));
    textureVBOs.push(generateVBO(&Rotation::flipHorizontally.textureCoordinates()));
    textureVBOs.push(generateVBO(&Rotation::flipVertically.textureCoordinates()));
    textureVBOs.push(generateVBO(&Rotation::rotateClockwiseAndFlipVertically.textureCoordinates()));
    textureVBOs.push(generateVBO(&Rotation::rotateClockwiseAndFlipHorizontally.textureCoordinates()));

    textureVBOs
}


fn generateVBO<T>(vertices: &[T]) -> GLuint {

    let mut newBuffer: GLuint = 0;
    let length = vertices.len();
    let kind_size = mem::size_of::<T>();
    let size = kind_size * length;

    unsafe {
        glGenBuffers(1,&mut newBuffer);
        glBindBuffer(GL_ARRAY_BUFFER,newBuffer);
        glBufferData(GL_ARRAY_BUFFER,size as GLsizeiptr , vertices.as_ptr() as *const GLvoid,GL_STATIC_DRAW);
        glBindBuffer(GL_ARRAY_BUFFER,0);
        newBuffer
    }
}

fn deleteVBO(vbo: GLuint){
    unsafe {
        glDeleteBuffers(1,&vbo);
    }
}



lazy_static!{
    pub static ref sharedImageProcessingContext : GlContext = GlContext::new();
}
