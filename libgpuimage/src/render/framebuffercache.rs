#![allow(dead_code)]

use fnv::FnvHashMap;
use std::cell::{RefCell,Cell};
use gles_rust_binding::*;
use std::rc::Rc;

use std::marker::Sync;

use super::*;

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

        let hashSet = self.0.borrow();
        if hashSet.contains_key(&hash) {
            println!("contains this hashString,find framebuffer from cache");
            // 存在这个hashString 则进一步在framebufferCache中寻找
            let frameBufferCache = self.0.borrow();
            let fb = frameBufferCache.get(&hash).expect("Error, Cannot Find Framebuffer");
            fb.orientation.set(orientation);
            fb.clone()


        }else{
            // 为什么不在这里直接存入，因为在使用RefCell时，不允许借用和可变可用同时存在，
            println!("create a new framebuffer, hashString {}",hash);
            let framebuffer = Framebuffer::new(orientation,size,textureOnly,textureOptions,None);
            framebuffer
        }



    }

    pub fn purgeAllUnassignedFramebuffer(&self){
        self.0.borrow_mut().clear();
    }


    pub fn returnToCache(&self, framebuffer:&Framebuffer){
        let mut framebufferCache = self.0.borrow_mut();
        let hashString = framebuffer.hashString();
        framebufferCache.insert(hashString,framebuffer.clone());

    }

}