use std::os::raw::c_void;
use std::mem::transmute;

use ios_rust_binding::*;
use ios_rust_binding::runtime::{Class,Object,Sel};
use ios_rust_binding::runtime::{BOOL,YES,NO};
use ios_rust_binding::declare::ClassDecl;
use ios_rust_binding::rc::StrongPtr;
use ios_rust_binding::{CALayer,UIView,UIColor,ShareId,NSUInteger,EAGLContext};

use gles_rust_binding::*;

use core::{DataConsumer,DataSource};

#[repr(C)]
pub struct XHeyView {}


impl DataConsumer for XHeyView {
    fn set_source<T : DataSource>(&self, _source: &T, _location: u32) {
        println!("XheyView set_source");

    }
}

impl XHeyView {
    fn new() -> Self{
        XHeyView{}
    }
}


#[allow(non_snake_case, unused_variables, dead_code)]
#[no_mangle]
pub extern "C" fn xhey_init_view(source: *mut c_void) -> *mut XHeyView{



    let _source = unsafe {transmute::<*mut c_void, Box<UIView>>(source)};
    let layer = _source.get_layer();
    let layer = layer.share();
    let context = EAGLContext::with_api(kEAGLRenderingAPIOpenGLES2);
    let context = context.share();
    EAGLContext::set_current_context(&context);



    unsafe {



        let mut colorRenderBuffer : GLuint = 0;
        glGenRenderbuffers(1,&mut colorRenderBuffer);
        glBindRenderbuffer(GL_RENDERBUFFER,colorRenderBuffer);


        context.render_buffer_storage(GL_RENDERBUFFER as NSUInteger,&layer);


        let mut frameBuffer : GLuint = 0;
        glGenFramebuffers(1,&mut frameBuffer);
        glBindFramebuffer(GL_FRAMEBUFFER, frameBuffer);
        glFramebufferRenderbuffer(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0,GL_RENDERBUFFER, colorRenderBuffer);

        glClearColor(1.0,1.0,0.0,1.0);
        glClear(GL_COLOR_BUFFER_BIT);


        context.present_render_buffer(GL_FRAMEBUFFER as NSUInteger);

        transmute(Box::new(XHeyView::new()))
    }
}