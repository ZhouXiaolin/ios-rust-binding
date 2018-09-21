use super::basic::XHeyBasicFilter;
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