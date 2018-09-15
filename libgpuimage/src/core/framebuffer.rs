#![allow(snake_case_name)]
use core::GlContext;
use core::framebuffercache::FramebufferCache;
use core::sharedImageProcessingContext;
use gles_rust_binding::GLuint;
use gles_rust_binding::*;
use std::cell::Cell;
use std::rc::{Weak,Rc};
// framebuffer

#[derive(Copy, Clone)]
pub struct GPUTextureOptions {
    minFilter : GLenum,
    magFilter : GLenum,
    wrapS : GLenum,
    wrapT : GLenum,
    internalFormat : GLenum,
    format : GLenum,
    _type : GLenum
}

impl Default for GPUTextureOptions {
    fn default() -> Self {
        GPUTextureOptions {
            minFilter: GL_LINEAR,
            magFilter: GL_LINEAR,
            wrapS: GL_CLAMP_TO_EDGE,
            wrapT: GL_CLAMP_TO_EDGE,
            internalFormat: GL_RGBA,
            format: GL_BGRA,
            _type: GL_UNSIGNED_BYTE
        }
    }
}

#[derive(Clone)]
pub struct Framebuffer {
    pub size : GLSize,
    pub orientation: Cell<ImageOrientation>,
    pub hash: i64,
    pub texture: u32,
    framebuffer: u32,
    framebufferRetainCount: Cell<u32>,
    textureOptions: GPUTextureOptions,
    textureOverride: bool,


}

pub enum Rotation {
    noRotation,
    rotateCounterclockwise,
    rotateClockwise,
    rotate180,
    flipHorizontally,
    flipVertically,
    rotateClockwiseAndFlipVertically,
    rotateClockwiseAndFlipHorizontally
}

impl Rotation {
    fn flipsDimensions(&self) -> bool {
        match self {
            Rotation::noRotation | Rotation::rotate180 | Rotation::flipHorizontally | Rotation::flipVertically => false,
            _ => true
        }
    }
}
#[derive(Copy, Clone)]
pub enum ImageOrientation{
    portrait,
    portraitUpsideDown,
    landscapeLeft,
    landscapeRight
}

impl ImageOrientation {
    fn rotationNeededForOrientation(&self, targetOrientation: ImageOrientation) -> Rotation {
        match (self,targetOrientation) {
            (ImageOrientation::portrait, ImageOrientation::portrait) | (ImageOrientation::portraitUpsideDown, ImageOrientation::portraitUpsideDown)
            | (ImageOrientation::landscapeLeft,ImageOrientation::landscapeLeft) | (ImageOrientation::landscapeRight,ImageOrientation::landscapeRight) => Rotation::noRotation,

            (ImageOrientation::portrait, ImageOrientation::portraitUpsideDown)  |  (ImageOrientation::portraitUpsideDown, ImageOrientation::portrait)
            | (ImageOrientation::landscapeLeft, ImageOrientation::landscapeRight) | (ImageOrientation::landscapeRight, ImageOrientation::landscapeLeft) => Rotation::rotate180,

            (ImageOrientation::portrait, ImageOrientation::landscapeLeft) | (ImageOrientation::landscapeRight, ImageOrientation::portrait)
            | (ImageOrientation::landscapeLeft, ImageOrientation::portraitUpsideDown) | (ImageOrientation::portraitUpsideDown, ImageOrientation::landscapeRight) => Rotation::rotateCounterclockwise,

            (ImageOrientation::landscapeRight, ImageOrientation::portraitUpsideDown) | (ImageOrientation::landscapeLeft, ImageOrientation::portrait)
            | (ImageOrientation::portrait, ImageOrientation::landscapeRight) | (ImageOrientation::portraitUpsideDown, ImageOrientation::landscapeLeft) => Rotation::rotateClockwise
        }
    }
}

#[derive(Copy,Clone)]
pub struct GLSize {
    pub width : i32,
    pub height: i32
}
impl GLSize {
    pub fn new(width: i32, height: i32) -> Self {
        GLSize{width:width,height:height}
    }
}
impl Default for GLSize {
    fn default() -> Self {
        GLSize{width:0,height:0}
    }
}

#[derive(Copy, Clone)]
pub struct Size {
    pub width: f32,
    pub height: f32
}

pub fn hashForFramebufferWithProperties(size:GLSize, textureOnly:bool, textureOptions: GPUTextureOptions , stencil:bool ) -> i64 {
    let mut result:i64 = 1;
    let prime:i64 = 31;
    let yesPrime:i64 = 1231;
    let noPrime:i64 = 1237;

    result = prime * result + (size.width as i64);
    result = prime * result + (size.height as i64);
    result = prime * result + (textureOptions.internalFormat as i64);
    result = prime * result + (textureOptions.format as i64);
    result = prime * result + (textureOptions._type as i64);
    result = prime * result + (if textureOnly { yesPrime } else {noPrime});
    result = prime * result + (if stencil {yesPrime} else{ noPrime});
    return result
}

