use super::basic::XHeyBasicFilter;
use core::{Node, NodeType};
#[repr(C)]
pub struct XHeyToneCurveFilter{
    base: XHeyBasicFilter,
}


impl XHeyToneCurveFilter {
    pub fn new() -> Self {
        XHeyToneCurveFilter {
            base: XHeyBasicFilter::new()
        }
    }
}