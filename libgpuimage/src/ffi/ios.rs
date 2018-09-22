use super::operation::*;

use ios_rust_binding::UIView;
use std::os::raw::{c_char,c_void};
use std::ffi::{CStr};
use std::mem::transmute;
use ios_rust_binding::UIImage;


use super::common::Graph;

#[no_mangle]
pub extern "C" fn xhey_init_graph<'a>() -> *mut Graph<'a> {
    let graph = Box::new(Graph::new());
    Box::into_raw(graph)
}
#[no_mangle]
pub unsafe extern "C" fn xhey_graph<'a>(graph: *mut Graph<'a>,source: *mut XheyPicture,filter: *mut XHeyBasicFilter, filter2: *mut XHeyBasicFilter,filter3: *mut XHeyBasicFilter, view: *mut XHeyView){
    let box_graph = graph.as_mut().unwrap();

    let box_picture = source.as_ref().unwrap();
    let box_view = view.as_ref().unwrap();
    let box_filter = filter.as_ref().unwrap();
    let box_filter2 = filter2.as_ref().unwrap();
    let box_filter3 = filter3.as_ref().unwrap();

    let pic = box_graph.add_input("picture",box_picture);
    let filter1 = box_graph.add_function("filter1",&[pic],box_filter);
    let filter2 = box_graph.add_function("filter2",&[filter1],box_filter2);
    let filter3 = box_graph.add_function("filter3",&[filter2],box_filter3);
    let vi = box_graph.add_function("view",&[filter3],box_view);
    box_graph.PrintGraphviz();
    box_graph.forward();
    box_graph.forward();
//    box_graph.forward();
//    box_graph.forward();
//    box_graph.forward();
//    box_graph.forward();
//    box_graph.forward();
//    box_graph.forward();


}

#[no_mangle]
pub extern "C" fn xhey_init_basic_filter() -> *mut XHeyBasicFilter {
    let filter = Box::new(XHeyBasicFilter::new());
    Box::into_raw(filter)
}

#[no_mangle]
pub extern "C" fn xhey_init_basic_filter_2() -> *mut XHeyBasicFilter {

    let vertexString = r#"
 attribute vec4 position;
 attribute vec4 inputTextureCoordinate;

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
     gl_FragColor = vec4(color.r,0.0, 0.0, 1.0);
 }
    "#;

    let filter = Box::new(XHeyBasicFilter::new_shader(vertexString,fragmentString,1));
    Box::into_raw(filter)
}


#[no_mangle]
pub extern "C" fn xhey_init_view(source: *const UIView) -> *mut XHeyView{
    let _source = unsafe{source.as_ref().unwrap()};
    let view = XHeyView::new(_source);
    Box::into_raw(Box::new(view))

}


#[no_mangle]
pub extern "C" fn xhey_init_picture(data: *const c_void, width: i32, height: i32) ->  *mut XheyPicture {
    println!("xhey_init_picture");
    let picture = Box::new(XheyPicture::new(data,width,height));
    Box::into_raw(picture)

}

#[no_mangle]
pub extern "C" fn xhey_process_picture(picture: *const XheyPicture){

}


#[no_mangle]
pub extern "C" fn xhey_init_camera() -> *mut XheyCamera {
    println!("xhey_init_camera");
    let camera = Box::new(XheyCamera::new());
    Box::into_raw(camera)
}


#[no_mangle]
pub extern "C" fn xhey_start_capture(camera: *mut XheyCamera){
    println!("xhey_start_camera");
}

#[no_mangle]
pub extern "C" fn xhey_stop_capture(camera: *mut XheyCamera){
    println!("xhey_start_camera");
}




#[no_mangle]
pub extern "C" fn test(path: *const c_char) -> *mut c_void{
    unsafe {
        let a =  CStr::from_ptr(path);
        let a = a.to_str().unwrap();
        let image = UIImage::get_image(a);
        transmute(image)
    }
}
