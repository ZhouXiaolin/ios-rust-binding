mod color;
mod fill_mode;
mod image_orientation;
mod position;
mod rotation;
mod graph_node;
mod graph;
mod op;
mod tensor;

pub use self::color::*;
pub use self::fill_mode::*;
pub use self::image_orientation::*;
pub use self::position::*;
pub use self::rotation::*;
pub use self::graph_node::*;
pub use self::graph::*;
pub use self::op::*;
pub use self::tensor::*;

use super::render::{GLSize,Size};
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