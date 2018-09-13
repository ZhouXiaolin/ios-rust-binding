use std::os::raw::{c_void};
use std::mem::transmute;
use ios_rust_binding::{UIView,NSUInteger,ShareId,CALayer};

use gles_rust_binding::*;

use core::{Consumer,Source,sharedImageProcessingContext};
use core::context::SerialDispatch;
use core::{Node,NodeType, RenderNode};
use core::framebuffer::{GLSize,Framebuffer};
use std::cell::Cell;
#[repr(C)]
pub struct XHeyView {
    _type: RenderNode,
    displayFramebuffer: Cell<GLuint>,
    displayRenderbuffer: Cell<GLuint>,
    backingSize: Cell<GLSize>,
    layer: ShareId<CALayer>,
    program: GLProgram
}




impl Consumer for XHeyView {
    fn set_source(&self, _source: &dyn Source, _location: u32) {
        println!("XheyView set_source");

    }
}

struct Color {
    redComponent: f32,
    greenComponent: f32,
    blueComponent: f32,
    alphaComponent: f32
}

impl Color{
    fn new(redComponent: f32, greenComponent: f32, blueComponent: f32, alphaComponent: f32) -> Self{
        Color{redComponent:redComponent,greenComponent:greenComponent,blueComponent:blueComponent,alphaComponent:alphaComponent}
    }

    fn black() -> Self {
        Color::new(0.0,0.0,0.0,1.0)
    }

    fn white() -> Self {
        Color::new(1.0,1.0,1.0,1.0)
    }

    fn red() -> Self {
        Color::new(1.0, 0.0, 0.0,1.0)
    }

    fn green() -> Self {
        Color::new(0.0,1.0,0.0,1.0)
    }

    fn blue() -> Self {
        Color::new(0.0,0.0,1.0,1.0)
    }

    fn transparent() -> Self {
        Color::new(0.0,0.0,0.0,0.0)
    }


}

fn clearFramebufferWithColor(color:Color) {
    unsafe {
        glClearColor(color.redComponent, color.greenComponent, color.blueComponent, color.alphaComponent);
        glClear(GL_COLOR_BUFFER_BIT);
    }
}

fn renderQuadWithShader(program: &GLProgram, framebuffer: &Framebuffer) {
    sharedImageProcessingContext.makeCurrentContext();
    unsafe {

        program.bind();


//        let mut imageTexture : GLuint = 0;
//        glGenTextures(1,&mut imageTexture);
//        glBindTexture(GL_TEXTURE_2D,imageTexture);
//        glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_MIN_FILTER, GL_LINEAR as i32);
//        glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_MAG_FILTER, GL_LINEAR as i32);
//        glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE as i32);
//        glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE as i32);
//        glTexImage2D(GL_TEXTURE_2D,0,GL_RGBA as i32,width,height,0,GL_BGRA,GL_UNSIGNED_BYTE,data as *const _);


        let position = program.get_attribute("position");
        let textureCoordinate = program.get_attribute("inputTextureCoordinate");
        let inputTexture = program.get_uniform("inputImageTexture");


        let vertices:[f32;8] = [-1.0,1.0,1.0,1.0,-1.0,-1.0,1.0,-1.0];

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



impl XHeyView {
    fn new(view: Box<UIView>) -> Self {
        let layer = view.get_layer();
        let layer = layer.share();


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

        XHeyView{
            _type:RenderNode::new(NodeType::View),
            displayFramebuffer:Cell::new(0),
            displayRenderbuffer:Cell::new(0),
            backingSize:Cell::new(GLSize::default()),
            layer:layer,
            program:program
        }
    }
    fn newFramebufferAvailable(&self,framebuffer: &Framebuffer){
        sharedImageProcessingContext.makeCurrentContext();

        if self.displayFramebuffer.get() == 0 {
            self.createDisplayFramebuffer()
        }

        let program = &self.program;

        self.activateDisplayFramebuffer();
        clearFramebufferWithColor(Color::black());

        renderQuadWithShader(program,framebuffer);

        unsafe {
            glBindRenderbuffer(GL_RENDERBUFFER,self.displayRenderbuffer.get());
        }
        sharedImageProcessingContext.presentBufferForDisplay();

    }

    fn activateDisplayFramebuffer(&self) {
        unsafe {
            glBindBuffer(GL_FRAMEBUFFER,self.displayRenderbuffer.get());
            glViewport(0,0,self.backingSize.get().width,self.backingSize.get().height);
        }
    }


    fn createDisplayFramebuffer(&self){
        unsafe {
            let mut frameBuffer : GLuint = 0;
            glGenFramebuffers(1,&mut frameBuffer);
            self.displayFramebuffer.set(frameBuffer);
            glBindFramebuffer(GL_FRAMEBUFFER, frameBuffer);


            let mut colorRenderBuffer : GLuint = 0;
            glGenRenderbuffers(1,&mut colorRenderBuffer);
            self.displayRenderbuffer.set(colorRenderBuffer);
            glBindRenderbuffer(GL_RENDERBUFFER,colorRenderBuffer);

            sharedImageProcessingContext.context.renderBufferStorage(GL_RENDERBUFFER as NSUInteger,&self.layer);


            let mut backingWidth : GLint = 0;
            let mut backingHeight : GLint = 0;

            glGetRenderbufferParameteriv(GL_RENDERBUFFER, GL_RENDERBUFFER_WIDTH, &mut backingWidth);
            glGetRenderbufferParameteriv(GL_RENDERBUFFER, GL_RENDERBUFFER_HEIGHT, &mut backingHeight);

            self.backingSize.set(GLSize::new(backingWidth,backingHeight));

            glFramebufferRenderbuffer(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0,GL_RENDERBUFFER, colorRenderBuffer);


        }
    }
}



#[allow(non_snake_case, unused_variables, dead_code)]
#[no_mangle]
pub extern "C" fn xhey_init_view(source: *mut c_void) -> *mut XHeyView{

    let _source = unsafe {transmute::<*mut c_void, Box<UIView>>(source)};
    let view = XHeyView::new(_source);
    unsafe {transmute(Box::new(view))}
}