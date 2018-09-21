use super::basic::XHeyBasicFilter;
#[repr(C)]
pub struct XHeyGaussianBlurFilter{
    basic : XHeyBasicFilter,
}



impl XHeyGaussianBlurFilter {
    pub fn new() -> Self {
        XHeyGaussianBlurFilter{
            basic: XHeyBasicFilter::new()
        }
    }
}