#![feature(nll)]
#![allow(non_camel_case_types,non_upper_case_globals)]
extern crate gles_rust_binding;
extern crate fnv;
#[cfg(target_os = "ios")]
extern crate ios_rust_binding;

pub mod core;
pub mod filter;
pub mod input;
pub mod output;

#[macro_use]
extern crate lazy_static;


