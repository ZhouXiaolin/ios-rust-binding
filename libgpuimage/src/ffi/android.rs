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
pub unsafe extern "C" fn xhey_graph<'a>(graph: *mut RenderGraph<'a>,source: *mut XheyOESTexture, filter: *mut XHeyBasicFilter,surfaceView: *mut XheySurfaceView){
    let box_graph = graph.as_mut().unwrap();
    let box_texture = source.as_ref().unwrap();
    let box_filter = filter.as_ref().unwrap();
    let box_surfaceView = surfaceView.as_ref().unwrap();

    let texture = box_graph.add_input("texture",box_texture);
    let filter = box_graph.add_function("filter",&[texture],box_filter);
    let view = box_graph.add_function("surface view",&[filter],box_surfaceView);

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
pub unsafe extern "C" fn xhey_init_surface_view() -> *mut XheySurfaceView {
    let surfaceView = Box::new(XheySurfaceView::new());
    Box::into_raw(surfaceView)
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
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_initSurfaceview(env: JNIEnv, _: JClass) -> jlong{
    xhey_init_surface_view() as jlong
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
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_graphConfig(env: JNIEnv, _: JClass, graph_ptr: jlong, texture_ptr: jlong, filter_ptr: jlong,view_ptr: jlong){
    xhey_graph(graph_ptr as *mut RenderGraph, texture_ptr as *mut XheyOESTexture, filter_ptr as *mut XHeyBasicFilter,view_ptr as *mut XheySurfaceView);
}

