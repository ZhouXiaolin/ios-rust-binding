mod color;
mod fill_mode;
mod image_orientation;
mod position;
mod rotation;
mod size;

pub use self::color::*;
pub use self::fill_mode::*;
pub use self::image_orientation::*;
pub use self::position::*;
pub use self::rotation::*;
pub use self::size::*;

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