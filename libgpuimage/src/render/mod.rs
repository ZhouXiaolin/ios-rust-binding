pub mod context;
pub mod framebuffer;
pub mod framebuffercache;
pub mod gl_render;
pub mod gpu_texture_options;

pub use self::context::*;
pub use self::framebuffer::*;
pub use self::framebuffercache::*;
pub use self::gl_render::*;
pub use self::gpu_texture_options::*;

pub use super::common::*;


// 这两个trait描述滤镜链图的关系
// 更确切地说，滤镜关系是一张计算图，渲染方向就是前向计算Forward Compute， Graph = {Node Edge}


pub trait Source<'a>{
    fn addTarget(&self, target: &'a dyn Consumer, _location: u32);
    fn removeAllTargets(&self);
    fn updateTargetsWithFramebuffer(&self, framebuffer:&Framebuffer);
}

pub trait Consumer {
    fn setSource(&self, _source: &dyn Source, _location: u32);
    fn newFramebufferAvailable(&self,framebuffer: &Framebuffer,fromSourceIndex: usize);
}







lazy_static!{
    pub static ref sharedImageProcessingContext : GlContext = GlContext::new();
}



