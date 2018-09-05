use core::{DataConsumer,DataSource};
use std::os::raw::c_void;
use std::rc::Rc;
use std::mem::transmute;
use objc::Message;
use objc_id::id::*;

use objc::declare::ClassDecl;
use objc::{Encode, Encoding};
use objc::runtime::{BOOL, Class, NO, Object, Sel, YES};

#[repr(C)]
pub struct XHeyView{
}


impl DataConsumer for XHeyView{
    fn set_source<T : DataSource>(&self, _source: &T, _location: u32){
        println!("XheyView set_source");

    }
}


#[allow(non_snake_case, unused_variables, dead_code)]
#[no_mangle]
pub extern "C" fn xhey_init_view(view: *mut XHeyView, source: *mut c_void){

    let _source = unsafe {transmute::<*mut c_void, Box<UIView>>(source)};
    let color = UIColor::from_rgba(1.0,0.0,0.0,1.0).share();

    _source.set_background_color(color);

    println!("xhey_init_view");
}

pub type id = *mut Object;
pub const nil: id = 0 as id;

pub const UIViewAutoresizingFlexibleWidth: NSUInteger = 1 << 1;
pub const UIViewAutoresizingFlexibleHeight: NSUInteger = 1 << 4;

#[cfg(target_pointer_width = "32")]
pub type CGFloat = f32;
#[cfg(target_pointer_width = "64")]
pub type CGFloat = f64;

#[cfg(target_pointer_width = "32")]
pub type NSUInteger = u32;
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = u64;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
}

unsafe impl Encode for CGRect {
    fn encode() -> Encoding {
        #[cfg(target_pointer_width = "32")]
            unsafe {
            Encoding::from_str("{CGRect={CGPoint=ff}{CGSize=ff}}")
        }
        #[cfg(target_pointer_width = "64")]
            unsafe {
            Encoding::from_str("{CGRect={CGPoint=dd}{CGSize=dd}}")
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}


struct UIView{
    _priv: ()
}
unsafe impl Message for UIView {}

impl UIView {
    fn class() -> &'static Class {
        Class::get("UIView").unwrap()
    }

    fn with_frame(frame : CGRect) -> Id<Self> {
        let cls = Self::class();
        unsafe {
            let obj : *mut Self = msg_send![cls, alloc];
            let obj : *mut Self = msg_send![obj, initWithFrame:frame];
            Id::from_retained_ptr(obj)
        }
    }

    fn set_background_color(&self, color: ShareId<UIColor>){
        unsafe {
            msg_send![self, setBackgroundColor:&*color];
        }
    }
}


struct UIColor {
    _priv : ()
}

unsafe  impl Message for UIColor {}
impl UIColor {
    fn class() -> &'static Class {
        Class::get("UIColor").unwrap()
    }

    fn from_rgba(red: CGFloat, green: CGFloat, blue: CGFloat, alpha: CGFloat) -> Id<Self> {
        let cls = Self::class();
        unsafe {
            let obj : *mut Self = msg_send![cls, alloc];
            let obj : *mut Self = msg_send![obj, initWithRed:red green:green blue:blue alpha:alpha];
            Id::from_retained_ptr(obj)
        }
    }
}