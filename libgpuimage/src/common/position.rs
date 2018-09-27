#[derive(Copy, Clone, Debug, Default)]
pub struct Position{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Position{
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Position{x:x,y:y,z:z}
    }
    pub fn center() -> Self {
        Position::new(0.5,0.5,0.0)
    }
    pub fn zero() -> Self {
        Position::new(0.0,0.0,0.0)
    }
}