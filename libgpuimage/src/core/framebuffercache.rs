#![allow(dead_code)]
use core::framebuffer::*;
use core::context::GlContext;
use core::sharedImageProcessingContext;
use fnv::FnvHashMap;
use std::cell::{RefCell,Cell};
use core::context::SerialDispatch;
use gles_rust_binding::*;
use std::rc::Rc;

use std::marker::Sync;


// 这个缓存如何设计 内部可变 RefCell 字典 FnvHashMap 以 String 为key, 储存一个Framebuffer,内部可变，

pub struct FramebufferCache(RefCell<FnvHashMap<String,Framebuffer>>);

impl Default for FramebufferCache {
    fn default() -> Self {
        FramebufferCache(RefCell::default())
    }

}
unsafe impl Sync for FramebufferCache{}




impl FramebufferCache {

    pub fn requestFramebufferWithDefault(&self, orientation: ImageOrientation, size: GLSize, textureOnly:bool) -> Framebuffer {
        let default = GPUTextureOptions::default();
        self.requestFramebufferWithProperties(orientation,size,textureOnly,default)
    }

    pub fn requestFramebufferWithProperties(&self,orientation:ImageOrientation, size:GLSize, textureOnly:bool, textureOptions: GPUTextureOptions) -> Framebuffer {



        let hash = hashStringForFramebuffer(size,textureOnly,textureOptions);

        let framebufferCache = self.0.borrow();


        let result = framebufferCache.get(&hash);

        match result {
            Some(framebuffer) => {

                println!("framebufferCache count {}",framebufferCache.len());

                let fb = framebuffer;

                fb.orientation.set(orientation);
                fb.clone()



            },
            None => {
                let framebuffer = Framebuffer::new(orientation,size,textureOnly,textureOptions,None);
                framebuffer
            }
        }

    }

    pub fn purgeAllUnassignedFramebuffer(&self){
        self.0.borrow_mut().clear();
    }


    pub fn returnToCache(&self, framebuffer: &Framebuffer){
        sharedImageProcessingContext.makeCurrentContext();
        let mut framebufferCache = self.0.borrow_mut();
        let hashString = framebuffer.hashString();

        framebufferCache.insert(hashString,framebuffer.clone());

        println!("framebufferCache count {}",framebufferCache.len());
    }

}