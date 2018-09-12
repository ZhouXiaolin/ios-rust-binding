
use core::{Source,Consumer};
use std::mem::transmute;
use core::{Node,NodeType};
#[repr(C)]
pub struct XheyPicture{
}

impl XheyPicture {
    fn new() -> Self {
        XheyPicture{
        }
    }

    fn log(&self){
        println!("Xhey Picture log");
    }
}

impl Node for XheyPicture {
    fn get_type_id() -> NodeType {
        NodeType::Picture
    }
}

impl Source for XheyPicture {
    fn add_target<T:Consumer>(&self, target: &T, _location: u32){
        println!("XheyCamera add_target");
        target.set_source(self,_location);
    }

    fn remove_all_targets(){

    }
}

#[allow(non_snake_case, unused_variables, dead_code)]
#[no_mangle]
pub extern "C" fn xhey_init_picture() -> *mut XheyPicture {
    println!("xhey_init_picture");
    unsafe {transmute(Box::new(XheyPicture::new()))}
}

#[allow(non_snake_case, unused_variables, dead_code)]
#[no_mangle]
pub extern "C" fn xhey_process_picture(picture: *mut XheyPicture){
    let picture :Box<XheyPicture>  = unsafe{transmute(picture)};
    picture.log();
}


use std::os::raw::{c_char,c_void};
use std::ffi::{CStr};
use ios_rust_binding::UIImage;
#[allow(non_snake_case, unused_variables, dead_code)]
#[no_mangle]
pub extern "C" fn test(path: *const c_char) -> *mut c_void{
    unsafe {
        let a =  CStr::from_ptr(path);
        let a = a.to_str().unwrap();
        let image = UIImage::get_image(a);
        transmute(image)
    }
}