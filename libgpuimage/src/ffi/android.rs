extern crate jni;


use super::operation::*;
use self::jni::JNIEnv;
use self::jni::objects::{JClass, JString};
use self::jni::sys::{jint, jlong};
use std::os::raw::c_void;

#[no_mangle]
pub unsafe extern "C" fn xhey_init_basic_filter(env: JNIEnv, _: JClass) -> *mut XHeyBasicFilter {
    let filter = Box::new(XHeyBasicFilter::new());
    Box::into_raw(filter)
}

#[no_mangle]
pub unsafe extern "C" fn xhey_init_basic_filter_2(env: JNIEnv, _: JClass) -> *mut XHeyBasicFilter {

    let vertexString = r#"
 attribute vec4 position;
 attribute vec4 inputTextureCoordinate;

 varying vec2 textureCoordinate;

 void main()
 {
     gl_Position = position;
     textureCoordinate = inputTextureCoordinate.xy;
 }
    "#;

    let fragmentString = r#"
 precision mediump float;

 varying highp vec2 textureCoordinate;
 uniform sampler2D inputImageTexture;

 void main()
 {
     vec4 color = texture2D(inputImageTexture, textureCoordinate);
     gl_FragColor = vec4(0.0,color.r, 0.0, 1.0);
 }
    "#;

    let filter = Box::new(XHeyBasicFilter::new_shader(vertexString,fragmentString,1));
    Box::into_raw(filter)
}


#[no_mangle]
pub unsafe extern "C" fn xhey_init_picture(env: JNIEnv, _: JClass,data: *const c_void, width: i32, height: i32) ->  *mut XheyPicture {
    println!("xhey_init_picture");
    let picture = Box::new(XheyPicture::new(data,width,height));
    Box::into_raw(picture)

}

#[no_mangle]
pub unsafe extern "C" fn xhey_process_picture(env: JNIEnv, _: JClass, picture: *const XheyPicture){

}