use core::{Node, NodeType,RenderNode,Source,Consumer};
use std::os::raw::{c_void};
use std::mem;
use input::camera::XheyCamera;
use filter::basic::XHeyBasicFilter;
use filter::tonecurve::XHeyToneCurveFilter;
use filter::gaussianblur::XHeyGaussianBlurFilter;
use filter::lookuptable::XHeyLookupTableFilter;
use input::picture::XheyPicture;
use output::view::XHeyView;
pub mod basic;
pub mod gaussianblur;
pub mod lookuptable;
pub mod tonecurve;






#[allow(non_snake_case, unused_variables, dead_code)]
#[no_mangle]
pub extern "C" fn xhey_add_target<'a>(source: *mut XheyPicture<'a>, filter: *mut XHeyBasicFilter<'a>, consumer: *mut XHeyView){
    let box_picture = unsafe{source.as_ref().unwrap()};
    let box_filter = unsafe{filter.as_ref().unwrap()};
    let box_view = unsafe{consumer.as_ref().unwrap()};
    box_picture.add_target(box_filter,0);
    box_filter.add_target(box_view,0);
}