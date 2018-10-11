
use super::ios_rust_binding::{UIView,UIImage};
use super::std::os::raw::{c_char,c_void,c_uint,c_float};
use super::std::ffi::{CStr};
use super::std::mem::transmute;
use super::std::rc::Rc;

use super::structure::{Graph,Edge};
use super::render::{Framebuffer,sharedImageProcessingContext};
use super::operation::{XheyPicture,XHeyBasicFilter,XHeyView,XHeyCombineFilter};
type RenderGraph<'a> = Graph<'a,Framebuffer>;
#[no_mangle]
pub extern "C" fn xhey_init_graph<'a>() -> *mut RenderGraph<'a> {
    let graph = Box::new(Graph::new());
    Box::into_raw(graph)
}
#[no_mangle]
pub unsafe extern "C" fn xhey_graph<'a>(graph: *mut RenderGraph<'a>,source: *mut XheyPicture ,filter: *mut XHeyBasicFilter, filter2: *mut XHeyBasicFilter,filter3: *mut XHeyCombineFilter, view: *mut XHeyView){
    let box_graph = graph.as_mut().unwrap();

    let box_picture = source.as_ref().unwrap();
    let box_view = view.as_ref().unwrap();
    let box_filter = filter.as_ref().unwrap();
    let box_filter2 = filter2.as_ref().unwrap();
    let combine = filter3.as_ref().unwrap();

    let pic = box_graph.add_input("picture",box_picture);
    let filter1 = box_graph.add_function("filter1",&[pic],box_filter);
    let filter2 = box_graph.add_function("filter2",&[pic],box_filter2);
    let filter3 = box_graph.add_function("filter3",&[filter1,filter2],combine);
    let vi = box_graph.add_function("view",&[filter3],box_view);


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
pub unsafe extern "C" fn xhey_release_view(source: *mut XHeyView){
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
pub unsafe extern "C" fn xhey_graph_forward<'a>(graph: *mut RenderGraph<'a>){
    let box_graph = graph.as_mut().unwrap();
    box_graph.forward();
}

#[no_mangle]
pub unsafe extern "C" fn xhey_graph_printgraphviz<'a>(graph: *mut RenderGraph<'a>){
    let box_graph = graph.as_mut().unwrap();
    box_graph.PrintGraphviz();
}


#[no_mangle]
pub extern "C" fn xhey_init_basic_filter() -> *mut XHeyBasicFilter {
    let filter = Box::new(XHeyBasicFilter::new());
    Box::into_raw(filter)
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

type XheyLookupFilter = XHeyBasicFilter;
#[no_mangle]
pub extern "C" fn xhey_init_lookup_filter() -> *mut XheyLookupFilter {

    let fragmentString = r#"
 varying highp vec2 textureCoordinate;
 varying highp vec2 textureCoordinate2; // TODO: This is not used

 uniform sampler2D inputImageTexture;
 uniform sampler2D inputImageTexture2; // lookup texture

 const float intensity = 1.0;

 void main()
 {
     highp vec4 textureColor = texture2D(inputImageTexture, textureCoordinate);

     highp float blueColor = textureColor.b * 63.0;

     highp vec2 quad1;
     quad1.y = floor(floor(blueColor) / 8.0);
     quad1.x = floor(blueColor) - (quad1.y * 8.0);

     highp vec2 quad2;
     quad2.y = floor(ceil(blueColor) / 8.0);
     quad2.x = ceil(blueColor) - (quad2.y * 8.0);

     highp vec2 texPos1;
     texPos1.x = (quad1.x * 0.125) + 0.5/512.0 + ((0.125 - 1.0/512.0) * textureColor.r);
     texPos1.y = (quad1.y * 0.125) + 0.5/512.0 + ((0.125 - 1.0/512.0) * textureColor.g);

     highp vec2 texPos2;
     texPos2.x = (quad2.x * 0.125) + 0.5/512.0 + ((0.125 - 1.0/512.0) * textureColor.r);
     texPos2.y = (quad2.y * 0.125) + 0.5/512.0 + ((0.125 - 1.0/512.0) * textureColor.g);

     lowp vec4 newColor1 = texture2D(inputImageTexture2, texPos1);
     lowp vec4 newColor2 = texture2D(inputImageTexture2, texPos2);

     lowp vec4 newColor = mix(newColor1, newColor2, fract(blueColor));
     gl_FragColor = mix(textureColor, vec4(newColor.rgb, textureColor.w), intensity);
 }
    "#;

    let filter = Box::new(XHeyBasicFilter::new_shader_with_fragment(fragmentString,2));
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
pub extern "C" fn xhey_update_picture(picture: *const XheyPicture, data: *const c_void, width: i32, height: i32){
    let picture = unsafe{picture.as_ref().unwrap()};
    picture.update(data,width,height);
}



#[no_mangle]
pub extern "C" fn test(path: *const c_char){
    unsafe {
        let a =  CStr::from_ptr(path);
        let a = a.to_str().unwrap();
    }
}
