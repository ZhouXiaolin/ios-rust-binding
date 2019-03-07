
use ios_rust_binding::{UIView,UIImage};
use std::os::raw::{c_char,c_void,c_uint,c_float,c_long,c_int};
use std::ffi::{CStr};
use std::mem::transmute;
use std::rc::Rc;
use gles_rust_binding::*;
use crate::structure::{Graph,Edge,Computeable};
use crate::render::{Framebuffer,GlContext,Matrix3x3};
use crate::operation::*;

type RenderGraph<'a> = Graph<'a,Framebuffer>;
#[no_mangle]
pub extern "C" fn xhey_init_graph<'a>() -> c_long {
    let graph = Box::new(RenderGraph::new());
    Box::into_raw(graph) as c_long
}


#[no_mangle]
pub unsafe extern "C" fn release_graph(graph_ptr: c_long) {
    drop(Box::from_raw(graph_ptr as *mut RenderGraph));
}


#[no_mangle]
pub unsafe extern "C" fn init_context() -> c_long {

    let context = Box::new(GlContext::new());
    Box::into_raw(context) as c_long
}


#[no_mangle]
pub unsafe extern "C" fn release_context(context: c_long){

    let context = context as *mut GlContext;
    let context = context.as_ref().unwrap();
    context.framebufferCache.purgeAllUnassignedFramebuffer();

}

#[no_mangle]
pub unsafe extern "C" fn xhey_picture_graph<'a>(graph: c_long, picture: c_long, basic: c_long,lut: c_long, lut_filter: c_long, unsharpask: c_long, water_mask: c_long, output: c_long) {

    let box_graph = graph as *mut RenderGraph;
    let box_graph = box_graph.as_mut().unwrap();

    let box_picture = picture as *mut XheyPicture;
    let box_picture = box_picture.as_mut().unwrap();

    let box_basic = basic as *mut XHeyBasicFilter;
    let box_basic = box_basic.as_mut().unwrap();

    let box_lut = lut as *mut XheyPicture;
    let box_lut = box_lut.as_mut().unwrap();
//
    let box_lut_filter = lut_filter as *mut XHeyLookupFilter;
    let box_lut_filter = box_lut_filter.as_mut().unwrap();


    let box_unsharp_mark = unsharpask as *mut XHeyUnsharpMaskFilter;
    let box_unsharp_mark = box_unsharp_mark.as_mut().unwrap();
//
    let box_water_mask = water_mask as *mut XHeyBlendFilter;
    let box_water_mask = box_water_mask.as_mut().unwrap();

    let box_output = output as *mut XheyPictureOutput;
    let box_output = box_output.as_mut().unwrap();

    let pic = box_graph.add_input("picture", box_picture);
    let basic = box_graph.add_function("basic",&[pic],box_basic);
    let lut = box_graph.add_input("lut", box_lut);
    let lut_filter = box_graph.add_function("lut filter",&[basic, lut], box_lut_filter);
    let unsharp_mask = box_graph.add_function("unsharp mask", &[lut_filter],box_unsharp_mark);
    let water_mask = box_graph.add_function("water mask",&[unsharp_mask],box_water_mask);
    let output = box_graph.add_function("output",&[water_mask], box_output);
}


#[no_mangle]
pub unsafe extern "C" fn xhey_camera_watermark_graph<'a>(graph: c_long, picture: c_long, basic: c_long, watermark: c_long, output: c_long){
    let box_graph = graph as *mut RenderGraph;
    let box_graph = box_graph.as_mut().unwrap();

    let box_picture = picture as *mut XheyPicture;
    let box_picture = box_picture.as_mut().unwrap();

    let box_basic = basic as *mut XHeyBasicFilter;
    let box_basic = box_basic.as_mut().unwrap();

    let box_watermark = watermark as *mut XHeyBlendFilter;
    let box_watermark = box_watermark.as_mut().unwrap();

    let box_output = output as *mut XheyPictureOutput;
    let box_output = box_output.as_mut().unwrap();

    let pic = box_graph.add_input("picture",box_picture);
    let basic = box_graph.add_function("basic",&[pic],box_basic);
    let water_mark = box_graph.add_function("water mask",&[basic],box_watermark);
    let output = box_graph.add_function("output",&[water_mark],box_output);
}




