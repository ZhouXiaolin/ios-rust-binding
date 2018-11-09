use fnv::FnvHashMap;
use std::cell::{RefCell,Ref};

use std::marker::Sync;
use super::{Framebuffer,GPUTextureOptions,GLSize,ImageOrientation};
use super::hashStringForFramebuffer;
use std::rc::Rc;
use gles_rust_binding::*;


// 缓存的策略 首先，这是一个哈希表
// 因为要把它放到context 可变性 RefCell
// 在渲染中，可能产生多个相同属性的fbo,需要lock和unlock
// 多个相同属性的fbo 需要Vec
// fbo需要Rc

#[derive(Debug, Default)]
pub struct FramebufferCacheValue{
    value:RefCell<Vec<Rc<Framebuffer>>>
}


impl FramebufferCacheValue {
    fn new(f : Rc<Framebuffer>) -> Self {
        FramebufferCacheValue {
            value: RefCell::new(vec![f])
        }
    }
    fn pop(&self) -> Rc<Framebuffer> {
        let mut values = self.value.borrow_mut();
        if values.len() > 0 {
            values.pop().unwrap()
        }else{
            info!("why ?");
            panic!("why ?")
        }

    }

}


#[derive(Debug,Default)]
pub struct FramebufferCache{
    cache:RefCell<FnvHashMap<String,FramebufferCacheValue>>,
}


impl Drop for FramebufferCache {
    fn drop(&mut self){
        info!("Drop FramebufferCache");
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
        let mut cache = self.cache.borrow_mut();

        for i in cache.iter() {
            if i.0 == &hash {
                let mut value_vec = i.1.value.borrow_mut();
                for f in value_vec.iter() {
                    if f.valid() {

                        return f.clone();
                    }
                }

                let f = Rc::new(Framebuffer::new(orientation,size,textureOnly,textureOptions,None));
                value_vec.push(f.clone());

                return f;
            }
        }

        let f = Rc::new(Framebuffer::new(orientation,size,textureOnly,textureOptions,None));

        cache.insert(hash,FramebufferCacheValue::new(f.clone()));
        return f;


    }

    pub fn purgeAllUnassignedFramebuffer(&self){

        info!("-----------> release framebuffer {:?}",self.cache.borrow().len());
        self.cache.borrow_mut().clear();
        info!("-----------> release framebuffer {:?}",self.cache.borrow().len());

    }

}
