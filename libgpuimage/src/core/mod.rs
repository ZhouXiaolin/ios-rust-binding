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
pub struct RenderNode {
    pub _type: NodeType
}

impl RenderNode {
    pub fn new(_type: NodeType) -> Self {
        RenderNode{_type:_type}
    }
    pub fn name(&self) -> &str {
        self._type.get_name()
    }
}

impl Node for RenderNode {
    fn get_type_name(&self) -> NodeType {
        self._type
    }
}

pub trait Source<'a>{
    fn add_target(&self, target: &'a dyn Consumer, _location: u32);
    fn remove_all_targets(&self);
}

pub trait Consumer {
    fn set_source(&self, _source: &dyn Source, _location: u32);
}






pub mod context;
pub mod framebuffer;
pub mod framebuffercache;

pub use core::context::{GlContext,SerialDispatch};

lazy_static!{
    pub static ref sharedImageProcessingContext : GlContext = GlContext::new();
}
