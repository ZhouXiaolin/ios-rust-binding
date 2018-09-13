use super::basic::XHeyBasicFilter;
use core::{Node,NodeType};
#[repr(C)]
pub struct XHeyGaussianBlurFilter<'a>{
    basic : XHeyBasicFilter<'a>,
}



impl<'a> XHeyGaussianBlurFilter<'a> {
    pub fn new() -> Self {
        XHeyGaussianBlurFilter{
            basic: XHeyBasicFilter::new()
        }
    }
}