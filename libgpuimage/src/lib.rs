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
// 内存释放检查
// 统一Android iOS接口
// 移除平台相关的代码，在平台相关直接使用GLKView或者GLSurfaceView
// 添加更多Filter
// 考虑移除Node接口，(或者使用新的Graph去渲染 创建Graph与释放Graph代价很低，Node节点包含共享环境)
// 考虑实现SubGraph (实现特殊效果时需要这个Operation)
// 为今后引入抽象图形API(GFX)留下接入空间
