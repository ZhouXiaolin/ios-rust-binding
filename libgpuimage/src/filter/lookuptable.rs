use super::basic::XHeyBasicFilter;
use core::{Node,NodeType};
#[repr(C)]
pub struct XHeyLookupTableFilter<'a>{
    basic: XHeyBasicFilter<'a>,
}



impl<'a> XHeyLookupTableFilter<'a> {
    pub fn new() -> Self {
        XHeyLookupTableFilter {
            basic: XHeyBasicFilter::new()
        }
    }
}