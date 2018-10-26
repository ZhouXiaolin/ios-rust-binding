extern crate jni;
extern crate android_logger;

use super::operation::*;
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::*;
use std::os::raw::{c_void,c_int,c_uint,c_float};
use gles_rust_binding::*;
use super::render::{Framebuffer};
use super::common::Matrix4x4;
use super::structure::{Graph,Edge};
use super::render::sharedImageProcessingContext;

use log::Level;
use android_logger::Filter;





type RenderGraph<'a> = Graph<'a,Framebuffer>;
#[no_mangle]
pub extern "C" fn xhey_init_graph<'a>() -> *mut RenderGraph<'a> {
    let graph = Box::new(Graph::new());
    Box::into_raw(graph)
}

#[no_mangle]
pub unsafe extern "C" fn xhey_graph(graph: *mut RenderGraph,source: *mut XheyOESTexture, lookup_picture: *mut XheyPicture,lookup_filter: *mut XHeyLookupFilter, unsharpask: *mut XHeyUnsharpMaskFilter,surfaceView: *mut XheySurfaceView){
    let box_graph = graph.as_mut().unwrap();
    let box_texture = source.as_ref().unwrap();
    let box_lookup_picture = lookup_picture.as_ref().unwrap();
    let box_lookup_filter = lookup_filter.as_ref().unwrap();
    let box_unsharp_filter = unsharpask.as_ref().unwrap();
    let box_surfaceView = surfaceView.as_ref().unwrap();

    let texture = box_graph.add_input("texture",box_texture);
    let lookup_picture = box_graph.add_input("lookup picture",box_lookup_picture);
    let lookup_filter = box_graph.add_function("lookup filter",&[texture,lookup_picture],box_lookup_filter);
    let unsharp_mask = box_graph.add_function("unsharp mask",&[lookup_filter, texture],box_unsharp_filter);
    let view = box_graph.add_function("surface view",&[unsharp_mask],box_surfaceView);

}





//#[no_mangle]
//pub unsafe extern "C" fn xhey_picture_graph(graph: *mut RenderGraph, picture: *mut XheyPicture, lut: *mut XheyPicture, lut_filter: *mut XHeyLookupFilter, water_mark: *mut XheyPicture, blend_filter: *mut XHeyAlphaBlendFilter, output: *mut XheyPictureOutput) {
//    let box_graph = graph.as_mut().unwrap();
//    let box_picture = picture.as_mut().unwrap();
//    let box_lut = lut.as_mut().unwrap();
//    let box_lut_filter = lut_filter.as_mut().unwrap();
//    let box_water_mark = water_mark.as_mut().unwrap();
//    let box_alpha_blend = blend_filter.as_mut().unwrap();
//    let box_output = output.as_mut().unwrap();
//
//    let pic = box_graph.add_input("picture", box_picture);
//    let lut = box_graph.add_input("lut", box_lut);
//    let lut_filter = box_graph.add_function("lut filter",&[pic, lut], box_lut_filter);
//    let water_mark = box_graph.add_input("water mark",box_water_mark);
//    let alpha_blend = box_graph.add_function("alpha blend",&[lut_filter,water_mark],box_alpha_blend);
//    let output = box_graph.add_function("output",&[alpha_blend], box_output);
//}


#[no_mangle]
pub unsafe extern "C" fn xhey_picture_graph(graph: *mut RenderGraph, picture: *mut XheyPicture, lut: *mut XheyPicture, lut_filter: *mut XHeyLookupFilter, water_mask: *mut XHeyBlendFilter, output: *mut XheyPictureOutput) {
    let box_graph = graph.as_mut().unwrap();
    let box_picture = picture.as_mut().unwrap();
    let box_lut = lut.as_mut().unwrap();
    let box_lut_filter = lut_filter.as_mut().unwrap();

//    let box_water_mark = water_mark.as_mut().unwrap();
//    let box_alpha_blend = blend_filter.as_mut().unwrap();

    let box_water_mask = water_mask.as_mut().unwrap();

    let box_output = output.as_mut().unwrap();

    let pic = box_graph.add_input("picture", box_picture);
    let lut = box_graph.add_input("lut", box_lut);
    let lut_filter = box_graph.add_function("lut filter",&[pic, lut], box_lut_filter);
//    let water_mark = box_graph.add_input("water mark",box_water_mark);
//    let alpha_blend = box_graph.add_function("alpha blend",&[lut_filter,water_mark],box_alpha_blend);
    let water_mask = box_graph.add_function("water mask",&[lut_filter],box_water_mask);

    let output = box_graph.add_function("output",&[water_mask], box_output);
}


