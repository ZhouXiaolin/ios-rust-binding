use super::basic::XHeyBasicFilter;
use core::{Node,NodeType};
#[repr(C)]
pub struct XHeyLookupTableFilter{
    basic: XHeyBasicFilter,
}



impl XHeyLookupTableFilter {
    pub fn new() -> Self {
        XHeyLookupTableFilter {
            basic: XHeyBasicFilter::new()
        }
    }
}