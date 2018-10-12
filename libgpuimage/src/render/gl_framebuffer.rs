use gles_rust_binding::*;
use super::sharedImageProcessingContext;
use super::gl_texture_options::*;
use super::{Rotation,ImageOrientation,Size,GLSize};
use std::cell::Cell;
use super::Tensor;
use std::ptr;

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


// 在渲染中，可能同时使用多个相同属性的fbo，需要缓存，缓存策略
#[derive(Debug)]
pub struct Framebuffer {
    pub size : GLSize,
    pub orientation: Cell<ImageOrientation>,
    pub texture: u32,
    hashString: String,
    pub framebuffer: u32,
    locked: Cell<i32>, // 引用计数
    textureOptions: GPUTextureOptions,
    textureOverride: bool,


}

impl Default for Framebuffer {
    fn default() -> Self{
        Framebuffer{
            size : GLSize::new(0,0),
            orientation: Cell::from(ImageOrientation::portrait),
            texture: 0,
            hashString: String::from(""),
            framebuffer: 0,
            locked: Cell::from(0),
            textureOptions: GPUTextureOptions::default(),
            textureOverride: false,
        }
    }
}

impl Tensor for Framebuffer{
    fn lock(&self){

        let v = self.locked.get();
        self.locked.set(v+1);
    }
    fn unlock(&self){
        let v = self.locked.get();
        self.locked.set(v-1);
    }
}


impl Framebuffer {


    pub fn new_default(orientation: ImageOrientation, size: GLSize, textureOnly:bool) -> Self {
        let default = GPUTextureOptions::default();
        Framebuffer::new(orientation,size,textureOnly,default,Option::None)
    }

    pub fn new_texture(orientation: ImageOrientation, size: GLSize, textureId:GLuint) -> Self {
        let default = GPUTextureOptions::default();
        Framebuffer::new(orientation,size,true,default,Some(textureId))
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
            size,
            orientation:Cell::new(orientation),
            textureOptions,
            hashString,
            textureOverride,
            framebuffer,
            texture,
            locked: Cell::from(0)
        }

    }

    pub fn valid(&self) -> bool {
        self.locked.get() == 0
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

        println!("Drop Framebuffer");
        if self.textureOverride == false {
            unsafe {
                glDeleteTextures(1,&mut self.texture);
                println!("Delete texture at size {:?}",self.size);
            }
        }
        unsafe {
            glDeleteFramebuffers(1,&mut self.framebuffer);
        }
    }
}



pub fn hashStringForFramebuffer(size:GLSize, textureOnly:bool, textureOptions: GPUTextureOptions) -> String {
    if textureOnly {
        let string = format!("NOFB-{}{}-{}{}{}{}{}{}{}",size.width, size.height, textureOptions.minFilter, textureOptions.magFilter, textureOptions.wrapS, textureOptions.wrapT, textureOptions.internalFormat, textureOptions.format, textureOptions._type);
        string
    }else{
        let string = format!("FB-{}{}-{}{}{}{}{}{}{}",size.width, size.height, textureOptions.minFilter, textureOptions.magFilter, textureOptions.wrapS, textureOptions.wrapT, textureOptions.internalFormat, textureOptions.format, textureOptions._type);
        string
    }
}

#[inline]
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

#[inline]
fn generateFramebufferForTexture(texture: GLuint, width: GLint, height: GLint, textureOptions:GPUTextureOptions) -> GLuint{
    let mut framebuffer : GLuint = 0;
    unsafe {
        glActiveTexture(GL_TEXTURE1);
        glGenFramebuffers(1,&mut framebuffer);
        glBindFramebuffer(GL_FRAMEBUFFER,framebuffer);
        glBindTexture(GL_TEXTURE_2D, texture);
        glTexImage2D(GL_TEXTURE_2D,0,textureOptions.internalFormat as i32,width,height,0,textureOptions.format,textureOptions._type,ptr::null());
        glFramebufferTexture2D(GL_FRAMEBUFFER,GL_COLOR_ATTACHMENT0,GL_TEXTURE_2D,texture,0);

        let status = glCheckFramebufferStatus(GL_FRAMEBUFFER);
        if status != GL_FRAMEBUFFER_COMPLETE {
            info!("Error framebuffer fail {}",status);
            panic!("Error framebuffer fail {}",status);
        }

        glBindTexture(GL_TEXTURE_2D,0);
        glBindFramebuffer(GL_FRAMEBUFFER,0);

        framebuffer

    }
}

