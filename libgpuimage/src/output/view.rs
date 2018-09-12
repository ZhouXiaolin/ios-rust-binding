use std::os::raw::{c_void};
use std::mem::transmute;
use ios_rust_binding::{UIView,NSUInteger,EAGLContext};

use gles_rust_binding::*;

use core::{Consumer,Source};
use core::{Node,NodeType};
#[repr(C)]
pub struct XHeyView {
}


impl Consumer for XHeyView {
    fn set_source<T : Source>(&self, _source: &T, _location: u32) {
        println!("XheyView set_source");

    }
}

impl Node for XHeyView {
    fn get_type_id() -> NodeType {
        NodeType::View
    }
}

impl XHeyView {
    fn new() -> Self{
        XHeyView{
        }
    }
}

#[allow(non_snake_case, unused_variables, dead_code)]
#[no_mangle]
pub extern "C" fn xhey_init_triangle() {
    let vertexStr = r#"
    attribute vec4 vPosition;

 void main(void)
{
    gl_Position = vPosition;
}
    "#;

    let fragmentStr = r#"
     precision mediump float;

 void main()
{
    gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
}

    "#;

    let program = GLProgram::new(vertexStr,fragmentStr);
    let position = program.get_attribute("vPosition");

    let vertices:[f32;6] = [
        -0.5,-0.5, 0.5,-0.5, 0.0,0.5 ];


    unsafe {
        glClearColor(0.0,1.0,0.0,1.0);
        glClear(GL_COLOR_BUFFER_BIT);

        glVertexAttribPointer(position.location() as u32,2,GL_FLOAT,GL_FALSE,0,vertices.as_ptr() as *const _);
        glEnableVertexAttribArray(position.location() as u32);

        glDrawArrays(GL_TRIANGLES,0,3);
    }

}

#[allow(non_snake_case, unused_variables, dead_code)]
#[no_mangle]
pub extern "C" fn xhey_init_view(source: *mut c_void, data: *const c_void, width: i32, height: i32) -> *mut XHeyView{



    let _source = unsafe {transmute::<*mut c_void, Box<UIView>>(source)};
    let layer = _source.get_layer();
    let layer = layer.share();
    let context = EAGLContext::with_api(2);
    let context = context.share();
    EAGLContext::set_current_context(&context);


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

    unsafe {

        let mut colorRenderBuffer : GLuint = 0;
        glGenRenderbuffers(1,&mut colorRenderBuffer);
        glBindRenderbuffer(GL_RENDERBUFFER,colorRenderBuffer);
        context.render_buffer_storage(GL_RENDERBUFFER as NSUInteger,&layer);


        let mut backingWidth : GLint = 0;
        let mut backingHeight : GLint = 0;

        glGetRenderbufferParameteriv(GL_RENDERBUFFER, GL_RENDERBUFFER_WIDTH, &mut backingWidth);
        glGetRenderbufferParameteriv(GL_RENDERBUFFER, GL_RENDERBUFFER_HEIGHT, &mut backingHeight);

        glViewport(0,0,backingWidth,backingHeight);


        let mut frameBuffer : GLuint = 0;
        glGenFramebuffers(1,&mut frameBuffer);
        glBindFramebuffer(GL_FRAMEBUFFER, frameBuffer);
        glFramebufferRenderbuffer(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0,GL_RENDERBUFFER, colorRenderBuffer);


        let program = GLProgram::new(vertexStr,fragmentStr);

        let mut imageTexture : GLuint = 0;
        glGenTextures(1,&mut imageTexture);
        glBindTexture(GL_TEXTURE_2D,imageTexture);
        glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_MIN_FILTER, GL_LINEAR as i32);
        glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_MAG_FILTER, GL_LINEAR as i32);
        glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE as i32);
        glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE as i32);
        glTexImage2D(GL_TEXTURE_2D,0,GL_RGBA as i32,width,height,0,GL_BGRA,GL_UNSIGNED_BYTE,data as *const _);


        let position = program.get_attribute("position");
        let textureCoordinate = program.get_attribute("inputTextureCoordinate");
        let inputTexture = program.get_uniform("inputImageTexture");


        let vertices:[f32;8] = [-1.0,1.0,1.0,1.0,-1.0,-1.0,1.0,-1.0];

        let textureCoordinates:[f32;8] = [1.0,1.0, 1.0,0.0, 0.0,1.0, 0.0,0.0];

        glClearColor(1.0,0.0,0.0,1.0);
        glClear(GL_COLOR_BUFFER_BIT);

        println!("backingWidth:{} backingHeight:{}",backingWidth, backingHeight);

        glVertexAttribPointer(position.location() as u32,2,GL_FLOAT,GL_FALSE,0,vertices.as_ptr() as *const _);
        glEnableVertexAttribArray(position.location() as u32);
        glVertexAttribPointer(textureCoordinate.location() as u32,2,GL_FLOAT,GL_FALSE,0,textureCoordinates.as_ptr() as *const _);
        glEnableVertexAttribArray(textureCoordinate.location() as u32);

        glActiveTexture(GL_TEXTURE0);
        glBindTexture(GL_TEXTURE_2D,imageTexture);
        glUniform1i(0,inputTexture.location() as i32);

        glDrawArrays(GL_TRIANGLE_STRIP,0,4);
        context.present_render_buffer(GL_FRAMEBUFFER as NSUInteger);

        glDeleteTextures(0,&imageTexture);
        transmute(Box::new(XHeyView::new()))
    }
}