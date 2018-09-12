#[repr(C)]
pub enum NodeType{
    Picture,
    Camera,
    BasicFilter,
    GuassianBlurFilter,
    LookupTableFilter,
    ToneCurveFilter,
    View
}
pub trait Node {
    fn get_type_id() -> NodeType;
}

pub trait Source : Node {
    fn add_target<T : Consumer>(&self, target: &T, _location: u32);
    fn remove_all_targets();
}

pub trait Consumer : Node {
    fn set_source<T : Source>(&self, _source: &T, _location: u32);
}



pub mod context;
pub mod framebuffer;
pub mod framebuffercache;

pub use core::context::GlContext;