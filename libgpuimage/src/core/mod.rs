use self::framebuffer::Framebuffer;
use gles_rust_binding::*;
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

pub trait Node {
    fn get_type_name(&self) -> NodeType;
}
pub struct RenderNode(NodeType);

impl RenderNode {
    pub fn new(_type: NodeType) -> Self {
        RenderNode(_type)
    }
    pub fn name(&self) -> &str {
        self.0.get_name()
    }
}

impl Node for RenderNode {
    fn get_type_name(&self) -> NodeType {
        self.0
    }
}


// 这两个trait描述滤镜链图的关系
// 更确切地说，滤镜关系是一张计算图，渲染方向就是前向计算Forward Compute， Graph = {Node Edge}
pub trait Source<'a>{
    fn add_target(&self, target: &'a dyn Consumer, _location: u32);
    fn remove_all_targets(&self);
    fn updateTargetsWithFramebuffer(&self, framebuffer:&Framebuffer);
}

pub trait Consumer {
    fn set_source(&self, _source: &dyn Source, _location: u32);
    fn newFramebufferAvailable(&self,framebuffer: &Framebuffer,fromSourceIndex: usize);
}






pub mod context;
pub mod framebuffer;
pub mod framebuffercache;

pub use core::context::{GlContext,SerialDispatch};

lazy_static!{
    pub static ref sharedImageProcessingContext : GlContext = GlContext::new();
}


pub fn clearFramebufferWithColor(color:Color) {
    unsafe {
        glClearColor(color.redComponent, color.greenComponent, color.blueComponent, color.alphaComponent);
        glClear(GL_COLOR_BUFFER_BIT);
    }
}

pub fn renderQuadWithShader(program: &GLProgram, framebuffer: &Framebuffer) {
    sharedImageProcessingContext.makeCurrentContext();
    unsafe {

        program.bind();

        let position = program.get_attribute("position");
        let textureCoordinate = program.get_attribute("inputTextureCoordinate");
        let inputTexture = program.get_uniform("inputImageTexture");


        let vertices:[f32;8] = [-1.0,1.0,1.0,1.0,-1.0,-1.0,1.0,-1.0];

        let textureCoordinates:[f32;8] = [1.0,1.0, 1.0,0.0, 0.0,1.0, 0.0,0.0];

        glClearColor(1.0,0.0,0.0,1.0);
        glClear(GL_COLOR_BUFFER_BIT);


        glVertexAttribPointer(position.location() as u32,2,GL_FLOAT,GL_FALSE,0,vertices.as_ptr() as *const _);
        glEnableVertexAttribArray(position.location() as u32);

        glVertexAttribPointer(textureCoordinate.location() as u32,2,GL_FLOAT,GL_FALSE,0,textureCoordinates.as_ptr() as *const _);
        glEnableVertexAttribArray(textureCoordinate.location() as u32);

        glActiveTexture(GL_TEXTURE0);
        glBindTexture(GL_TEXTURE_2D,framebuffer.texture);
        glUniform1i(0,inputTexture.location() as i32);

        glDrawArrays(GL_TRIANGLE_STRIP,0,4);


    }
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


}


