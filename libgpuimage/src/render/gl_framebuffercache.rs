use super::fnv::FnvHashMap;
use super::std::cell::{RefCell,Ref};

use super::std::marker::Sync;
use super::{Framebuffer,GPUTextureOptions,GLSize,ImageOrientation};
use super::hashStringForFramebuffer;
use super::std::rc::Rc;



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
            panic!("why ?")
        }

    }

}


#[derive(Debug,Default)]
pub struct FramebufferCache{
    cache:RefCell<FnvHashMap<String,FramebufferCacheValue>>,
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
                println!("has key, find fbo vec");
                let mut value_vec = i.1.value.borrow_mut();
                for f in value_vec.iter() {
                    if f.valid() {
                        println!("fbo valid return");
                        return f.clone();
                    }
                }

                println!("fbo unvalid new framebuffer");
                let f = Rc::new(Framebuffer::new(orientation,size,textureOnly,textureOptions,None));
                value_vec.push(f.clone());
                return f;
            }
        }


        println!("new framebuffer");
        let f = Rc::new(Framebuffer::new(orientation,size,textureOnly,textureOptions,None));
        cache.insert(hash,FramebufferCacheValue::new(f.clone()));
        return f;


    }

    pub fn purgeAllUnassignedFramebuffer(&self){
        self.cache.borrow_mut().clear();
    }

}
