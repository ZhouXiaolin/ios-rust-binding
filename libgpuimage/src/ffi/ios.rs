use super::operation::*;
use super::common::structure::*;

use ios_rust_binding::UIView;
use std::os::raw::{c_char,c_void};
use std::ffi::{CStr};
use std::mem::transmute;
use ios_rust_binding::UIImage;


#[no_mangle]
pub extern "C" fn xhey_add_target<'a>(source: *mut XheyPicture<'a>, filter: *mut XHeyBasicFilter<'a>, filter2: *mut XHeyBasicFilter<'a>, consumer: *mut XHeyView){
    let box_picture = unsafe{source.as_ref().unwrap()};
    let box_filter = unsafe{filter.as_ref().unwrap()};
    let box_filter2 = unsafe{filter2.as_ref().unwrap()};


    let box_view = unsafe{consumer.as_ref().unwrap()};
    box_picture.addTarget(box_filter,0);
    box_filter.addTarget(box_filter2,0);
    box_filter2.addTarget(box_view,0);
}

use super::common::Graph;

#[no_mangle]
pub extern "C" fn xhey_init_graph<'a>() -> *mut Graph<'a> {
    let graph = Box::new(Graph::new());
    Box::into_raw(graph)
}
#[no_mangle]
pub unsafe extern "C" fn xhey_graph<'a>(graph: *mut Graph<'a>,source: *mut XheyPicture<'a>,filter: *mut XHeyBasicFilter<'a>, filter2: *mut XHeyBasicFilter<'a>, view: *mut XHeyView){
    let box_graph = graph.as_mut().unwrap();

    let box_picture = source.as_ref().unwrap();
    let box_view = view.as_ref().unwrap();
    let box_filter = filter.as_ref().unwrap();
    let box_filter2 = filter2.as_ref().unwrap();

    let pic = box_graph.placeholder("picture",box_picture);
    let filter1 = box_graph.add_function("filter1",&[pic],box_filter);
    let filter2 = box_graph.add_function("filter2",&[filter1],box_filter2);
    let vi = box_graph.add_function("view",&[filter2],box_view);
    box_graph.PrintGraphviz();
    box_graph.forward();

}

#[no_mangle]
pub extern "C" fn xhey_init_basic_filter<'a>() -> *mut XHeyBasicFilter<'a> {
    let filter = Box::new(XHeyBasicFilter::new());
    Box::into_raw(filter)
}

#[no_mangle]
pub extern "C" fn xhey_init_basic_filter_2<'a>() -> *mut XHeyBasicFilter<'a> {

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
     gl_FragColor = vec4(0.0,color.r, 0.0, 1.0);
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
pub extern "C" fn xhey_init_picture<'a>(data: *const c_void, width: i32, height: i32) ->  *mut XheyPicture<'a> {
    println!("xhey_init_picture");
    let picture = Box::new(XheyPicture::new(data,width,height));
    Box::into_raw(picture)

}

#[no_mangle]
pub extern "C" fn xhey_process_picture(picture: *const XheyPicture){
    let p = unsafe{picture.as_ref().unwrap()};
    p.processImage();


}


#[no_mangle]
pub extern "C" fn xhey_init_camera<'a>() -> *mut XheyCamera<'a> {
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