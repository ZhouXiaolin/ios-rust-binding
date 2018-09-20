pub mod context;
pub mod framebuffer;
pub mod framebuffercache;
pub mod program;
pub mod gl_render;

pub use self::framebuffer::*;
pub use self::framebuffercache::*;
pub use self::context::*;
pub use gles_rust_binding::GLProgram as Program;
pub use gles_rust_binding::*;
#[repr(C)]
#[derive(Copy,Clone)]
pub enum NodeType{
    Picture,
    Camera,
    BasicFilter,
    GuassianBlurFilter,
    LookupTableFilter,
    ToneCurveFilter,
    View
}

impl NodeType {
    pub fn get_name(&self) -> &str {
        match self {
            NodeType::Picture => "NodeType::Picture",
            NodeType::Camera => "NodeType::Camera",
            NodeType::BasicFilter => "NodeType::BasicFilter",
            NodeType::GuassianBlurFilter => "NodeType::GuassianBlurFilter",
            NodeType::LookupTableFilter => "NodeType::LookupTableFilter",
            NodeType::ToneCurveFilter => "NodeType::ToneCurveFilter",
            NodeType::View => "NodeType::View"
        }
    }
}


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




pub struct Color {
    pub redComponent: f32,
    pub greenComponent: f32,
    pub blueComponent: f32,
    pub alphaComponent: f32
}

impl Color{
    pub fn new(redComponent: f32, greenComponent: f32, blueComponent: f32, alphaComponent: f32) -> Self{
        Color{redComponent:redComponent,greenComponent:greenComponent,blueComponent:blueComponent,alphaComponent:alphaComponent}
    }

    pub fn black() -> Self {
        Color::new(0.0,0.0,0.0,1.0)
    }

    pub fn white() -> Self {
        Color::new(1.0,1.0,1.0,1.0)
    }

    pub fn red() -> Self {
        Color::new(1.0, 0.0, 0.0,1.0)
    }

    pub fn green() -> Self {
        Color::new(0.0,1.0,0.0,1.0)
    }

    pub fn blue() -> Self {
        Color::new(0.0,0.0,1.0,1.0)
    }

    pub fn transparent() -> Self {
        Color::new(0.0,0.0,0.0,0.0)
    }


    pub fn toGLArray(&self) -> [GLfloat;3] {
        [self.redComponent as GLfloat,self.greenComponent as GLfloat,self.blueComponent as GLfloat]
    }

    pub fn toGLArrayWithAlpha(&self) -> [GLfloat;4] {
        [self.redComponent as GLfloat,self.greenComponent as GLfloat,self.blueComponent as GLfloat, self.alphaComponent as GLfloat]
    }

}


