use super::Position;
use super::Size;

#[derive(Copy, Clone, Debug, Default)]
pub struct Rect{
    pub position: Position,
    pub size: Size
}


impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            position: Position::new(x,y,0.0),
            size: Size::new(w,h)
        }
    }
}