/// 这个应当进一步抽象
mod gl_context;
mod gl_framebuffer;
mod gl_framebuffercache;
mod gl_render;
mod gl_texture_options;
mod gl_shader_uniform_settings;

pub use self::gl_context::*;
pub use self::gl_framebuffer::*;
pub use self::gl_framebuffercache::*;
pub use self::gl_render::*;
pub use self::gl_texture_options::*;
pub use self::gl_shader_uniform_settings::*;

use super::common::*;
use super::structure::{Tensor,Context};

extern crate std;
extern crate fnv;
extern crate gles_rust_binding;


#[cfg(target_os = "ios")]
extern crate ios_rust_binding;