#[no_mangle]
pub unsafe extern "C" fn xhey_print_graph(graph: c_long){
    let box_graph = graph as *mut RenderGraph;
    let box_graph = box_graph.as_mut().unwrap();
    box_graph.PrintGraphviz();
}

#[no_mangle]
pub unsafe extern "C" fn xhey_normal_camera_graph(graph: c_long, camera: c_long, basic: c_long, output: c_long){
    let box_graph = graph as *mut RenderGraph;
    let box_graph = box_graph.as_mut().unwrap();

    let box_camera = camera as *mut XheyCamera;
    let box_camera = box_camera.as_mut().unwrap();

    let box_basic = basic as *mut XHeyBasicFilter;
    let box_basic = box_basic.as_mut().unwrap();

    let box_output = output as *mut XheyPictureOutput;
    let box_output = box_output.as_mut().unwrap();

    let pic = box_graph.add_input("picture", box_camera);
    let basic = box_graph.add_function("basic",&[pic],box_basic);
    let output = box_graph.add_function("output",&[basic], box_output);
}

#[no_mangle]
pub unsafe extern "C" fn xhey_camera_graph<'a>(graph: c_long, camera: c_long, basic: c_long,normal_basic: c_long,lut: c_long, lut_filter: c_long, tone_curve:c_long, tone_curve_filter:c_long,unsharpask: c_long, water_mask: c_long, output: c_long, normal_output: c_long) {

    let box_graph = graph as *mut RenderGraph;
    let box_graph = box_graph.as_mut().unwrap();

    let box_camera = camera as *mut XheyCamera;
    let box_camera = box_camera.as_mut().unwrap();

    let box_basic = basic as *mut XHeyBasicFilter;
    let box_basic = box_basic.as_mut().unwrap();


//    let box_lut = lut as *mut XheyPicture;
//    let box_lut = box_lut.as_mut().unwrap();
//
//    let box_lut_filter = lut_filter as *mut XHeyLookupFilter;
//    let box_lut_filter = box_lut_filter.as_mut().unwrap();

    let box_tone_curve = tone_curve as *mut XheyPicture;
    let box_tone_curve = box_tone_curve.as_mut().unwrap();

    let box_tone_curve_filter = tone_curve_filter as *mut XheyToneCurveFilter;
    let box_tone_curve_filter = box_tone_curve_filter.as_mut().unwrap();


    let box_unsharp_mark = unsharpask as *mut XHeyUnsharpMaskFilter;
    let box_unsharp_mark = box_unsharp_mark.as_mut().unwrap();

    let box_output = output as *mut XheyPictureOutput;
    let box_output = box_output.as_mut().unwrap();

    let box_normal_output = normal_output as *mut XheyPictureOutput;
    let box_normal_output = box_normal_output.as_mut().unwrap();


    let cam = box_graph.add_input("camera", box_camera);
    let basic = box_graph.add_function("basic",&[cam],box_basic);

    let normal_output = box_graph.add_function("normal output",&[cam], box_normal_output);

//    let lut = box_graph.add_input("lut", box_lut);
//    let lut_filter = box_graph.add_function("lut filter",&[basic, lut], box_lut_filter);

    let tone_curve = box_graph.add_input("tone curve",box_tone_curve);
    let tone_curve_filter = box_graph.add_function("tone curve filter",&[basic,tone_curve], box_tone_curve_filter);

    let unsharp_mark = box_graph.add_function("unsharp mask",&[tone_curve_filter],box_unsharp_mark);
    let output = box_graph.add_function("output",&[unsharp_mark], box_output);
}

#[repr(C)]
pub enum InputKind{
    Picture,
    Camera
}

#[repr(C)]
pub enum OutputKind{
    AlphaBlend,
    Basic,
    Blend,
    Combine,
    Lookup,
    UnsharpMask,
    PictureOutput
}

