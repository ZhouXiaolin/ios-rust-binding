
use ios_rust_binding::{UIView,UIImage};
use std::os::raw::{c_char,c_void,c_uint,c_float,c_long};
use std::ffi::{CStr};
use std::mem::transmute;
use std::rc::Rc;

use super::structure::{Graph,Edge};
use super::render::{Framebuffer,sharedImageProcessingContext,GlContext};

use super::operation::*;
type RenderGraph<'a> = Graph<'a,Framebuffer>;
#[no_mangle]
pub extern "C" fn xhey_init_graph<'a>() -> *mut RenderGraph<'a> {
    let graph = Box::new(Graph::new());
    Box::into_raw(graph)
}

#[no_mangle]
pub unsafe extern "C" fn xhey_graph(graph: *mut RenderGraph,source: *mut XheyPicture, filter: *mut XHeyBasicFilter,filter2: *mut XHeyBasicFilter,surfaceView: *mut XheyPictureOutput){
    let box_graph = graph.as_mut().unwrap();
    let box_texture = source.as_ref().unwrap();
    let box_filter = filter.as_ref().unwrap();
//    let box_filter2 = filter2.as_ref().unwrap();
    let box_surfaceView = surfaceView.as_ref().unwrap();

    let texture = box_graph.add_input("texture",box_texture);
    let filter = box_graph.add_function("filter",&[texture],box_filter);
//    let filter2 = box_graph.add_function("filter2",&[filter],box_filter2);
    let view = box_graph.add_function("surface view",&[filter],box_surfaceView);

}

#[no_mangle]
pub extern "C" fn xhey_init_picture_output(width: i32, height: i32) -> *mut XheyPictureOutput {
    let output = Box::new(XheyPictureOutput::new(width, height, 0));
    Box::into_raw(output)
}

#[no_mangle]
pub unsafe extern "C" fn xhey_picture_output_get_texture_id(source: *mut XheyPictureOutput) -> u32 {
    let box_output = source.as_ref().unwrap();
    box_output.textureId()
}

#[no_mangle]
pub extern "C" fn xhey_context_release(){
    sharedImageProcessingContext.framebufferCache.purgeAllUnassignedFramebuffer();
}

#[no_mangle]
pub unsafe extern "C" fn xhey_release_picture(source: *mut XheyPicture){
    drop(Box::from_raw(source));
}


#[no_mangle]
pub unsafe extern "C" fn xhey_release_basic_filter(source: *mut XHeyBasicFilter){
    drop(Box::from_raw(source));
}

#[no_mangle]
pub unsafe extern "C" fn xhey_release_combine_filter(source: *mut XHeyCombineFilter){
    drop(Box::from_raw(source));
}

#[no_mangle]
pub unsafe extern "C" fn xhey_release_graph(s: *mut RenderGraph){
    drop(Box::from_raw(s));
}

#[no_mangle]
pub unsafe extern "C" fn xhey_graph_forward(graph: *mut RenderGraph){
    let box_graph = graph.as_mut().unwrap();
    box_graph.forward();
}

#[no_mangle]
pub unsafe extern "C" fn xhey_graph_printgraphviz(graph: *mut RenderGraph){
    let box_graph = graph.as_mut().unwrap();
    box_graph.PrintGraphviz();
}


#[no_mangle]
pub extern "C" fn xhey_init_basic_filter() -> *mut XHeyBasicFilter {
    let filter = Box::new(XHeyBasicFilter::new());
    Box::into_raw(filter)
}

#[no_mangle]
pub extern "C" fn xhey_update_outputSize(filter: *mut XHeyBasicFilter, width: i32, height: i32){
    let combine = unsafe{filter.as_mut().unwrap()};
    combine.updateOutputSize(width,height);
}

#[no_mangle]
pub extern "C" fn xhey_update_outputRotation(filter: *mut XHeyBasicFilter, rotation: i32){
    let combine = unsafe{filter.as_mut().unwrap()};
    combine.updateOutputRotation(rotation);
}


#[no_mangle]
pub extern "C" fn xhey_init_combine_filter() -> *mut XHeyCombineFilter {
    let filter = Box::new(XHeyCombineFilter::new());
    Box::into_raw(filter)
}


#[no_mangle]
pub extern "C" fn xhey_combine_value(filter: *mut XHeyCombineFilter, value: c_float){
    let combine = unsafe{filter.as_mut().unwrap()};
    combine.set_value(value);
}

#[no_mangle]
pub extern "C" fn xhey_init_basic_filter_2() -> *mut XHeyBasicFilter {

    let vertexString = r#"
 attribute vec4 position;
 attribute vec2 inputTextureCoordinate;

 varying vec2 textureCoordinate;

 void main()
 {
     gl_Position = position;
     textureCoordinate = inputTextureCoordinate.xy;
 }
    "#;

    let fragmentString = r#"
 precision mediump float;

 varying highp vec2 textureCoordinate;
 uniform sampler2D inputImageTexture;

 void main()
 {
     vec4 color = texture2D(inputImageTexture, textureCoordinate);
     gl_FragColor = vec4(color.r, 0.0, 0.0, 1.0);
 }
    "#;

    let filter = Box::new(XHeyBasicFilter::new_shader(vertexString,fragmentString,1));
    Box::into_raw(filter)
}

#[no_mangle]
pub extern "C" fn xhey_init_lookup_filter() -> *mut XHeyLookupFilter {

    let filter = Box::new(XHeyLookupFilter::new());
    Box::into_raw(filter)
}
//#[no_mangle]
//pub extern "C" fn xhey_init_view(source: *const UIView) -> *mut XHeyView{
//    let _source = unsafe{source.as_ref().unwrap()};
//    let view = XHeyView::new(_source);
//    Box::into_raw(Box::new(view))
//
//}

#[no_mangle]
pub unsafe extern "C" fn xhey_init_surface_view(width: i32, height: i32) -> *mut XheySurfaceView {
    let surfaceView = Box::new(XheySurfaceView::new(width,height));
    Box::into_raw(surfaceView)
}
#[no_mangle]
pub unsafe extern "C" fn xhey_release_surface_view(source: *mut XheySurfaceView){
    drop(Box::from_raw(source));
}


#[no_mangle]
pub extern "C" fn xhey_init_picture(data: *const c_void, width: i32, height: i32) ->  *mut XheyPicture {
    println!("xhey_init_picture");
    let picture = Box::new(XheyPicture::new(data,width,height));
    Box::into_raw(picture)

}


#[no_mangle]
pub extern "C" fn xhey_update_picture(picture: *const XheyPicture, data: *const c_void, width: i32, height: i32){
    let picture = unsafe{picture.as_ref().unwrap()};
    picture.update(data,width,height);
}

#[no_mangle]
pub extern "C" fn xhey_init_glcontext() -> *mut GlContext {
    Box::into_raw(Box::new(GlContext::new()))
}


#[no_mangle]
pub extern "C" fn test(path: *const c_char){
    unsafe {
        let a =  CStr::from_ptr(path);
        let a = a.to_str().unwrap();
    }
}

// new api

