use ios_rust_binding::{UIView,NSUInteger,ShareId,CALayer};
use gles_rust_binding::*;
use std::cell::{Cell,RefCell};
use std::rc::Rc;
use std::os::raw::c_void;
use super::*;


#[derive(Debug)]
#[repr(C)]
pub struct XheyTestView {
    displayFramebuffer: Cell<GLuint>,
    displayRenderbuffer: Cell<GLuint>,
    backingSize: Cell<GLSize>,
    layer: ShareId<CALayer>,
    orientation: ImageOrientation,
    imageTexture: Framebuffer,
    uniformSettings:ShaderUniformSettings,

}


impl Drop for XheyTestView {
    fn drop(&mut self) {
        println!("Drop");
        self.destroyDisplayFramebuffer();
    }
}

#[cfg(target_os="ios")]
static  FORMAT : GLenum = GL_BGRA;
#[cfg(target_os="android")]
static  FORMAT : GLenum = GL_RGBA;

impl XheyTestView {
    pub fn new(view: &UIView, data: *const c_void, width: i32, height: i32) -> Self {
        sharedImageProcessingContext.makeCurrentContext();
        let layer = view.get_layer();
        let layer = layer.share();
        let size = GLSize::new(width,height);


        let imageTexture = Framebuffer::new_default(ImageOrientation::portrait, size, true);

        unsafe {
            glBindTexture(GL_TEXTURE_2D,imageTexture.texture);
            glTexImage2D(GL_TEXTURE_2D,0,GL_RGBA as i32,width,height,0,FORMAT,GL_UNSIGNED_BYTE,data as *const _);
            glBindTexture(GL_TEXTURE_2D,0);
        }


        XheyTestView {
            displayFramebuffer: Cell::default(),
            displayRenderbuffer: Cell::default(),
            backingSize: Cell::default(),
            layer,
            orientation: ImageOrientation::portrait,
            imageTexture,
            uniformSettings: ShaderUniformSettings::default()
        }

    }


    fn activateDisplayFramebuffer(&self) {
        unsafe {
            glBindFramebuffer(GL_FRAMEBUFFER,self.displayFramebuffer.get());
            glViewport(0,0,self.backingSize.get().width,self.backingSize.get().height);
        }
    }


    fn createDisplayFramebuffer(&self){
        unsafe {
            let mut frameBuffer : GLuint = 0;
            glGenFramebuffers(1,&mut frameBuffer);
            self.displayFramebuffer.set(frameBuffer);
            glBindFramebuffer(GL_FRAMEBUFFER, frameBuffer);


            let mut colorRenderBuffer : GLuint = 0;
            glGenRenderbuffers(1,&mut colorRenderBuffer);
            self.displayRenderbuffer.set(colorRenderBuffer);
            glBindRenderbuffer(GL_RENDERBUFFER,colorRenderBuffer);

            sharedImageProcessingContext.context.renderBufferStorage(GL_RENDERBUFFER as NSUInteger,&self.layer);


            let mut backingWidth : GLint = 0;
            let mut backingHeight : GLint = 0;

            glGetRenderbufferParameteriv(GL_RENDERBUFFER, GL_RENDERBUFFER_WIDTH, &mut backingWidth);
            glGetRenderbufferParameteriv(GL_RENDERBUFFER, GL_RENDERBUFFER_HEIGHT, &mut backingHeight);

            self.backingSize.set(GLSize::new(backingWidth,backingHeight));

            glFramebufferRenderbuffer(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0,GL_RENDERBUFFER, colorRenderBuffer);


        }
    }


    fn destroyDisplayFramebuffer(&self) {
        unsafe {
            let mut displayFramebuffer = self.displayFramebuffer.get();
            if displayFramebuffer > 0 {
                glDeleteFramebuffers(1,&mut displayFramebuffer);
            }
            let mut displayRenderbuffer = self.displayRenderbuffer.get();
            if displayRenderbuffer > 0 {
                glDeleteRenderbuffers(1,&mut displayRenderbuffer);
            }
        }
    }



    pub fn display(&self) {

        self.render(&self.imageTexture);
    }

    fn render(&self, framebuffer:&Framebuffer) {
        sharedImageProcessingContext.makeCurrentContext();
        if self.displayFramebuffer.get() == 0 {
            self.createDisplayFramebuffer();
        }

        self.activateDisplayFramebuffer();
        clearFramebufferWithColor(Color::black());

        let program = &sharedImageProcessingContext.passthroughShader;


        let verticallyInvertedImageVertices: [f32;8] = [-1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0];

        let scaledVertices = FillMode::preserveAspectRatio.transformVertices(verticallyInvertedImageVertices,framebuffer.sizeForTargetOrientation(self.orientation),self.backingSize.get());


        let inputTexture = framebuffer.texturePropertiesForTargetOrientation(self.orientation);

        let vertex = InputTextureStorageFormat::textureCoordinate(scaledVertices);

        renderQuadWithShader(program,&self.uniformSettings,&vec![inputTexture],vertex);

        unsafe {
            glBindRenderbuffer(GL_RENDERBUFFER,self.displayRenderbuffer.get());
        }

        sharedImageProcessingContext.presentBufferForDisplay();

    }

}


