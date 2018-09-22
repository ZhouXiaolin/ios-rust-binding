use fnv::FnvHashMap;
use std::cell::{RefCell,Ref};

use std::marker::Sync;
use super::Framebuffer;
use super::*;

/// 这个缓存如何设计 内部可变 RefCell 字典 FnvHashMap 以 String 为key, 储存一个Framebuffer,内部可变，
pub struct FramebufferCache{
    unused:RefCell<FnvHashMap<String,Framebuffer>>,
    prepared:RefCell<Vec<Framebuffer>>
}

impl Default for FramebufferCache {
    fn default() -> Self {
        FramebufferCache{
            unused:RefCell::default(),
            prepared:RefCell::default()
        }
    }

}
unsafe impl Sync for FramebufferCache{}




impl FramebufferCache {


    pub fn pull(&self) -> Framebuffer {
        self.prepared.borrow_mut().pop().unwrap()
    }
    pub fn requestFramebufferWithDefault(&self, orientation: ImageOrientation, size: GLSize, textureOnly:bool) {
        let default = GPUTextureOptions::default();
        self.requestFramebufferWithProperties(orientation,size,textureOnly,default)
    }

    pub fn requestFramebufferWithProperties(&self,orientation:ImageOrientation, size:GLSize, textureOnly:bool, textureOptions: GPUTextureOptions) {


        let hash = hashStringForFramebuffer(size,textureOnly,textureOptions);

        let mut hashSet = self.unused.borrow_mut();
        let values: Option<Framebuffer> = hashSet.remove(&hash);

        match &values {
            Some(fb) if fb.valid() => {
                println!("has key, find fbo from cache");
                fb.orientation.set(orientation);
                self.prepared.borrow_mut().push(fb.clone())
            },
            _ => {
                println!("create a new framebuffer");
                // 为什么不在这里直接存入，因为在使用RefCell时，不允许借用和可变可用同时存在，
                let framebuffer = Framebuffer::new(orientation,size,textureOnly,textureOptions,None);
                self.prepared.borrow_mut().push(framebuffer)
            }

        }

    }

    pub fn purgeAllUnassignedFramebuffer(&self){
        self.unused.borrow_mut().clear();
        self.prepared.borrow_mut().clear();
    }


    pub fn returnToCache(&self, framebuffer:&Framebuffer){
        let mut framebufferCache = self.unused.borrow_mut();
        framebufferCache.insert(framebuffer.hashString(),framebuffer.clone());

    }

}
