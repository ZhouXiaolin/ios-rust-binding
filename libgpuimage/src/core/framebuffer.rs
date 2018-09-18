use gles_rust_binding::*;
use super::sharedImageProcessingContext;
use std::cell::Cell;
use std::ptr;

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

bitflags!{
    struct TimestampFlags : u32 {
        const valid = 1 << 0;
        const hasBeenRounded = 1 << 1;
        const positiveInfinity = 1 << 2;
        const negativeInfinity = 1 << 3;
        const indefinite = 1 << 4;
    }
}

pub struct Timestamp{
    value: i64,
    timescale: i64,
    flag: TimestampFlags,
    epoch: i64
}



pub enum FramebufferTimingStyle {
    stillImage,
    videoFrame()
}

#[derive(Clone)] // 严格来讲，不该是Clone语义，但这里只是标记，Clone语义不会影响帧缓冲。
pub struct Framebuffer {
    pub size : GLSize,
    pub orientation: Cell<ImageOrientation>, // Cell 在cache中可能修改
    pub texture: u32,
    hashString: String,
    framebuffer: u32,
    framebufferRetainCount: Cell<u32>,
    textureOptions: GPUTextureOptions,
    textureOverride: bool,


}

pub struct Position{
    x: f32,
    y: f32,
    z: f32
}

impl Position{
    fn new(x: f32, y: f32, z: f32) -> Self {
        Position{x:x,y:y,z:z}
    }
    fn center() -> Self {
        Position::new(0.5,0.5,0.0)
    }
    fn zero() -> Self {
        Position::new(0.0,0.0,0.0)
    }
}
pub enum Rotation {
    noRotation,
    rotateCounterclockwise,
    rotateClockwise,
    rotate180,
    flipHorizontally,
    flipVertically,
    rotateClockwiseAndFlipVertically,
    rotateClockwiseAndFlipHorizontally,
}



impl Rotation {
    pub fn toRawValue(&self) -> usize {
        match self {
            Rotation::noRotation => 0,
            Rotation::rotateCounterclockwise => 1,
            Rotation::rotateClockwise => 2,
            Rotation::rotate180 => 3,
            Rotation::flipHorizontally => 4,
            Rotation::flipVertically => 5,
            Rotation::rotateClockwiseAndFlipVertically => 6,
            Rotation::rotateClockwiseAndFlipHorizontally => 7
        }
    }
    pub fn flipsDimensions(&self) -> bool {

        match self {
            Rotation::noRotation | Rotation::rotate180 | Rotation::flipHorizontally | Rotation::flipVertically => false,
            _ => true
        }
    }

    pub fn textureCoordinates(&self) -> [f32;8] {

        match self {
            Rotation::noRotation => [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0],
            Rotation::rotateCounterclockwise => [0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0],
            Rotation::rotateClockwise =>[1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0],
            Rotation::rotate180 => [1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0],
            Rotation::flipHorizontally => [1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0],
            Rotation::flipVertically => [0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0],
            Rotation::rotateClockwiseAndFlipVertically => [0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0],
            Rotation::rotateClockwiseAndFlipHorizontally => [1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0]
        }
    }

