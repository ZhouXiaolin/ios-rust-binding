use super::{Source,Consumer,Framebuffer};
use std::cell::RefCell;
#[repr(C)]
pub struct XheyCamera<'a>{
    _targets: RefCell<Vec<Box<&'a dyn Consumer>>>,
}


impl<'a,'b:'a> Source<'b> for XheyCamera<'a> {
    fn addTarget(&self, target: &'b dyn Consumer, _location: u32){
        println!("XheyCamera add_target");
        target.setSource(self,_location);
    }

    fn removeAllTargets(&self){

    }
    fn updateTargetsWithFramebuffer(&self, framebuffer:&Framebuffer){
        for (index,target) in self._targets.borrow_mut().iter().enumerate() {
            target.newFramebufferAvailable(framebuffer,index);
        }
    }
}

impl<'a> XheyCamera<'a> {
    pub fn new() -> Self {
        XheyCamera {
            _targets:RefCell::default()
        }
    }
}


