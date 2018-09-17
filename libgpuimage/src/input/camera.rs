use super::{Source,Consumer,Framebuffer,NodeType,RenderNode};
use std::cell::RefCell;
#[repr(C)]
pub struct XheyCamera<'a>{
    _type:RenderNode,
    _targets: RefCell<Vec<Box<&'a dyn Consumer>>>,
}


impl<'a,'b:'a> Source<'b> for XheyCamera<'a> {
    fn add_target(&self, target: &'b dyn Consumer, _location: u32){
        println!("XheyCamera add_target");
        target.set_source(self,_location);
    }

    fn remove_all_targets(&self){

    }
    fn updateTargetsWithFramebuffer(&self, framebuffer:&Framebuffer){
        for (index,target) in self._targets.borrow_mut().iter().enumerate() {
            target.newFramebufferAvailable(framebuffer,index);
        }
    }
}

impl<'a> XheyCamera<'a> {
    fn new() -> Self {
        XheyCamera {
            _type:RenderNode::new(NodeType::Camera),
            _targets:RefCell::default()
        }
    }
}


#[allow(non_snake_case, unused_variables, dead_code)]
#[no_mangle]
pub extern "C" fn xhey_init_camera<'a>() -> *mut XheyCamera<'a> {
    println!("xhey_init_camera");
    let camera = Box::new(XheyCamera::new());
    Box::into_raw(camera)
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