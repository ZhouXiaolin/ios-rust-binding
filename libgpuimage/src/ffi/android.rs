extern crate jni;
extern crate gles_rust_binding;
extern crate android_logger;

use super::operation::*;
use self::jni::JNIEnv;
use self::jni::objects::{JClass, JString};
use self::jni::sys::{jint, jlong};
use super::std::os::raw::{c_void,c_int,c_uint};
use self::gles_rust_binding::*;
use super::render::{Framebuffer};
use super::structure::{Graph,Edge};


use super::log::Level;
use self::android_logger::Filter;





type RenderGraph<'a> = Graph<'a,Framebuffer>;
#[no_mangle]
pub extern "C" fn xhey_init_graph<'a>() -> *mut RenderGraph<'a> {
    let graph = Box::new(Graph::new());
    Box::into_raw(graph)
}

#[no_mangle]
pub unsafe extern "C" fn xhey_graph<'a>(graph: *mut RenderGraph<'a>,source: *mut XheyOESTexture){
    let box_graph = graph.as_mut().unwrap();
    let box_texture = source.as_ref().unwrap();
    let pic = box_graph.add_input("picture",box_texture);

}

#[no_mangle]
pub unsafe extern "C" fn xhey_graph_forward<'a>(graph: *mut RenderGraph<'a>){
    let box_graph = graph.as_mut().unwrap();
    box_graph.forward();
}

#[no_mangle]
pub extern "C" fn xhey_init_oes_texture(width: c_int, height: c_int) -> *mut XheyOESTexture {
    let texture = Box::new(XheyOESTexture::new(width,height));
    Box::into_raw(texture)
}

#[no_mangle]
pub extern "C" fn xhey_oes_texture_update(texture: *mut XheyOESTexture, textureId: c_uint){
    let t = unsafe{texture.as_mut().unwrap()};
    t.update(textureId);
}

#[no_mangle]
pub unsafe extern "C" fn xhey_init_basic_filter() -> *mut XHeyBasicFilter {
    let filter = Box::new(XHeyBasicFilter::new());
    Box::into_raw(filter)
}


#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_initLogger(env: JNIEnv, _: JClass){
    android_logger::init_once(
             Filter::default().with_min_level(Level::Trace),
             Some("solaren")
         );
}



#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_initGraph(env: JNIEnv, _: JClass) -> jlong{
    xhey_init_graph() as jlong

}
#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_initTexture(env: JNIEnv, _: JClass, width: jint, height: jint) -> jlong{
    xhey_init_oes_texture(width,height) as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_initBasicfilter(env: JNIEnv, _: JClass) -> jlong{
    xhey_init_basic_filter() as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_updateTexture(env: JNIEnv, _: JClass, texture_ptr: jlong, textureId: jint){
    xhey_oes_texture_update(texture_ptr as *mut XheyOESTexture,textureId as c_uint);
}


#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_graphForward(env: JNIEnv, _: JClass, graph_ptr: jlong){
    xhey_graph_forward(graph_ptr as *mut RenderGraph);
}



#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_graphConfig(env: JNIEnv, _: JClass, graph_ptr: jlong, texture_ptr: jlong){
    xhey_graph(graph_ptr as *mut RenderGraph, texture_ptr as *mut XheyOESTexture);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_bindFramebuffer(env: JNIEnv, _: JClass){
//    glActiveTexture(GL_TEXTURE1);
//    glGenTextures(1, &mut texture);
//    glBindTexture(GL_TEXTURE_2D, texture);
//    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, textureOptions.minFilter as i32);
//    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, textureOptions.magFilter as i32);
//    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, textureOptions.wrapS as i32);
//    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, textureOptions.wrapT as i32);
//
//    glBindTexture(GL_TEXTURE_2D, 0);
}