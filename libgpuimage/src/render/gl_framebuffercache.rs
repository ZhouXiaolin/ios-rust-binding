use fnv::FnvHashMap;
use std::cell::{RefCell,Ref};

use std::marker::Sync;
use super::{Framebuffer,GPUTextureOptions,GLSize,ImageOrientation};
use super::hashStringForFramebuffer;
use std::rc::Rc;
pub struct FramebufferCache{
    cache:RefCell<FnvHashMap<String,Rc<Framebuffer>>>,
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


    pub fn requestFramebufferWithDefault(&self, orientation: ImageOrientation, size: GLSize, textureOnly:bool) -> Rc<Framebuffer> {
        let default = GPUTextureOptions::default();
        self.requestFramebufferWithProperties(orientation,size,textureOnly,default)
    }

    pub fn requestFramebufferWithProperties(&self,orientation:ImageOrientation, size:GLSize, textureOnly:bool, textureOptions: GPUTextureOptions) -> Rc<Framebuffer> {


        let hash = hashStringForFramebuffer(size,textureOnly,textureOptions);

        for i in self.cache.borrow_mut().iter() {
            if i.0 == &hash && i.1.valid() == true {
                println!("has key, find it in cache");
                i.1.orientation.set(orientation);
                return i.1.clone();
            }
        }

        println!("new framebuffer");
        let framebuffer = Rc::new(Framebuffer::new(orientation,size,textureOnly,textureOptions,None));
        self.cache.borrow_mut().insert(hash,framebuffer.clone());
        framebuffer


    }

    pub fn purgeAllUnassignedFramebuffer(&self){
        self.cache.borrow_mut().clear();
    }

}
