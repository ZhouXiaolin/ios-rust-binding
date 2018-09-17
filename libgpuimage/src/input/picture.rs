
use super::{Source,Consumer,sharedImageProcessingContext,Framebuffer,ImageOrientation,GLSize,NodeType,RenderNode};

use std::mem::transmute;
use gles_rust_binding::*;
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
    fn add_target(&self, target: &'b dyn Consumer, _location: u32){
        println!("XheyPicture add_target");
        let mut targets = self._targets.borrow_mut();
        targets.push(Box::new(target));
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

#[allow(non_snake_case, unused_variables, dead_code)]
#[no_mangle]
pub extern "C" fn xhey_init_picture<'a>(data: *const c_void, width: i32, height: i32) ->  *mut XheyPicture<'a> {
    println!("xhey_init_picture");
    let picture = Box::new(XheyPicture::new(data,width,height));
    Box::into_raw(picture)

}

#[allow(non_snake_case, unused_variables, dead_code)]
#[no_mangle]
pub extern "C" fn xhey_process_picture(picture: *const XheyPicture){
    let p = unsafe{picture.as_ref().unwrap()};
    p.processImage();


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

