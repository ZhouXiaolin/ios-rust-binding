use gles_rust_binding::*;
use super::FramebufferCache;
use super::Rotation;
use std::collections::BTreeMap;
use std::mem;
#[cfg(target_os = "ios")]
use ios_rust_binding::{EAGLContext,NSUInteger,ShareId};

#[cfg(target_os = "ios")]
pub struct GlContext{
    pub context: ShareId<EAGLContext>,
    pub standardImageVBO: GLuint,
    pub passthroughShader: GLProgram,
    pub frameubfferCache: FramebufferCache,
    pub textureVBOs: Vec<GLuint>

}

#[cfg(target_os = "android")]
pub struct GlContext{
    pub standardImageVBO: GLuint,
    pub passthroughShader: GLProgram,
    pub frameubfferCache: FramebufferCache,
    pub textureVBOs: Vec<GLuint>
}

static vertexStr: &str = r#"
 attribute vec4 position;
 attribute vec4 inputTextureCoordinate;

 varying vec2 textureCoordinate;

 void main()
 {
     gl_Position = position;
     textureCoordinate = inputTextureCoordinate.xy;
 }
    "#;

static fragmentStr: &str = r#"
 precision mediump float;

 varying highp vec2 textureCoordinate;
 uniform sampler2D inputImageTexture;

 void main()
 {
     gl_FragColor = texture2D(inputImageTexture, textureCoordinate);
 }
    "#;

impl GlContext {
    #[cfg(target_os = "ios")]
    pub fn new() -> Self{
        let generatedContext = EAGLContext::withApi(2);
        let generatedContext = generatedContext.share();
        EAGLContext::setCurrentContext(&generatedContext);

        let standardImageVertices:[f32;8] = [-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0];
        let standardImageVBO = generateVBO(&standardImageVertices);

        let program = GLProgram::new(vertexStr,fragmentStr);
        let textureVBOs = Self::generateTextureVBOs();

        GlContext{
            context:generatedContext,
            standardImageVBO:standardImageVBO,
            passthroughShader:program,
            frameubfferCache: FramebufferCache::default(),
            textureVBOs: textureVBOs
        }
    }

    #[cfg(target_os = "android")]
    pub fn new() -> Self{
        let standardImageVertices:[f32;8] = [-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0];
        let standardImageVBO = generateVBO(&standardImageVertices);

        let program = GLProgram::new(vertexStr,fragmentStr);
        let textureVBOs = Self::generateTextureVBOs();

        GlContext{
            standardImageVBO:standardImageVBO,
            passthroughShader:program,
            frameubfferCache: FramebufferCache::default(),
            textureVBOs: textureVBOs
        }
    }

    #[cfg(target_os = "ios")]
    pub fn presentBufferForDisplay(&self){
        self.context.presentRenderBuffer(GL_RENDERBUFFER as NSUInteger);
    }
    #[cfg(target_os = "android")]
    pub fn presentBufferForDisplay(&self){

    }

    #[cfg(target_os = "ios")]
    pub fn makeCurrentContext(&self){
        EAGLContext::makeCurrentContext(&self.context);
    }

    #[cfg(target_os = "android")]
    pub fn makeCurrentContext(&self){

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






    pub fn textureVBO(&self, rotation: Rotation) -> GLuint {
        let textureVBO = self.textureVBOs[rotation.toRawValue()];
        textureVBO
    }
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

