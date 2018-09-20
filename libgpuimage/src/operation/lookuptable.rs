use super::basic::XHeyBasicFilter;
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