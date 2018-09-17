use super::basic::XHeyBasicFilter;
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