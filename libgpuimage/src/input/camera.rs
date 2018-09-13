use core::{Source,Consumer};
use std::mem::transmute;
use core::{Node,NodeType,RenderNode};
#[repr(C)]
pub struct XheyCamera{
    _type:RenderNode
}


impl<'a> Source<'a> for XheyCamera {
    fn add_target(&self, target: &'a dyn Consumer, _location: u32){
        println!("XheyCamera add_target");
        target.set_source(self,_location);
    }

    fn remove_all_targets(&self){

    }
}

impl XheyCamera {
    fn new() -> Self {
        XheyCamera {
            _type:RenderNode::new(NodeType::Camera)
        }
    }
}


#[allow(non_snake_case, unused_variables, dead_code)]
#[no_mangle]
pub extern "C" fn xhey_init_camera() -> *mut XheyCamera {
    println!("xhey_init_camera");
    unsafe {transmute(Box::new(XheyCamera::new()))}
}

#[allow(non_snake_case, unused_variables, dead_code)]
#[no_mangle]
pub extern "C" fn xhey_start_capture(camera: *mut XheyCamera){
    println!("xhey_start_camera");
}

#[allow(non_snake_case, unused_variables, dead_code)]
#[no_mangle]
pub extern "C" fn xhey_stop_capture(camera: *mut XheyCamera){
    println!("xhey_start_camera");
}