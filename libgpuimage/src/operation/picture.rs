
use super::*;

use std::mem::transmute;
use gles_rust_binding::*;
use std::os::raw::c_void;
use std::cell::{RefCell,Cell};
#[repr(C)]
pub struct XheyPicture<'a>{
    _targets: RefCell<Vec<Box<&'a dyn Consumer>>>,
    _framebuffer: Cell<Framebuffer>,
    index:u32,
    inputs: RefCell<Vec<u32>>
}

impl<'a> Drop for XheyPicture<'a> {
    fn drop(&mut self){
        println!("Drop XheyPicture");
    }

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
            _targets: RefCell::default(),
            _framebuffer: Cell::new(framebuffer),
            index:sharedContext.operation_id(),
            inputs:RefCell::default()
        }
    }

    pub fn processImage(&self) {
        self.updateTargetsWithFramebuffer(&self._framebuffer.take());
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

#[cfg(feature = "new")]
impl<'a> Operation for XheyPicture<'a>{
    /// 将ni加入这个节点的输入序列
    fn append(&self, ni: u32){
        self.inputs.borrow_mut().push(ni);
    }

    /// 返回输入序列
    fn inputs(&self) -> Vec<u32>{
        let inputs = self.inputs.borrow();
        let mut outputs = Vec::new();
        for input in inputs.iter() {
            outputs.push(input.clone());
        }
        outputs
    }

    /// 节点在图中的序号
    fn index(&self) -> u32{
        self.index
    }

    /// 指定输入最大个数
    fn arity(&self) -> u32{
        1
    }

    /// 前向计算
    fn forward(&self, xs: Vec<Framebuffer>) -> Framebuffer{
        println!("XHeyPicture 前向计算");
        self._framebuffer.take().clone()
    }

    ///针对Source节点，在渲染过程中指定其Framebufer
    fn set_framebuffer(&self, value:Framebuffer){
        self._framebuffer.set(value)
    }
}

