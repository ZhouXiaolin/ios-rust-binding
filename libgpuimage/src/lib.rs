#![allow(
non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
missing_copy_implementations, unused_imports,unused_variables,unused_assignments
)]




#[macro_use]
extern crate log;

pub mod operation;
pub mod render;
pub mod ffi;
pub mod structure;

extern crate fnv;
extern crate gles_rust_binding;


// 开发计划
// 统一Android iOS接口
// 添加更多Filter
// 使用新的Graph去渲染 创建Graph与释放Graph代价很低，只有在Forward(或者Backward)中才会产生Tensor结构
// 考虑实现SubGraph SubGraph有没有必要？ SubGraph是为了实现在特定情况下的更复杂的结构
// 考虑实现ImageGenerator 考虑加入Compute 这一步可以结合
// 考虑使用GFX抽象描述
