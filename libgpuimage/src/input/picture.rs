
use super::{Source,Consumer,sharedImageProcessingContext,Framebuffer,ImageOrientation,GLSize,NodeType,RenderNode};

use std::mem::transmute;
use gles_rust_binding::*;
use std::os::raw::c_void;
use std::cell::{RefCell};
#[repr(C)]
pub struct XheyPicture<'a>{
    _type: RenderNode,
    _targets: RefCell<Vec<Box<&'a dyn Consumer>>>,
    _framebuffer: Framebuffer
}



impl<'a> XheyPicture<'a> {
    pub fn new(data: *const c_void, width: i32, height: i32) -> Self {



        sharedImageProcessingContext.makeCurrentContext();
        let size = GLSize::new(width,height);
        let framebuffer = sharedImageProcessingContext.frameubfferCache.requestFramebufferWithDefault(ImageOrientation::portrait,size,true);

        unsafe {
            glBindTexture(GL_TEXTURE_2D,framebuffer.texture);
            glTexImage2D(GL_TEXTURE_2D,0,GL_RGBA as i32,width,height,0,GL_BGRA,GL_UNSIGNED_BYTE,data as *const _);
            glBindTexture(GL_TEXTURE_2D,0);
        }

        XheyPicture{
            _type: RenderNode::new(NodeType::Picture),
            _targets: RefCell::default(),
            _framebuffer: framebuffer
        }
    }

    pub fn processImage(&self) {
        self.updateTargetsWithFramebuffer(&self._framebuffer);
    }


}




impl<'a,'b:'a> Source<'b> for XheyPicture<'a> {
    fn addTarget(&self, target: &'b dyn Consumer, _location: u32){
        println!("XheyPicture add_target");
        let mut targets = self._targets.borrow_mut();
        targets.push(Box::new(target));
        target.setSource(self,_location);
    }

    fn removeAllTargets(&self){
        println!("XheyPicture remove")
    }

    fn updateTargetsWithFramebuffer(&self, framebuffer:&Framebuffer){
        for (index,target) in self._targets.borrow_mut().iter().enumerate() {
            target.newFramebufferAvailable(framebuffer,index);
        }
    }
}



