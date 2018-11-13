
pub enum PrimitiveType {
    Point,
    Line,
    LineStrip,
    Triangle,
    TriangleStrip
}


pub trait PassData<'a> {}

pub trait CommandEncoder: for<'a> PassData<'a> {}
