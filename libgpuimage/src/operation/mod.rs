/// 这个模块和gl耦合非常高，需要进一步抽象
/// 滤镜是渲染操作的一个特化，渲染的一般是一个矩形，在片元上操作
///
mod basic;
mod picture;
mod combine;

#[cfg(target_os = "ios")]
mod view;

#[cfg(target_os = "android")]
mod texture;

pub use self::basic::*;
pub use self::picture::*;
pub use self::combine::*;

#[cfg(target_os = "ios")]
pub use self::view::*;

#[cfg(target_os = "android")]
pub use self::texture::*;

use super::common::*;
use super::structure::*;
use super::render::*;

extern crate std;
extern crate gles_rust_binding;

#[cfg(target_os = "ios")]
extern crate ios_rust_binding;

