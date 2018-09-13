#![feature(nll)]
extern crate gles_rust_binding;
#[cfg(target_os = "ios")]
extern crate ios_rust_binding;

pub mod core;
pub mod filter;
pub mod input;
pub mod output;

#[macro_use]
extern crate lazy_static;


