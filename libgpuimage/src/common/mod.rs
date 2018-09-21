pub mod color;
pub mod fill_mode;
pub mod image_orientation;
pub mod position;
pub mod rotation;
pub mod graph_context;
pub mod graph_node;

#[cfg(feature = "new")]
pub mod graph;

pub use self::color::*;
pub use self::fill_mode::*;
pub use self::image_orientation::*;
pub use self::position::*;
pub use self::rotation::*;
pub use self::graph_context::*;
pub use self::graph_node::*;

#[cfg(feature = "new")]
pub use self::graph::*;

pub use super::render::gl_render::{GLSize,Size};
pub use super::render::framebuffer::Framebuffer;




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