#![allow(
non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
missing_copy_implementations, unused_imports,unused_variables,unused_assignments
)]

extern crate gles_rust_binding;
extern crate fnv;
#[cfg(target_os = "ios")]
extern crate ios_rust_binding;

pub mod core;
pub mod filter;
pub mod input;
pub mod output;

pub use self::core::*;
pub use self::filter::*;
pub use self::input::*;
pub use self::output::*;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;


use self::filter::basic::*;

#[no_mangle]
pub extern "C" fn xhey_add_target<'a>(source: *mut XheyPicture<'a>, filter: *mut XHeyBasicFilter<'a>, filter2: *mut XHeyBasicFilter<'a>, consumer: *mut XHeyView){
    let box_picture = unsafe{source.as_ref().unwrap()};
    let box_filter = unsafe{filter.as_ref().unwrap()};
    let box_filter2 = unsafe{filter2.as_ref().unwrap()};

    let box_view = unsafe{consumer.as_ref().unwrap()};
    box_picture.addTarget(box_filter,0);
    box_filter.addTarget(box_filter2,0);
    box_filter2.addTarget(box_view,0);
}