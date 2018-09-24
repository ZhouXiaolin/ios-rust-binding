mod color;
mod fill_mode;
mod image_orientation;
mod position;
mod rotation;
mod size;
mod matrix3x3;
mod matrix4x4;

pub use self::color::*;
pub use self::fill_mode::*;
pub use self::image_orientation::*;
pub use self::position::*;
pub use self::rotation::*;
pub use self::size::*;
pub use self::matrix3x3::*;
pub use self::matrix4x4::*;

use super::render::Framebuffer;




/// PlaceHolder
use std::mem;
use std::marker::PhantomData;
pub struct PlaceHolder<T>{
    _priv: PhantomData<T>
}
impl<T> PlaceHolder<T> {
    pub fn new() -> T {
        unsafe{mem::uninitialized()}
    }
}