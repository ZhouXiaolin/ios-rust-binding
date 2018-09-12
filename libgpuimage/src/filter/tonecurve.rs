use super::basic::XHeyBasicFilter;
use core::{Node, NodeType};
#[repr(C)]
pub struct XHeyToneCurveFilter{
    base: XHeyBasicFilter,
}
impl Node for XHeyToneCurveFilter {
    fn get_type_id() -> NodeType {
        NodeType::ToneCurveFilter
    }
}

impl XHeyToneCurveFilter {
    pub fn new() -> Self {
        XHeyToneCurveFilter {
            base: XHeyBasicFilter::new()
        }
    }
}