    pub fn croppedTextureCoordinates(&self, offsetFromOrigin:Position, cropSize: Size) -> [f32;8] {
        let minX = offsetFromOrigin.x;
        let minY = offsetFromOrigin.y;
        let maxX = offsetFromOrigin.x + cropSize.width;
        let maxY = offsetFromOrigin.y + cropSize.height;

        match self {
            Rotation::noRotation => [minX, minY, maxX, minY, minX, maxY, maxX, maxY],
            Rotation::rotateCounterclockwise => [minX, maxY, minX, minY, maxX, maxY, maxX, minY],
            Rotation::rotateClockwise => [maxX, minY, maxX, maxY, minX, minY, minX, maxY],
            Rotation::rotate180 => [maxX, maxY, minX, maxY, maxX, minY, minX, minY],
            Rotation::flipHorizontally => [maxX, minY, minX, minY, maxX, maxY, minX, maxY],
            Rotation::flipVertically => [minX, maxY, maxX, maxY, minX, minY, maxX, minY],
            Rotation::rotateClockwiseAndFlipVertically => [minX, minY, minX, maxY, maxX, minY, maxX, maxY],
            Rotation::rotateClockwiseAndFlipHorizontally => [maxX, maxY, maxX, minY, minX, maxY, minX, minY],
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
    pub fn rotationNeededForOrientation(&self, targetOrientation: ImageOrientation) -> Rotation {
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
    pub fn unlock(&self){
        let newValue = self.framebufferRetainCount.get() - 1;
        self.framebufferRetainCount.set(newValue);

        if newValue < 1 {

            println!("return To Cache");
            self.resetRetainCount();

            sharedImageProcessingContext.frameubfferCache.returnToCache(self);
        }

    }
    fn resetRetainCount(&self){
        self.framebufferRetainCount.set(0);
    }

    fn valid(&self) -> bool {
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
        InputTextureProperties::new(Some(rotation.textureCoordinates()),None,self.texture as GLuint)
//        let vbo = sharedImageProcessingContext.textureVBO(rotation);
//        InputTextureProperties::new(None,Some(vbo),self.texture)
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

pub enum InputTextureStorageFormat {
    textureCoordinate([GLfloat;8]),
    textureVBO(GLuint)
}

pub struct InputTextureProperties{
    pub textureStorage: InputTextureStorageFormat,
    pub texture: GLuint
}

impl InputTextureProperties {
    pub fn new(textureCoordinates: Option<[GLfloat;8]>, textureVBO:Option<GLuint>, texture:GLuint) -> Self {

        match (textureCoordinates, textureVBO) {
            (Some(coordinates),None) => {
                InputTextureProperties{
                    textureStorage:InputTextureStorageFormat::textureCoordinate(coordinates),
                    texture:texture
                }
            },
            (None,Some(vbo)) => {
                InputTextureProperties{
                    textureStorage:InputTextureStorageFormat::textureVBO(vbo),
                    texture:texture
                }
            },
            (None,None) => {
                panic!("Need to specify either texture coordinates or a VBO to InputTextureProperties");
            },
            (Some(_),Some(_)) => {
                panic!("Can't specify both texture coordinates and a VBO to InputTextureProperties");
            }
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


pub enum FillMode {
    stretch,
    preserveAspectRatio,
    preserveAspectRatioAndFill
}

impl FillMode {
    pub fn transformVertices(&self, vertices: [f32;8], fromInputSize : GLSize, toFitSize: GLSize) -> [f32;8] {


        let aspectRatio = (fromInputSize.height as f32) / (fromInputSize.width as f32);
        let targetAspectRatio = (toFitSize.height as f32) / (toFitSize.width as f32);

        let (xRatio, yRatio) =  match self {
            FillMode::stretch => {
                (1.0,1.0)
            },
            FillMode::preserveAspectRatio => {
                if aspectRatio > targetAspectRatio {
                    let x = fromInputSize.width as f32 / toFitSize.width as f32 * ( toFitSize.height as f32 / fromInputSize.height as f32);
                    (x,1.0)
                }else{
                    let y = fromInputSize.height as f32 / toFitSize.height as f32 * ( toFitSize.width as f32 / fromInputSize.width as f32);
                    (1.0,y)
                }
            },
            FillMode::preserveAspectRatioAndFill => {
                if aspectRatio > targetAspectRatio {
                    let y = fromInputSize.height as f32 / toFitSize.height as f32 * (toFitSize.width as f32 / fromInputSize.width as f32);
                    (1.0,y)
                }else {
                    let x = toFitSize.height as f32 / fromInputSize.height as f32 * (fromInputSize.width as f32 / toFitSize.width as f32);
                    (x,1.0)
                }
            }
        };

        let xConversionRatio = xRatio * (toFitSize.width as f32) / 2.0;
        let xConversionDivisor = (toFitSize.width as f32) / 2.0;
        let yConversionRatio = yRatio * (toFitSize.height as f32) / 2.0;
        let yConversionDivisor = (toFitSize.height as f32) / 2.0;

        let value1 = vertices[0] * xConversionRatio / xConversionDivisor;
        let value2 = vertices[1] * yConversionRatio / yConversionDivisor;
        let value3 = vertices[2] * xConversionRatio / xConversionDivisor;
        let value4 = vertices[3] * yConversionRatio / yConversionDivisor;
        let value5 = vertices[4] * xConversionRatio / xConversionDivisor;
        let value6 = vertices[5] * yConversionRatio / yConversionDivisor;
        let value7 = vertices[6] * xConversionRatio / xConversionDivisor;
        let value8 = vertices[7] * yConversionRatio / yConversionDivisor;

        return [value1, value2, value3, value4, value5, value6, value7, value8]
    }
}