#[no_mangle]
pub unsafe extern "C" fn xhey_graph_add_function(graph: c_long, filter: c_long, arg:&[u32],kind: OutputKind) -> c_long{
    let box_graph = graph as *mut RenderGraph;
    let box_graph = box_graph.as_mut().unwrap();

    let result = match kind {
        OutputKind::AlphaBlend => {
            let alpha_blend = filter as *mut XHeyAlphaBlendFilter;
            let alpha_blend = alpha_blend.as_mut().unwrap();

            box_graph.add_function("alpha blend",arg,alpha_blend)
        },

        OutputKind::Basic => {
            let basic = filter as *mut XHeyBasicFilter;
            let basic = basic.as_mut().unwrap();

            box_graph.add_function("basic",arg,basic)
        },

        OutputKind::Blend => {
            let blend = filter as *mut XHeyBlendFilter;
            let blend = blend.as_mut().unwrap();

            box_graph.add_function("blend", arg, blend)
        },

        OutputKind::Combine => {
            let combine = filter as *mut XHeyCombineFilter;
            let combine = combine.as_mut().unwrap();

            box_graph.add_function("combine", arg, combine)
        },

        OutputKind::Lookup => {
            let lookup = filter as *mut XHeyLookupFilter;
            let lookup = lookup.as_mut().unwrap();

            box_graph.add_function("lookup", arg, lookup)
        },

        OutputKind::UnsharpMask => {
            let unsharp_mask = filter as *mut XHeyUnsharpMaskFilter;
            let unsharp_mask = unsharp_mask.as_mut().unwrap();

            box_graph.add_function("unsharp mask",arg,unsharp_mask)
        },

        OutputKind::PictureOutput => {
            let picture_output = filter as *mut XheyPictureOutput;
            let picture_output = picture_output.as_mut().unwrap();

            box_graph.add_function("picture output", arg, picture_output)
        }
    };

    result as c_long

}
#[no_mangle]
pub unsafe extern "C" fn xhey_graph_add_input(graph: c_long,filter:c_long,kind:InputKind) -> c_long {
    let box_graph = graph as *mut RenderGraph;
    let box_graph = box_graph.as_mut().unwrap();


    let result = match kind {
        InputKind::Picture => {
            let picture = filter as *mut XheyPicture;
            let picture = picture.as_mut().unwrap();
            box_graph.add_input("picture",picture)
        },
        InputKind::Camera => {
            let camera = filter as *mut XheyCamera;
            let camera = camera.as_mut().unwrap();
            box_graph.add_input("camera",camera)

        }
    };

    result as c_long

}





#[no_mangle]
pub unsafe extern "C" fn xhey_graph_forward<'a>(graph: c_long){
    let box_graph = graph as *mut RenderGraph;
    let box_graph = box_graph.as_mut().unwrap();
    box_graph.forward();
}



#[no_mangle]
pub unsafe extern "C" fn xhey_init_alpha_blend(context:c_long) -> c_long {
    let context = context as *mut GlContext;
    let context = context.as_ref().unwrap();
    let filter = Box::new(XHeyAlphaBlendFilter::new(context));
    Box::into_raw(filter) as c_long
}

#[no_mangle]
pub unsafe extern "C" fn xhey_init_tone_curve(context:c_long) -> c_long {
    let context = context as *mut GlContext;
    let context = context.as_ref().unwrap();
    let filter = Box::new(XheyToneCurveFilter::new(context));
    Box::into_raw(filter) as c_long
}

#[no_mangle]
pub unsafe extern "C" fn release_alpha_blend_filter(filter_ptr: c_long) {
    drop(Box::from_raw(filter_ptr as *mut XHeyAlphaBlendFilter))
}


#[no_mangle]
pub unsafe extern "C" fn xhey_init_camera(context: c_long, width: i32, height: i32, orient: i32) -> c_long {
    let context = context as *mut GlContext;
    let context = context.as_ref().unwrap();
    let filter = Box::new(XheyCamera::new(context,width,height,orient));
    Box::into_raw(filter) as c_long
}

#[no_mangle]
pub unsafe extern "C" fn camera_update_rotation(filter: c_long, rotation: i32){
    let filter = filter as *mut XheyCamera;
    let filter = filter.as_mut().unwrap();
    filter.updateRotation(rotation);
}

#[no_mangle]
pub unsafe extern "C" fn release_camera(filter: c_long){

    drop(Box::from_raw(filter as *mut XheyCamera));

}

