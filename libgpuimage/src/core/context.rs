use ios_rust_binding::EAGLContext;
use gles_rust_binding::*;
use super::FramebufferCache;
use std::mem;

use ios_rust_binding::{ShareId};
pub struct GlContext{
    pub context: ShareId<EAGLContext>,
    pub standardImageVBO: GLuint,
    pub passthroughShader: GLProgram,
    pub frameubfferCache: FramebufferCache
}


static standardImageVertices: [f32; 8] = [
    -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0
];

static verticallyInvertedImageVertices: [f32; 8] = [
    -1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0
];


impl GlContext {
    pub fn new() -> Self{
        let generatedContext = EAGLContext::withApi(2);
        let generatedContext = generatedContext.share();
        EAGLContext::setCurrentContext(&generatedContext);

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
        GlContext{
            context:generatedContext,
            standardImageVBO:standardImageVBO,
            passthroughShader:program,
            frameubfferCache: FramebufferCache::default()
        }
    }


    pub fn presentBufferForDisplay(&self){
        self.context.presentRenderBuffer(GL_RENDERBUFFER as u64);
    }

    pub fn makeCurrentContext(&self){
        EAGLContext::makeCurrentContext(&self.context);
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

