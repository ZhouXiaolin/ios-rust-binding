#[derive(Copy,Clone,Debug,Default)]
pub struct Size {
    pub width: f32,
    pub height: f32
}
impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height
        }
    }
}


#[derive(Copy,Clone,Debug,Default)]
pub struct GLSize {
    pub width : i32,
    pub height: i32
}
impl GLSize {
    pub fn new(width: i32, height: i32) -> Self {
        GLSize{width,height}
    }
}