#[no_mangle]
pub unsafe extern "C" fn camera_update_luminance(camera:c_long, luminance: i32) {
    let filter = camera as *mut XheyCamera;
    let filter = filter.as_mut().unwrap();
    filter.update_luminance(luminance as GLuint);
}

#[no_mangle]
pub unsafe extern "C" fn camera_update_chrominance(camera:c_long, chrominance: i32) {
    let filter = camera as *mut XheyCamera;
    let filter = filter.as_mut().unwrap();
    filter.update_chrominance(chrominance as GLuint);
}

#[no_mangle]
pub unsafe extern "C" fn camera_update_matrix(camera: c_long, matrix: *mut [f32;9]){
    let filter = camera as *mut XheyCamera;
    let filter = filter.as_mut().unwrap();

    let mat = Matrix3x3::new(matrix.as_ref().unwrap());
    filter.updateMatrix(mat)
}

#[no_mangle]
pub unsafe extern "C" fn camera_update_size(camera: c_long, width: i32, height: i32){
    let filter = camera as *mut XheyCamera;
    let filter = filter.as_mut().unwrap();
    filter.updateSize(width,height);
}


#[no_mangle]
pub unsafe extern "C" fn xhey_init_watermark(context:c_long) -> c_long {
    let context = context as *mut GlContext;
    let context = context.as_ref().unwrap();
    let filter = Box::new(XHeyBlendFilter::new(context));
    Box::into_raw(filter) as c_long
}


#[no_mangle]
pub unsafe extern "C" fn release_water_mark_filter(filter_ptr: c_long){
    drop(Box::from_raw(filter_ptr as *mut XHeyBlendFilter))
}


#[no_mangle]
pub unsafe extern "C" fn xhey_watermark_update(filter: c_long, texId: c_uint, x: c_float, y: c_float, w: c_float, h: c_float, rotation: c_int){
    let filter = filter as *mut XHeyBlendFilter;
    let filter = filter.as_mut().unwrap();
    filter.appendWaterMark(texId,x,y,w,h,rotation);
}

#[no_mangle]
pub unsafe extern "C" fn xhey_picture_update(filter: c_long, data: *const c_void, width: i32, height: i32){
    let picture = filter as *mut XheyPicture;
    let picture = picture.as_mut().unwrap();

    picture.update(data,width,height);
}

#[no_mangle]
pub unsafe extern "C" fn xhey_init_unsharp_mask(context:c_long) -> c_long {
    let context = context as *mut GlContext;
    let context = context.as_ref().unwrap();
    let filter = Box::new(XHeyUnsharpMaskFilter::new(context));
    Box::into_raw(filter) as c_long
}


#[no_mangle]
pub unsafe extern "C" fn release_unsharp_mask_filter(filter_ptr: c_long){
    drop(Box::from_raw(filter_ptr as *mut XHeyUnsharpMaskFilter))
}


#[no_mangle]
pub unsafe extern "C" fn xhey_init_basic_filter(context:c_long) -> c_long {
    let context = context as *mut GlContext;
    let context = context.as_ref().unwrap();
    let filter = Box::new(XHeyBasicFilter::new(context));
    Box::into_raw(filter) as c_long
}

#[no_mangle]
pub unsafe extern "C" fn xhey_update_basic_hook(basic_filter_ptr: c_long, hook: extern "C" fn(context: *mut c_void),ctxt: *mut c_void){
    let filter = basic_filter_ptr as *mut XHeyBasicFilter;
    let filter = filter.as_mut().unwrap();
    filter.updateHookFunction(hook,ctxt);

}

#[no_mangle]
pub unsafe extern "C" fn xhey_update_basic_rotation(basic: c_long, rotation: c_int){
    let filter = basic as *mut XHeyBasicFilter;
    let filter = filter.as_mut().unwrap();
    filter.updateOutputRotation(rotation);
}

#[no_mangle]
pub unsafe extern "C" fn xhey_update_basic_size(basic: c_long, width: i32, height: i32){
    let filter = basic as *mut XHeyBasicFilter;
    let filter = filter.as_mut().unwrap();
    filter.updateOutputSize(width,height);
}

#[no_mangle]
pub unsafe extern "C" fn xhey_get_basic_texture_id(basic: c_long) -> c_int {
    let filter = basic as *mut XHeyBasicFilter;
    let filter = filter.as_mut().unwrap();
    filter.textureId() as c_int
}

