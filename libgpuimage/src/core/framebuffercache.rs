#![allow(dead_code)]
use core::framebuffer::*;
use core::context::GlContext;
use core::sharedImageProcessingContext;
use std::collections::BTreeMap;
use std::cell::{RefCell,Cell};
use core::context::SerialDispatch;
use gles_rust_binding::*;
use std::rc::Rc;

use std::marker::Sync;


// 这个缓存如何设计 内部可变 RefCell 字典 BTreeMap 以 i64 为key, 储存一个Framebuffer的数组,内部可变，每个Framebuffer是Rc 这里应该设计为struct()???

pub struct FramebufferCache(pub RefCell<BTreeMap<i64,RefCell<Vec<Rc<Framebuffer>>>>>);

impl Default for FramebufferCache {
    fn default() -> Self {
        FramebufferCache(RefCell::default())
    }

}
unsafe impl Sync for FramebufferCache{}




impl FramebufferCache {

    pub fn requestFramebufferWithDefault(&self, orientation: ImageOrientation, size: GLSize, textureOnly:bool) -> Rc<Framebuffer> {
        let default = GPUTextureOptions::default();
        self.requestFramebufferWithProperties(orientation,size,textureOnly,default)
    }

    pub fn requestFramebufferWithProperties(&self,orientation:ImageOrientation, size:GLSize, textureOnly:bool, textureOptions: GPUTextureOptions) -> Rc<Framebuffer> {

        let hash = hashForFramebufferWithProperties(size,textureOnly,textureOptions,false);

        let mut framebufferCache = self.0.borrow_mut();

        let result = framebufferCache.get_mut(&hash);

        match result {
            Some(vec) => {


                let mut vec = vec.borrow_mut();
                let len = vec.len();

                let framebuffer = vec.remove(len - 1);
                framebuffer.orientation.set(orientation);
                framebuffer

            },
            None => {
                let framebuffer = Rc::new(Framebuffer::new(orientation,size,textureOnly,textureOptions,None));
                framebuffer
            }
        }

    }

    fn purgeAllUnassignedFramebuffer(&self){
        self.0.borrow_mut().clear();
    }

    pub fn returnToCache(&self, framebuffer : Framebuffer){

        sharedImageProcessingContext.makeCurrentContext();

        match self.0.borrow_mut().get(&framebuffer.hash) {
            Some(vec) => {
                let mut v = vec.borrow_mut();
                v.push(Rc::new(framebuffer));

            },
            None => {
                let hash = framebuffer.hash;
                let value = RefCell::new(vec![Rc::new(framebuffer)]);
                self.0.borrow_mut().insert(hash,value);
            }
        }
    }
}