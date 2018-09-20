pub mod context;
pub mod framebuffer;
pub mod framebuffercache;
pub mod gl_render;
pub mod color;
pub mod gpu_texture_options;
pub mod position;
pub mod rotation;
pub mod image_orientation;
pub mod fill_mode;

pub use self::framebuffer::*;
pub use self::framebuffercache::*;
pub use self::gl_render::*;
pub use self::context::*;
pub use self::gpu_texture_options::*;
pub use self::position::*;
pub use self::rotation::*;
pub use self::image_orientation::*;
pub use self::fill_mode::*;

pub use gles_rust_binding::GLProgram as Program;
pub use gles_rust_binding::*;


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



