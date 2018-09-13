use super::basic::XHeyBasicFilter;
use core::{Node, NodeType};
#[repr(C)]
pub struct XHeyToneCurveFilter<'a>{
    base: XHeyBasicFilter<'a>,
}


impl<'a> XHeyToneCurveFilter<'a> {
    pub fn new() -> Self {
        XHeyToneCurveFilter {
            base: XHeyBasicFilter::new()
        }
    }
}