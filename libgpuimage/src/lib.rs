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


// 开发计划
// 统一Android iOS接口
// 添加更多Filter
// 移除Node接口 这个remove可能会很复杂，破坏图的结构，性价比不如使用新的Graph去渲染
// 创建Graph与释放Graph代价很低，只有在forward计算中才会产生FBO
// 考虑实现SubGraph ???
// 考虑实现ImageGenerator
