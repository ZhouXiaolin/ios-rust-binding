#[cfg(target_os = "ios")]
#[allow(non_snake_case, unused_variables, dead_code)]
pub mod ios;

#[cfg(target_os="android")]
#[allow(non_snake_case, unused_variables, dead_code)]
pub mod android;


pub use super::*;

extern crate std;
#[cfg(target_os = "ios")]
extern crate ios_rust_binding;