// ??

use ios_rust_binding::EAGLContext;
use gles_rust_binding::*;
use std::mem;
pub trait SerialDispatch {
    fn makeCurrentContext(&self);
}

impl SerialDispatch {
    pub fn runOperationAsynchronously<F>(&self, operation: F)
        where F : FnOnce() -> ()
    {
        self.makeCurrentContext();
        operation();
    }

    pub fn runOperationSynchronously<T,F>(&self, operation: F) -> T
        where F: FnOnce() -> T
    {
        self.makeCurrentContext();
        operation()
    }

}
use ios_rust_binding::{ShareId};
pub struct GlContext{
    pub context: ShareId<EAGLContext>,
    pub standardImageVBO: GLuint
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
        GlContext{context:generatedContext,standardImageVBO:standardImageVBO}
    }


    pub fn presentBufferForDisplay(&self){
        self.context.presentRenderBuffer(GL_RENDERBUFFER as u64);
    }
}

impl SerialDispatch for GlContext {
    fn makeCurrentContext(&self){
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

