#![allow(
non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
missing_copy_implementations, unused_imports,unused_variables,unused_assignments
)]

extern crate gles_rust_binding;
extern crate fnv;
#[cfg(target_os = "ios")]
extern crate ios_rust_binding;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;

pub mod operation;
pub mod common;
pub mod render;
pub mod ffi;
pub mod structure;