#[no_mangle]
pub unsafe extern "C" fn xhey_graph_forward<'a>(graph: *mut RenderGraph<'a>){
    let box_graph = graph.as_mut().unwrap();
    box_graph.forward();
}

#[no_mangle]
pub extern "C" fn xhey_init_oes_texture(width: c_int, height: c_int, orient: c_int) -> *mut XheyOESTexture {
    let texture = Box::new(XheyOESTexture::new(width,height,orient));
    Box::into_raw(texture)
}

#[no_mangle]
pub unsafe extern "C" fn xhey_init_alpha_blend() -> *mut XHeyAlphaBlendFilter {
    let filter = Box::new(XHeyAlphaBlendFilter::new());
    Box::into_raw(filter)
}


#[no_mangle]
pub unsafe extern "C" fn xhey_init_watermark() -> *mut XHeyBlendFilter {
    let filter = Box::new(XHeyBlendFilter::new());
    Box::into_raw(filter)
}


#[no_mangle]
pub unsafe extern "C" fn xhey_watermark_update(filter: *mut XHeyBlendFilter, texId: c_uint, x: c_float, y: c_float, w: c_float, h: c_float){
    let filter = filter.as_mut().unwrap();
    filter.appendWaterMark(texId,x,y,w,h);
}

#[no_mangle]
pub unsafe extern "C" fn xhey_init_unsharp_mask() -> *mut XHeyUnsharpMaskFilter {
    let filter = Box::new(XHeyUnsharpMaskFilter::new());
    Box::into_raw(filter)
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
pub unsafe extern "C" fn xhey_init_surface_view(width: i32, height: i32) -> *mut XheySurfaceView {
    let surfaceView = Box::new(XheySurfaceView::new(width,height));
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
pub extern "C" fn xhey_init_picture_textureId(textureId: i32, width: i32, height: i32, orient: i32) ->  *mut XheyPicture {
    info!("-----> xhey_init_picture {} {}",width,height);

    let picture = XheyPicture::new_texture(textureId as GLuint,width,height,orient);
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
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_initPicturetexture(env: JNIEnv, _: JClass, textureId: jint, width: jint, height: jint, orient: jint) -> jlong {
    xhey_init_picture_textureId(textureId,width,height,orient) as jlong
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
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_updatePictureorientation(env: JNIEnv, _: JClass, picture_ptr: jlong, orient: jint) {

    let picture = picture_ptr as *mut XheyPicture;
    let picture = picture.as_mut().unwrap();

    picture.updateOrientation(orient)

}



#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_initGraph(env: JNIEnv, _: JClass) -> jlong{
    xhey_init_graph() as jlong

}
#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_initTexture(env: JNIEnv, _: JClass, width: jint, height: jint, orient: jint) -> jlong{
    xhey_init_oes_texture(width,height,orient) as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_initBasicfilter(env: JNIEnv, _: JClass) -> jlong{
    xhey_init_basic_filter() as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_initSurfaceview(env: JNIEnv, _: JClass, width: jint, height: jint) -> jlong{
    xhey_init_surface_view(width,height) as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_updateTexture(env: JNIEnv, _: JClass, texture_ptr: jlong, textureId: jint){
    xhey_oes_texture_update(texture_ptr as *mut XheyOESTexture,textureId as c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_updateTexturematrix(env: JNIEnv, _: JClass, texture_ptr: jlong, matrix: jfloatArray) {
    let mut array : [jfloat;16] = [0.0;16];
    let _ = env.get_float_array_region(matrix,0,&mut array);
    let texture = (texture_ptr as *mut XheyOESTexture).as_mut().unwrap();
    texture.updateMatrix(Matrix4x4::new(array));

}

#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_graphForward(env: JNIEnv, _: JClass, graph_ptr: jlong){
    xhey_graph_forward(graph_ptr as *mut RenderGraph);
}


#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_graphConfig(env: JNIEnv, _: JClass, graph_ptr: jlong, texture_ptr: jlong,lookup_picture_ptr: jlong, lookup_filter_ptr: jlong, unsharp_mask_filter_ptr: jlong,view_ptr: jlong){
    xhey_graph(graph_ptr as *mut RenderGraph, texture_ptr as *mut XheyOESTexture, lookup_picture_ptr as *mut XheyPicture,lookup_filter_ptr as *mut XHeyLookupFilter,unsharp_mask_filter_ptr as *mut XHeyUnsharpMaskFilter,view_ptr as *mut XheySurfaceView);
}


//#[no_mangle]
//pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_graphPictureconfig(env: JNIEnv, _: JClass, graph_ptr: jlong, picture_ptr: jlong, lut_ptr: jlong, lut_filter_ptr: jlong, water_mark_ptr: jlong, blend_filter_ptr:jlong,output_ptr:jlong) {
//    xhey_picture_graph(graph_ptr as *mut RenderGraph, picture_ptr as *mut XheyPicture, lut_ptr as *mut XheyPicture, lut_filter_ptr as *mut XHeyLookupFilter, water_mark_ptr as *mut XheyPicture, blend_filter_ptr as *mut XHeyAlphaBlendFilter, output_ptr as *mut XheyPictureOutput);
//}

#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_graphPictureconfig(env: JNIEnv, _: JClass, graph_ptr: jlong, picture_ptr: jlong, lut_ptr: jlong, lut_filter_ptr: jlong, water_mark_ptr: jlong, output_ptr:jlong) {
    xhey_picture_graph(graph_ptr as *mut RenderGraph, picture_ptr as *mut XheyPicture, lut_ptr as *mut XheyPicture, lut_filter_ptr as *mut XHeyLookupFilter, water_mark_ptr as *mut XHeyBlendFilter,  output_ptr as *mut XheyPictureOutput);
}

#[no_mangle]
pub extern "C" fn xhey_init_lookup_filter() -> *mut XHeyLookupFilter {

    let filter = Box::new(XHeyLookupFilter::new());
    Box::into_raw(filter)
}

#[no_mangle]
pub extern "C" fn xhey_init_picture_output(width: i32, height: i32, orient: i32) -> *mut XheyPictureOutput {
    let output = Box::new(XheyPictureOutput::new(width, height, orient));
    Box::into_raw(output)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_initLookupfilter(env: JNIEnv, _: JClass) -> jlong{
    xhey_init_lookup_filter() as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_initAlphablendfilter(env: JNIEnv, _: JClass) -> jlong{
    xhey_init_alpha_blend() as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_initUnsharpmaskfilter(env: JNIEnv, _: JClass) -> jlong {
    xhey_init_unsharp_mask() as jlong
}
#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_initWatermark(env: JNIEnv, _: JClass) -> jlong {
    xhey_init_watermark() as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_appendWatermark(env: JNIEnv, _: JClass, watermark: jlong, texId: jint, x: jfloat, y: jfloat, w: jfloat, h: jfloat)  {
    let filter = watermark as *mut XHeyBlendFilter;
    let filter = filter.as_mut().unwrap();
    filter.appendWaterMark(texId as u32,x,y,w,h);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_initPictureoutput(env: JNIEnv, _: JClass, width: jint, height: jint, orient: jint) -> jlong{
    xhey_init_picture_output(width, height, orient) as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_releaseGraph(env: JNIEnv, _: JClass, graph_ptr: jlong) {

    drop(Box::from_raw(graph_ptr as *mut RenderGraph));

}

#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_releaseTexture(env: JNIEnv, _: JClass, texture_ptr: jlong) {
    drop(Box::from_raw(texture_ptr as *mut XheyOESTexture))
}


#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_releaseSurfaceview(env: JNIEnv, _: JClass, surface_view_ptr: jlong) {
    drop(Box::from_raw(surface_view_ptr as *mut XheySurfaceView))
}


#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_releaseBasicfilter(env: JNIEnv, _: JClass, basic_filter_ptr: jlong) {
    drop(Box::from_raw(basic_filter_ptr as *mut XHeyBasicFilter))
}


#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_releasePicture(env: JNIEnv, _: JClass, picture_ptr: jlong) {
    drop(Box::from_raw(picture_ptr as *mut XheyPicture))
}


#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_releaseLookupfilter(env: JNIEnv, _: JClass, lookup_filter_ptr: jlong) {
    drop(Box::from_raw(lookup_filter_ptr as *mut XHeyLookupFilter))
}


#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_releaseAlphablendfilter(env: JNIEnv, _: JClass, filter_ptr: jlong) {
    drop(Box::from_raw(filter_ptr as *mut XHeyAlphaBlendFilter))
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_releaseUnsharpmaskfilter(env: JNIEnv, _: JClass, filter_ptr: jlong){
    drop(Box::from_raw(filter_ptr as *mut XHeyUnsharpMaskFilter))
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_releaseWatermarkfilter(env: JNIEnv, _: JClass, filter_ptr: jlong){
    drop(Box::from_raw(filter_ptr as *mut XHeyBlendFilter))
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_xcamera_camera_GPUImage_releaseContext(env: JNIEnv, _: JClass){
    sharedImageProcessingContext.framebufferCache.purgeAllUnassignedFramebuffer();
}

