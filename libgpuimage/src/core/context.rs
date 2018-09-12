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
        operation();
    }

}

pub struct GlContext{
    context: EAGLContext,
    standardImageVBO: GLuint
}


static standardImageVertices: [f32; 8] = [
    -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0
];

static verticallyInvertedImageVertices: [f32; 8] = [
    -1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0
];


impl GlContext {
    fn new(){
        let gneratedContext = EAGLContext::with_api(2);
        let vec = Vec::from(standardImageVertices);
        let standardImageVBO = generateVBO(vec);

    }
}

impl SerialDispatch for GlContext {
    fn makeCurrentContext(&self){

    }
}



fn generateVBO(vertices: Vec<GLfloat>) -> GLuint {
    let mut newBuffer: GLuint = 0;
    unsafe {
        glGenBuffers(1,&mut newBuffer);
        glBindBuffer(GL_ARRAY_BUFFER,newBuffer);
        glBufferData(GL_ARRAY_BUFFER,(mem::size_of::<GLfloat>() as usize) * vertices.len() as isize, vertices.as_ptr() as *const _,GL_STATIC_DRAW);
        glBindBuffer(GL_ARRAY_BUFFER,0);
        newBuffer
    }
}

fn deleteVBO(vbo: GLuint){
    glDeleteBuffers(1,&vbo);
}

