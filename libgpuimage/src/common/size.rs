#[derive(Copy,Clone,Debug,Default)]
pub struct Size {
    pub width: f32,
    pub height: f32
}


#[derive(Copy,Clone,Debug,Default)]
pub struct GLSize {
    pub width : i32,
    pub height: i32
}
impl GLSize {
    pub fn new(width: i32, height: i32) -> Self {
        GLSize{width:width,height:height}
    }
}