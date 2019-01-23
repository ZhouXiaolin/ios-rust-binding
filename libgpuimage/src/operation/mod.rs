/// 这个模块和gl耦合非常高，需要进一步抽象
/// 滤镜是渲染操作的一个特化，渲染的一般是一个矩形，在片元上操作
///
mod basic;
mod picture;
mod combine;
mod lookup;
mod alpha_blend;
mod surface_view;
mod picture_output;
mod unsharp_mask;
mod blend;
mod camera;
mod test;

#[cfg(target_os = "android")]
mod texture;

pub use self::basic::*;
pub use self::picture::*;
pub use self::combine::*;
pub use self::lookup::*;
pub use self::picture_output::*;
pub use self::alpha_blend::*;
pub use self::surface_view::*;
pub use self::unsharp_mask::*;
pub use self::blend::*;
pub use self::camera::*;

#[cfg(target_os = "android")]
pub use self::texture::*;

use super::structure::*;
use super::render::*;
use crate::render::common::*;
use std::os::raw::c_void;

