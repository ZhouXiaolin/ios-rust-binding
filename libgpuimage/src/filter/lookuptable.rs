use super::basic::XHeyBasicFilter;
use core::{Node,NodeType};
#[repr(C)]
pub struct XHeyLookupTableFilter{
    basic: XHeyBasicFilter,
}

impl Node for XHeyLookupTableFilter {
    fn get_type_id() -> NodeType {
        NodeType::LookupTableFilter
    }
}

impl XHeyLookupTableFilter {
    pub fn new() -> Self {
        XHeyLookupTableFilter {
            basic: XHeyBasicFilter::new()
        }
    }
}