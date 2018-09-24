use fnv::FnvHashMap;
use std::cell::{RefCell,Ref};

use std::marker::Sync;
use super::{Framebuffer,GPUTextureOptions,GLSize,ImageOrientation};
use super::hashStringForFramebuffer;
/// 这个缓存如何设计 内部可变 RefCell 字典 FnvHashMap 以 String 为key, 储存一个Framebuffer,内部可变，
pub struct FramebufferCache{
    unused:RefCell<FnvHashMap<String,Framebuffer>>,
}

impl Default for FramebufferCache {
    fn default() -> Self {
        FramebufferCache{
            unused:RefCell::default(),
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

        let hashSet = self.unused.borrow();
        let values: Option<&Framebuffer> = hashSet.get(&hash);

        match values {
            Some(fb) if fb.valid() => {
                println!("has key, find fbo from cache");
                fb.orientation.set(orientation);
                fb.clone()
            },
            _ => {
                println!("create a new framebuffer");
                let framebuffer = Framebuffer::new(orientation,size,textureOnly,textureOptions,None);
                framebuffer
            }

        }

    }

    pub fn purgeAllUnassignedFramebuffer(&self){
        self.unused.borrow_mut().clear();
    }


    pub fn returnToCache(&self, framebuffer:&Framebuffer){
        let mut framebufferCache = self.unused.borrow_mut();
        framebufferCache.insert(framebuffer.hashString(),framebuffer.clone());

    }

}