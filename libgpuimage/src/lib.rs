#![allow(
non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
missing_copy_implementations, unused_imports,unused_variables,unused_assignments
)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

pub mod operation;
pub mod common;
pub mod render;
pub mod ffi;
pub mod structure;

extern crate fnv;
extern crate gles_rust_binding;


#[cfg(target_os = "ios")]
extern crate ios_rust_binding;


// 开发计划
// 正常释放内存
// 移除平台相关的代码，在平台相关直接使用GLKView或者GLSurfaceView
// 添加更多滤镜
