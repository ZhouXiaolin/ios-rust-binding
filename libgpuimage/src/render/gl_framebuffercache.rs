use fnv::FnvHashMap;
use std::cell::{RefCell,Ref};

use std::marker::Sync;
use super::{Framebuffer,GPUTextureOptions,GLSize,ImageOrientation};
use super::hashStringForFramebuffer;
/// 这个缓存如何设计 内部可变 RefCell 字典 FnvHashMap 以 String 为key, 储存一个Framebuffer,内部可变，
pub struct FramebufferCache{
    cache:RefCell<FnvHashMap<String,Framebuffer>>,
}

impl Default for FramebufferCache {
    fn default() -> Self {
        FramebufferCache{
            cache:RefCell::default(),
        }
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

        let hashT = self.cache.borrow_mut();

        if hashT.contains_key(&hash) {
            let v = hashT.get(&hash).unwrap();
            v.orientation.set(orientation);
            v.clone()
        }else{
            let framebuffer = Framebuffer::new(orientation,size,textureOnly,textureOptions,None);
            framebuffer
        }

    }

    pub fn purgeAllUnassignedFramebuffer(&self){
        self.cache.borrow_mut().clear();
    }


    pub fn returnToCache(&self, framebuffer:&Framebuffer){
        let mut framebufferCache = self.cache.borrow_mut();
        framebufferCache.insert(framebuffer.hashString(),framebuffer.clone());

    }

}
