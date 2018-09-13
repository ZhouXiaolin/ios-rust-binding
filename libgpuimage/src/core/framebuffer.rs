#![allow(snake_case_name)]
use core::GlContext;
use gles_rust_binding::GLuint;
use gles_rust_binding::*;
pub struct Framebuffer {
    size : GLSize,
    orientation: ImageOrientation,
    internalFormat: i32,
    format: i32,
    _type: i32,
    hash: i64,
    textureOverride: bool,
    framebuffer: u32,
    pub texture: u32

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

pub fn hashForFramebufferWithProperties(size:GLSize, textureOnly:bool, minFilter:i32, magFilter:i32, wrapS:i32 , wrapT:i32 , internalFormat:i32 , format:i32 , _type:i32 , stencil:bool ) -> i64 {
    let mut result:i64 = 1;
    let prime:i64 = 31;
    let yesPrime:i64 = 1231;
    let noPrime:i64 = 1237;

    result = prime * result + (size.width as i64);
    result = prime * result + (size.height as i64);
    result = prime * result + (internalFormat as i64);
    result = prime * result + (format as i64);
    result = prime * result + (_type as i64);
    result = prime * result + (if textureOnly { yesPrime } else {noPrime});
    result = prime * result + (if stencil {yesPrime} else{ noPrime});
    return result
}

fn generateTexture(minFilter:i32, magFilter:i32, wrapS:i32, wrapT:i32) -> GLuint {
    let mut texture:GLuint = 0;

    unsafe {
        glActiveTexture(GL_TEXTURE1);
        glGenTextures(1, &mut texture);
        glBindTexture(GL_TEXTURE_2D, texture);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, minFilter);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, magFilter);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, wrapS);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, wrapT);

        glBindTexture(GL_TEXTURE_2D, 0);
    }

    texture
}
use std::ptr;
fn generateFramebufferForTexture(texture: GLuint, width: GLint, height: GLint, internalFormat: i32, format: i32, _type:i32) -> GLuint{
    let mut framebuffer : GLuint = 0;
    unsafe {
        glActiveTexture(GL_TEXTURE1);
        glGenFramebuffers(1,&mut framebuffer);
        glBindFramebuffer(GL_FRAMEBUFFER,framebuffer);
        glBindTexture(GL_TEXTURE_2D, texture);
        glTexImage2D(GL_TEXTURE_2D,0,internalFormat,width,height,0,format as u32,_type as u32,ptr::null());
        glFramebufferTexture2D(GL_FRAMEBUFFER,GL_COLOR_ATTACHMENT0,GL_TEXTURE_2D,texture,0);

        glBindTexture(GL_TEXTURE_2D,0);
        glBindFramebuffer(GL_FRAMEBUFFER,0);

        framebuffer

    }
}

impl Framebuffer {

    pub fn new_default(orientation: ImageOrientation, size: GLSize, textureOnly:bool) -> Self {
        Framebuffer::new(orientation,size,textureOnly,GL_LINEAR as i32,GL_LINEAR as i32,GL_CLAMP_TO_EDGE as i32,GL_CLAMP_TO_EDGE as i32,GL_RGBA as i32,GL_BGRA as i32,GL_UNSIGNED_BYTE as i32,Option::None)
    }

    pub fn new(orientation: ImageOrientation, size: GLSize, textureOnly: bool, minFilter: i32, magFilter: i32, wrapS: i32, wrapT: i32, internalFormat: i32, format: i32, _type: i32, overriddenTexture: Option<GLuint>) -> Self {
        let hash = hashForFramebufferWithProperties(size,textureOnly,minFilter,magFilter,wrapS,wrapT,internalFormat,format,_type,false);

        let (textureOverride,texture) = match overriddenTexture {
            Some(newTexture) => (true,newTexture),
            None => {
                let texture = generateTexture(minFilter,magFilter,wrapS,wrapT);
                (false, texture)
            }
        };

        let framebuffer = if !textureOnly {
            generateFramebufferForTexture(texture, size.width, size.height, internalFormat, format,_type)
        }else{
            0
        };

        Framebuffer{
            size: size,
            orientation:orientation,
            internalFormat:internalFormat,
            format:format,
            _type:_type,
            hash:hash,
            textureOverride:textureOverride,
            framebuffer:framebuffer,
            texture: texture
        }

    }


    fn sizeForTargetOrientation(&self, targetOrientation: ImageOrientation) -> GLSize {
        if self.orientation.rotationNeededForOrientation(targetOrientation).flipsDimensions() {
            GLSize{width:self.size.height,height:self.size.width}
        }else{
            self.size
        }
    }

    fn aspectRatioForRotation(&self, rotation: Rotation) -> f32 {
        if rotation.flipsDimensions() {
            (self.size.width as f32) / (self.size.height as f32)
        }else{
            (self.size.height as f32) / (self.size.width as f32)
        }
    }

    fn texelSize(&self, rotation: Rotation) -> Size {
        if rotation.flipsDimensions() {
            Size{width:1.0/(self.size.height as f32), height: 1.0/(self.size.width as f32)}
        }else{
            Size{width:1.0/(self.size.width as f32), height: 1.0/(self.size.height as f32)}
        }
    }

    fn initialStageTexelSize(&self, rotation: Rotation) -> Size {
        if rotation.flipsDimensions() {
            Size{width:1.0/(self.size.height as f32), height:0.0}
        }else{
            Size{width:0.0, height:1.0/(self.size.height as f32)}
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