extern crate jni;
extern crate android_logger;

use super::operation::*;
use self::jni::JNIEnv;
use self::jni::objects::{JClass, JString};
use self::jni::sys::*;
use std::os::raw::{c_void,c_int,c_uint};
use gles_rust_binding::*;
use super::render::{Framebuffer};
use super::structure::{Graph,Edge};


use log::Level;
use android_logger::Filter;





type RenderGraph<'a> = Graph<'a,Framebuffer>;
#[no_mangle]
pub extern "C" fn xhey_init_graph<'a>() -> *mut RenderGraph<'a> {
    let graph = Box::new(Graph::new());
    Box::into_raw(graph)
}

#[no_mangle]
pub unsafe extern "C" fn xhey_graph<'a>(graph: *mut RenderGraph<'a>,source: *mut XheyOESTexture, lookup_picture: *mut XheyPicture,lookup_filter: *mut XHeyLookupFilter,surfaceView: *mut XheySurfaceView){
    let box_graph = graph.as_mut().unwrap();
    let box_texture = source.as_ref().unwrap();
    let box_lookup_picture = lookup_picture.as_ref().unwrap();
    let box_lookup_filter = lookup_filter.as_ref().unwrap();
    let box_surfaceView = surfaceView.as_ref().unwrap();

    let texture = box_graph.add_input("texture",box_texture);
    let lookup_picture = box_graph.add_input("lookup picture",box_lookup_picture);
    let lookup_filter = box_graph.add_function("lookup filter",&[texture,lookup_picture],box_lookup_filter);
    let view = box_graph.add_function("surface view",&[lookup_filter],box_surfaceView);

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
pub extern "C" fn xhey_init_picture(data: *const c_void, width: i32, height: i32) ->  *mut XheyPicture {
    info!("-----> xhey_init_picture {} {}",width,height);
    let picture = XheyPicture::new(data,width,height);
    let picture = Box::new(picture);
    Box::into_raw(picture)

}

#[no_mangle]
pub extern "C" fn xhey_init_picture_textureId(textureId: i32, width: i32, height: i32) ->  *mut XheyPicture {
    info!("-----> xhey_init_picture {} {}",width,height);

    let picture = XheyPicture::new_texture(textureId as GLuint,width,height);
    let picture = Box::new(picture);
    Box::into_raw(picture)

}

#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_initLogger(env: JNIEnv, _: JClass){
    android_logger::init_once(
             Filter::default().with_min_level(Level::Trace),
             Some("solaren")
         );
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_initPicture(env: JNIEnv, _: JClass, data: jbyteArray, width: jint, height: jint) -> jlong {
    let buf_pic = env.convert_byte_array(data).unwrap();

    info!("hello picture width {} height {} count: {}",width, height, buf_pic.len());

    xhey_init_picture(buf_pic.as_ptr() as *const _, width, height) as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_initPicturetexture(env: JNIEnv, _: JClass, textureId: jint, width: jint, height: jint) -> jlong {
    xhey_init_picture_textureId(textureId,width,height) as jlong
}



#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_updatePicture(env: JNIEnv, _: JClass, picture_ptr: jlong, data: jbyteArray, width: jint, height: jint) {

    let picture = picture_ptr as *mut XheyPicture;
    let picture = picture.as_ref().unwrap();
    let buf_pic = env.convert_byte_array(data).unwrap();

    info!("hello picture width {} height {}",width, height);

    picture.update(buf_pic.as_ptr() as *const _, width, height);
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
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_graphConfig(env: JNIEnv, _: JClass, graph_ptr: jlong, texture_ptr: jlong,lookup_picture_ptr: jlong, filter_ptr: jlong,view_ptr: jlong){
    xhey_graph(graph_ptr as *mut RenderGraph, texture_ptr as *mut XheyOESTexture, lookup_picture_ptr as *mut XheyPicture,filter_ptr as *mut XHeyLookupFilter,view_ptr as *mut XheySurfaceView);
}




#[no_mangle]
pub extern "C" fn xhey_init_lookup_filter() -> *mut XHeyLookupFilter {

    let filter = Box::new(XHeyLookupFilter::new());
    Box::into_raw(filter)
}


#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_initLookupfilter(env: JNIEnv, _: JClass) -> jlong{
    xhey_init_lookup_filter() as jlong
}