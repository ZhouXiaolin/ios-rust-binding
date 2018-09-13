#[macro_use]
extern crate objc;
extern crate objc_id;

pub use objc::*;
pub use objc_id::*;

use objc::declare::ClassDecl;
use objc::{Encode, Encoding,Message};
use objc::runtime::{BOOL, Class, NO, Object, Sel, YES};


pub type id = *mut Object;
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



pub struct UIView{
    _priv: ()
}
unsafe impl Message for UIView {}

impl UIView {
    pub fn class() -> &'static Class {
        Class::get("UIView").unwrap()
    }

    pub fn with_frame(frame : CGRect) -> Id<Self> {
        let cls = Self::class();
        unsafe {
            let obj : *mut Self = msg_send![cls, alloc];
            let obj : *mut Self = msg_send![obj, initWithFrame:frame];
            Id::from_retained_ptr(obj)
        }
    }

    pub fn get_layer(&self) -> Id<CALayer> {
        unsafe {
            let obj: *mut CALayer = msg_send![self, layer];
            Id::from_ptr(obj)
        }

    }

    pub fn set_background_color(&self, color: ShareId<UIColor>){

        unsafe {
            msg_send![self, setBackgroundColor:&*color];
        }
    }


}


pub struct CALayer {
    _priv : ()
}

unsafe impl Message for CALayer {}
impl CALayer {
    pub fn get_bounds(&self) -> CGRect {
        unsafe {
            msg_send![self, bounds]
        }
    }

    pub fn is_eagl_layer(&self) -> BOOL {
        unsafe {
            msg_send![self, isKindOfClass:class!(CAEAGLLayer)]
        }
    }



}

pub struct UIColor {
    _priv : ()
}

unsafe  impl Message for UIColor {}
impl UIColor {
    pub fn class() -> &'static Class {
        Class::get("UIColor").unwrap()
    }

    pub fn fromRgba(red: CGFloat, green: CGFloat, blue: CGFloat, alpha: CGFloat) -> Id<Self> {
        let cls = Self::class();
        unsafe {
            let obj : *mut Self = msg_send![cls, alloc];
            let obj : *mut Self = msg_send![obj, initWithRed:red green:green blue:blue alpha:alpha];
            Id::from_retained_ptr(obj)

        }

    }
}

pub struct UIImage {
    _priv: ()
}
unsafe impl Message for UIImage {}

impl UIImage {
    pub fn class() -> &'static Class {
        Class::get("UIImage").unwrap()
    }

    pub fn get_image(name: &str) -> Id<Self> {

        let cls = Self::class();
        unsafe {
            let ns_string = Class::get("NSString").unwrap();
            let path = name.as_ptr();
            let name: *mut Object = msg_send![ns_string, stringWithUTF8String:path];
            let obj:*mut Self = msg_send![cls, imageNamed:name];
            Id::from_retained_ptr(obj)

        }

    }

}


pub type EAGLRenderingAPI = std::os::raw::c_char;
pub const kEAGLRenderingAPIOpenGLES1 : EAGLRenderingAPI = 1;
pub const kEAGLRenderingAPIOpenGLES2 : EAGLRenderingAPI = 2;
pub const kEAGLRenderingAPIOpenGLES3 : EAGLRenderingAPI = 3;

pub struct EAGLContext {
    _priv:()
}
unsafe impl Message for EAGLContext {}

impl EAGLContext {
    pub fn class() -> &'static Class {
        Class::get("EAGLContext").expect("Failed to get Class `EAGLContext`")
    }

    pub fn withApi(api: EAGLRenderingAPI) -> Id<Self> {
        let cls = Self::class();
        unsafe {
            let obj: *mut Self = msg_send![cls, alloc];
            let obj: *mut Self = msg_send![obj, initWithAPI:api];
            Id::from_retained_ptr(obj)
        }
    }

    pub fn setCurrentContext(context: &ShareId<Self>) -> BOOL {
        let cls = Self::class();

        unsafe { msg_send![cls, setCurrentContext:&*(*context)]}
    }

    pub fn makeCurrentContext(context: &ShareId<Self>){
        let cls = Self::class();
        unsafe {
            let current: *mut Self = msg_send![cls, currentContext];
            let equal : BOOL = msg_send![current, isEqual:&*(*context)];
            if equal == NO {
                EAGLContext::setCurrentContext(context);
            }
        }
    }

    pub fn currentContext() -> Id<Self> {
        let cls = Self::class();
        unsafe {
            let obj :*mut Self = msg_send![cls, currentContext];
            Id::from_retained_ptr(obj)
        }
    }

    pub fn presentRenderBuffer(&self, target: NSUInteger) -> BOOL {
        unsafe {msg_send![self, presentRenderbuffer:target]}
    }

    pub fn renderBufferStorage(&self, target: NSUInteger, drawable: &ShareId<CALayer>) -> BOOL {
        let rect = drawable.get_bounds();
        println!("rect : width{} height{}",rect.size.width, rect.size.height);
        unsafe {msg_send![self, renderbufferStorage:target fromDrawable:&*(*drawable)]}
    }

}