#[no_mangle]
pub unsafe extern "C" fn release_basic_filter(basic_filter_ptr: c_long) {
    drop(Box::from_raw(basic_filter_ptr as *mut XHeyBasicFilter))
}


#[no_mangle]
pub unsafe extern "C" fn xhey_init_surface_view(context:c_long, width: i32, height: i32) -> c_long {

    let context = context as *mut GlContext;
    let context = context.as_ref().unwrap();
    let surfaceView = Box::new(XheySurfaceView::new(context,width,height));
    Box::into_raw(surfaceView) as c_long
}


#[no_mangle]
pub unsafe extern "C" fn release_surfaceview(surface_view_ptr: c_long) {
    drop(Box::from_raw(surface_view_ptr as *mut XheySurfaceView))
}




#[no_mangle]
pub unsafe extern "C" fn xhey_init_lookup_filter(context:c_long) -> c_long {


    let context = context as *mut GlContext;
    let context = context.as_ref().unwrap();

    let filter = Box::new(XHeyLookupFilter::new(context));
    Box::into_raw(filter) as c_long
}
#[no_mangle]
pub unsafe extern "C" fn xhey_update_lookup_intensity(lut_filter: c_long, v: c_float){
    let filter = lut_filter as *mut XHeyLookupFilter;
    let filter = filter.as_mut().unwrap();
    filter.set_intensity(v);
}

#[no_mangle]
pub unsafe extern "C" fn release_lookup_filter(lookup_filter_ptr: c_long) {
    drop(Box::from_raw(lookup_filter_ptr as *mut XHeyLookupFilter))
}





#[no_mangle]
pub extern "C" fn xhey_init_picture(data: *const c_void, width: i32, height: i32) ->  c_long {
    let picture = XheyPicture::new(data,width,height);
    let picture = Box::new(picture);
    Box::into_raw(picture) as c_long

}

#[no_mangle]
pub extern "C" fn xhey_init_picture_textureId(textureId: i32, width: i32, height: i32, orient: i32) ->  c_long {

    let picture = XheyPicture::new_texture(textureId as GLuint,width,height,orient);
    let picture = Box::new(picture);
    Box::into_raw(picture) as c_long

}

#[no_mangle]
pub unsafe extern "C" fn release_picture(picture_ptr: c_long) {
    drop(Box::from_raw(picture_ptr as *mut XheyPicture))
}




#[no_mangle]
pub unsafe extern "C" fn xhey_init_picture_output(context:c_long,width: i32, height: i32, orient: i32) -> c_long {
    let context = context as *mut GlContext;
    let context = context.as_ref().unwrap();

    let output = Box::new(XheyPictureOutput::new(context,width, height, orient));
    Box::into_raw(output) as c_long
}

#[no_mangle]
pub unsafe extern "C" fn xhey_update_picture_output_hook(pic_output_filter: c_long, hook: extern "C" fn(context: *mut c_void), ctxt: *mut c_void){
    let filter = pic_output_filter as *mut XheyPictureOutput;
    let filter = filter.as_mut().unwrap();
    filter.updateHookFunction(hook,ctxt);
}

#[no_mangle]
pub unsafe extern "C" fn release_output(filter_ptr: c_long) {
    drop(Box::from_raw(filter_ptr as *mut XheyPictureOutput))
}

#[no_mangle]
pub unsafe extern "C" fn xhey_update_output_rotation(output: c_long, rotation: i32){
    let filter = output as *mut XheyPictureOutput;
    let filter = filter.as_mut().unwrap();
    filter.updateRotation(rotation);
}

#[no_mangle]
pub unsafe extern "C" fn xhey_update_output_size(output: c_long, width: i32, height: i32){
    let filter = output as *mut XheyPictureOutput;
    let filter = filter.as_mut().unwrap();
    filter.updateBackingSize(width,height);
}

#[no_mangle]
pub unsafe extern "C" fn xhey_picture_output_get_texture_id(filter_ptr: c_long) -> c_int {
    let filter = filter_ptr as *mut XheyPictureOutput;
    let filter = filter.as_ref().unwrap();
    filter.textureId() as c_int
}

