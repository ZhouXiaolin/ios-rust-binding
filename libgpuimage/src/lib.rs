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


// 开发计划
// 正常释放内存
// 移除平台相关的代码，在平台相关直接使用GLKView或者GLSurfaceView
// 添加更多滤镜
// 对Graph进一步抽象 在生成Graph时加入Framebuffer的Cache
// 要做Compute,可以在各平台重新实现一个context环境
// 使用宏来减少代码 使用更rust的方式来减少转化损失
// 抽象各个平台渲染接口 GFX 长远目标
//