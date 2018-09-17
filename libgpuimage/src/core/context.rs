use ios_rust_binding::EAGLContext;
use gles_rust_binding::*;
use super::FramebufferCache;
use super::Rotation;
use std::collections::BTreeMap;
use std::mem;

use ios_rust_binding::{ShareId};
pub struct GlContext{
    pub context: ShareId<EAGLContext>,
    pub standardImageVBO: GLuint,
    pub passthroughShader: GLProgram,
    pub frameubfferCache: FramebufferCache,
    pub textureVBOs: BTreeMap<u32,GLuint>

}





impl GlContext {
    pub fn new() -> Self{
        let generatedContext = EAGLContext::withApi(2);
        let generatedContext = generatedContext.share();
        EAGLContext::setCurrentContext(&generatedContext);

        let standardImageVertices:[f32;8] = [-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0];

        let standardImageVBO = generateVBO(standardImageVertices.as_ptr(),standardImageVertices.len());


        let vertexStr = r#"
 attribute vec4 position;
 attribute vec4 inputTextureCoordinate;

 varying vec2 textureCoordinate;

 void main()
 {
     gl_Position = position;
     textureCoordinate = inputTextureCoordinate.xy;
 }
    "#;

        let fragmentStr = r#"
 precision mediump float;

 varying highp vec2 textureCoordinate;
 uniform sampler2D inputImageTexture;

 void main()
 {
     gl_FragColor = texture2D(inputImageTexture, textureCoordinate);
 }
    "#;
        let program = GLProgram::new(vertexStr,fragmentStr);

        let mut textureVBOs = BTreeMap::new();

        textureVBOs.insert(Rotation::noRotation.toRawValue(),generateVBO(Rotation::noRotation.textureCoordinates().as_ptr(),Rotation::noRotation.textureCoordinates().len()));
        textureVBOs.insert(Rotation::rotateCounterclockwise.toRawValue(),generateVBO(Rotation::rotateCounterclockwise.textureCoordinates().as_ptr(),Rotation::rotateCounterclockwise.textureCoordinates().len()));
        textureVBOs.insert(Rotation::rotateClockwise.toRawValue(),generateVBO(Rotation::rotateClockwise.textureCoordinates().as_ptr(),Rotation::rotateClockwise.textureCoordinates().len()));
        textureVBOs.insert(Rotation::rotate180.toRawValue(),generateVBO(Rotation::rotate180.textureCoordinates().as_ptr(),Rotation::rotate180.textureCoordinates().len()));
        textureVBOs.insert(Rotation::flipHorizontally.toRawValue(),generateVBO(Rotation::flipHorizontally.textureCoordinates().as_ptr(),Rotation::flipHorizontally.textureCoordinates().len()));
        textureVBOs.insert(Rotation::flipVertically.toRawValue(),generateVBO(Rotation::flipVertically.textureCoordinates().as_ptr(),Rotation::flipVertically.textureCoordinates().len()));
        textureVBOs.insert(Rotation::rotateClockwiseAndFlipVertically.toRawValue(),generateVBO(Rotation::rotateClockwiseAndFlipVertically.textureCoordinates().as_ptr(),Rotation::rotateClockwiseAndFlipVertically.textureCoordinates().len()));
        textureVBOs.insert(Rotation::rotateClockwiseAndFlipHorizontally.toRawValue(),generateVBO(Rotation::rotateClockwiseAndFlipHorizontally.textureCoordinates().as_ptr(),Rotation::rotateClockwiseAndFlipHorizontally.textureCoordinates().len()));


        GlContext{
            context:generatedContext,
            standardImageVBO:standardImageVBO,
            passthroughShader:program,
            frameubfferCache: FramebufferCache::default(),
            textureVBOs: textureVBOs
        }
    }


    pub fn presentBufferForDisplay(&self){
        self.context.presentRenderBuffer(GL_RENDERBUFFER as u64);
    }

    pub fn makeCurrentContext(&self){
        EAGLContext::makeCurrentContext(&self.context);
    }



    pub fn textureVBO(&self, rotation: Rotation) -> GLuint {
        let textureVBO = self.textureVBOs.get(&rotation.toRawValue()).expect("Error in context 95 line");
        *textureVBO
    }
}





fn generateVBO(vertices: *const f32, len: usize) -> GLuint {
    let mut newBuffer: GLuint = 0;
    unsafe {
        glGenBuffers(1,&mut newBuffer);
        glBindBuffer(GL_ARRAY_BUFFER,newBuffer);
        glBufferData(GL_ARRAY_BUFFER,(mem::size_of::<GLfloat>() as isize) * (len as isize) , vertices as *const _,GL_STATIC_DRAW);
        glBindBuffer(GL_ARRAY_BUFFER,0);
        newBuffer
    }
}

fn deleteVBO(vbo: GLuint){
    unsafe {
        glDeleteBuffers(1,&vbo);
    }
}