fn generateTexture(textureOptions: GPUTextureOptions) -> GLuint {
    let mut texture:GLuint = 0;

    unsafe {
        glActiveTexture(GL_TEXTURE1);
        glGenTextures(1, &mut texture);
        glBindTexture(GL_TEXTURE_2D, texture);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, textureOptions.minFilter as i32);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, textureOptions.magFilter as i32);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, textureOptions.wrapS as i32);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, textureOptions.wrapT as i32);

        glBindTexture(GL_TEXTURE_2D, 0);
    }

    texture
}
use std::ptr;
fn generateFramebufferForTexture(texture: GLuint, width: GLint, height: GLint, textureOptions:GPUTextureOptions) -> GLuint{
    let mut framebuffer : GLuint = 0;
    unsafe {
        glActiveTexture(GL_TEXTURE1);
        glGenFramebuffers(1,&mut framebuffer);
        glBindFramebuffer(GL_FRAMEBUFFER,framebuffer);
        glBindTexture(GL_TEXTURE_2D, texture);
        glTexImage2D(GL_TEXTURE_2D,0,textureOptions.internalFormat as i32,width,height,0,textureOptions.format,textureOptions._type,ptr::null());
        glFramebufferTexture2D(GL_FRAMEBUFFER,GL_COLOR_ATTACHMENT0,GL_TEXTURE_2D,texture,0);

        glBindTexture(GL_TEXTURE_2D,0);
        glBindFramebuffer(GL_FRAMEBUFFER,0);

        framebuffer

    }
}

impl Default for Framebuffer {
    fn default() -> Self{
        Framebuffer::new_default(ImageOrientation::portrait,GLSize::default(),false)
    }
}

impl Framebuffer {

    pub fn new_default(orientation: ImageOrientation, size: GLSize, textureOnly:bool) -> Self {
        let default = GPUTextureOptions::default();
        Framebuffer::new(orientation,size,textureOnly,default,Option::None)
    }


    pub fn new(orientation: ImageOrientation, size: GLSize, textureOnly: bool, textureOptions: GPUTextureOptions, overriddenTexture: Option<GLuint>) -> Self {
        let hash = hashForFramebufferWithProperties(size,textureOnly,textureOptions,false);

        let (textureOverride,texture) = match overriddenTexture {
            Some(newTexture) => (true,newTexture),
            None => {
                let texture = generateTexture(textureOptions);
                (false, texture)
            }
        };

        let framebuffer = if !textureOnly {
            generateFramebufferForTexture(texture, size.width, size.height, textureOptions)
        }else{
            0
        };

        Framebuffer{
            size: size,
            orientation:Cell::new(orientation),
            textureOptions:textureOptions,
            hash:hash,
            textureOverride:textureOverride,
            framebuffer:framebuffer,
            texture: texture,
            framebufferRetainCount: Cell::default()
        }

    }

    fn lock(&self){
        let newValue = self.framebufferRetainCount.get() + 1;
        self.framebufferRetainCount.set(newValue);
    }
    fn unlock(&self){
        let newValue = self.framebufferRetainCount.get() - 1;
        self.framebufferRetainCount.set(newValue);
        if newValue < 1 {
            self.framebufferRetainCount.set(0);
            sharedImageProcessingContext.frameubfferCache.returnToCache(self.clone())
        }



    }
    fn resetRetainCount(&self){
        self.framebufferRetainCount.set(0);
    }


    pub fn sizeForTargetOrientation(&self, targetOrientation: ImageOrientation) -> GLSize {

        let mut orientation = self.orientation.get();

        if orientation.rotationNeededForOrientation(targetOrientation).flipsDimensions() {
            GLSize{width:self.size.height,height:self.size.width}
        }else{
            self.size
        }
    }

    pub fn aspectRatioForRotation(&self, rotation: Rotation) -> f32 {
        if rotation.flipsDimensions() {
            (self.size.width as f32) / (self.size.height as f32)
        }else{
            (self.size.height as f32) / (self.size.width as f32)
        }
    }

    pub fn texelSize(&self, rotation: Rotation) -> Size {
        if rotation.flipsDimensions() {
            Size{width:1.0/(self.size.height as f32), height: 1.0/(self.size.width as f32)}
        }else{
            Size{width:1.0/(self.size.width as f32), height: 1.0/(self.size.height as f32)}
        }
    }

    pub fn initialStageTexelSize(&self, rotation: Rotation) -> Size {
        if rotation.flipsDimensions() {
            Size{width:1.0/(self.size.height as f32), height:0.0}
        }else{
            Size{width:0.0, height:1.0/(self.size.height as f32)}
        }
    }

    pub fn activateFramebufferForRendering(&self){
        unsafe {
            glBindFramebuffer(GL_FRAMEBUFFER, self.framebuffer);
            glViewport(0,0,self.size.width,self.size.height);
        }
    }




}

enum InputTextureStorageFormat {
    textureCoordinate(GLfloat),
    textureVBO(GLuint)
}

struct InputTextureProperties{
    textureStorage: InputTextureStorageFormat,
    texture: GLuint
}

impl InputTextureProperties {

}


impl Drop for Framebuffer {
    fn drop(&mut self) {
        if self.textureOverride {
            unsafe {
                glDeleteTextures(1,&mut self.texture)
            }
        }
        unsafe {
            glDeleteFramebuffers(1,&mut self.framebuffer);
        }
    }
}