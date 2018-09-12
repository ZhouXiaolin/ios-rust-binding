use super::basic::XHeyBasicFilter;
use core::{Node,NodeType};
#[repr(C)]
pub struct XHeyGaussianBlurFilter{
    basic : XHeyBasicFilter,
}

impl Node for XHeyGaussianBlurFilter{
    fn get_type_id() -> NodeType {
        NodeType::GuassianBlurFilter
    }
}

impl XHeyGaussianBlurFilter {
    pub fn new() -> Self {
        XHeyGaussianBlurFilter{
            basic: XHeyBasicFilter::new()
        }
    }
}