/// 这个模块和gl耦合非常高，需要进一步抽象
/// 滤镜是渲染操作的一个特化，渲染的一般是一个矩形，在片元上操作
///
mod basic;
mod picture;
mod combine;
mod lookup;


#[cfg(target_os = "ios")]
mod view;

#[cfg(target_os = "android")]
mod texture;
#[cfg(target_os = "android")]
mod surface_view;

pub use self::basic::*;
pub use self::picture::*;
pub use self::combine::*;
pub use self::lookup::*;


#[cfg(target_os = "ios")]
pub use self::view::*;

#[cfg(target_os = "android")]
pub use self::texture::*;
#[cfg(target_os = "android")]
pub use self::surface_view::*;

use super::common::*;
use super::structure::*;
use super::render::*;



