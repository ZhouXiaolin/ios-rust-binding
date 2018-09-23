use gles_rust_binding::*;
use super::sharedImageProcessingContext;
use super::gpu_texture_options::*;
use super::{Rotation,ImageOrientation,Size,GLSize};
use std::cell::Cell;
use std::ptr;
use super::Tensor;

pub enum InputTextureStorageFormat {
    textureCoordinate([GLfloat;8]),
    textureVBO(GLuint)
}

pub struct InputTextureProperties{
    pub textureStorage: InputTextureStorageFormat,
    pub texture: GLuint
}

impl InputTextureProperties {
    pub fn new(textureStorageFormat: InputTextureStorageFormat, texture:GLuint) -> Self {
        InputTextureProperties{
            textureStorage: textureStorageFormat,
            texture: texture
        }
    }
}


/// 严格来讲，不该是Clone语义，但这里只是标记，Clone语义不会影响帧缓冲。
#[derive(Clone)]
pub struct Framebuffer {
    pub size : GLSize,
    pub orientation: Cell<ImageOrientation>,
    pub texture: u32,
    hashString: String,
    framebuffer: u32,
    framebufferRetainCount: Cell<u32>,
    textureOptions: GPUTextureOptions,
    textureOverride: bool,


}

impl Tensor for Framebuffer{}

pub fn hashStringForFramebuffer(size:GLSize, textureOnly:bool, textureOptions: GPUTextureOptions) -> String {
    if textureOnly {
        let string = format!("NOFB-{}{}-{}{}{}{}{}{}{}",size.width, size.height, textureOptions.minFilter, textureOptions.magFilter, textureOptions.wrapS, textureOptions.wrapT, textureOptions.internalFormat, textureOptions.format, textureOptions._type);
        string
    }else{
        let string = format!("FB-{}{}-{}{}{}{}{}{}{}",size.width, size.height, textureOptions.minFilter, textureOptions.magFilter, textureOptions.wrapS, textureOptions.wrapT, textureOptions.internalFormat, textureOptions.format, textureOptions._type);
        string
    }
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

    pub fn hashString (&self) -> String {
        self.hashString.clone()
    }

    pub fn new(orientation: ImageOrientation, size: GLSize, textureOnly: bool, textureOptions: GPUTextureOptions, overriddenTexture: Option<GLuint>) -> Self {

        let hashString = hashStringForFramebuffer(size,textureOnly,textureOptions);

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
            hashString:hashString,
            textureOverride:textureOverride,
            framebuffer:framebuffer,
            texture: texture,
            framebufferRetainCount: Cell::default()
        }

    }

    pub fn lock(&self){
        let newValue = self.framebufferRetainCount.get() + 1;
        self.framebufferRetainCount.set(newValue);
    }

    pub fn retainCount(&self) -> u32 {
        self.framebufferRetainCount.get()
    }

    pub fn unlock(&self){
        let newValue = self.framebufferRetainCount.get() - 1;
        self.framebufferRetainCount.set(newValue);

        if newValue < 1 {

            self.resetRetainCount();

            sharedImageProcessingContext.frameubfferCache.returnToCache(self);
        }

    }

    pub fn resetRetainCount(&self){
        self.framebufferRetainCount.set(0);
    }

    pub fn valid(&self) -> bool {
        self.framebufferRetainCount.get() == 0
    }

    pub fn sizeForTargetOrientation(&self, targetOrientation: ImageOrientation) -> GLSize {

        let orientation = self.orientation.get();

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

    pub fn texturePropertiesForOutputRotation(&self, rotation:Rotation) -> InputTextureProperties {
        let storage = InputTextureStorageFormat::textureVBO(sharedImageProcessingContext.textureVBO(rotation));
        InputTextureProperties::new(storage,self.texture)
    }

    pub fn texturePropertiesForTargetOrientation(&self,targetOrientation: ImageOrientation) -> InputTextureProperties {

        self.texturePropertiesForOutputRotation(self.orientation.get().rotationNeededForOrientation(targetOrientation))
    }
    pub fn activateFramebufferForRendering(&self){
        unsafe {
            glBindFramebuffer(GL_FRAMEBUFFER, self.framebuffer);
            glViewport(0,0,self.size.width,self.size.height);
        }
    }




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


