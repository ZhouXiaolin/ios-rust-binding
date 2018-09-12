use core::{Source,Consumer};
use std::mem::transmute;
use core::{Node,NodeType};
#[repr(C)]
pub struct XheyCamera{

}

impl Node for XheyCamera {
    fn get_type_id() -> NodeType {
        NodeType::Camera
    }
}

impl Source for XheyCamera {
    fn add_target<T:Consumer>(&self, target: &T, _location: u32){
        println!("XheyCamera add_target");
        target.set_source(self,_location);
    }

    fn remove_all_targets(){

    }
}

impl XheyCamera {
    fn new() -> Self {
        XheyCamera {
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