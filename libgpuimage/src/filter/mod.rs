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
pub extern "C" fn xhey_add_target(source: *mut c_void, consumer: *mut c_void){

    let box_source = unsafe {
        mem::transmute::<*mut c_void,Box<XheyPicture>>(source)
    };

    let box_consumer = unsafe {
        mem::transmute::<*mut c_void,Box<XHeyBasicFilter>>(consumer)
    };

//    box_source.add_target(&*box_consumer,0); //这里需要开启NLL特性
}