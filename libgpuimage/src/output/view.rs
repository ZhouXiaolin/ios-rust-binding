#[cfg(target_os = "ios")]
use ios_rust_binding::{UIView,NSUInteger,ShareId,CALayer};

use gles_rust_binding::*;

use super::{RenderNode,GLSize,Consumer,Source,Framebuffer,Color,NodeType,ImageOrientation,InputTextureProperties,InputTextureStorageFormat};
use super::GLRender::*;
use super::FillMode;
use super::sharedImageProcessingContext;
use super::color::*;
use super::gl_render::*;
use std::cell::Cell;
use std::ptr;


#[cfg(target_os = "ios")]
#[repr(C)]
pub struct XHeyView {
    displayFramebuffer: Cell<GLuint>,
    displayRenderbuffer: Cell<GLuint>,
    backingSize: Cell<GLSize>,
    layer: ShareId<CALayer>,
    orientation: ImageOrientation
}



#[cfg(target_os = "ios")]
impl Consumer for XHeyView {
    fn setSource(&self, _source: &dyn Source, _location: u32) {
        println!("XheyView set_source");

    }

    fn newFramebufferAvailable(&self,framebuffer: &Framebuffer, fromSourceIndex: usize){
        sharedImageProcessingContext.makeCurrentContext();

        if self.displayFramebuffer.get() == 0 {
            self.createDisplayFramebuffer()
        }


        self.activateDisplayFramebuffer();
        clearFramebufferWithColor(Color::black());

        let program = &sharedImageProcessingContext.passthroughShader;

        let verticallyInvertedImageVertices: [f32;8] = [-1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0];

        let scaledVertices = FillMode::preserveAspectRatio.transformVertices(verticallyInvertedImageVertices,framebuffer.sizeForTargetOrientation(self.orientation),self.backingSize.get());

        println!("scaledVertices: {:?}",scaledVertices);

        let inputTexture = framebuffer.texturePropertiesForTargetOrientation(self.orientation);

        let vertex = InputTextureStorageFormat::textureCoordinate(scaledVertices);

        renderQuadWithShader(program,&vec![inputTexture],vertex);

        unsafe {
            glBindRenderbuffer(GL_RENDERBUFFER,self.displayRenderbuffer.get());
        }
        sharedImageProcessingContext.presentBufferForDisplay();

    }

}



#[cfg(target_os = "ios")]
impl XHeyView {
    pub fn new(view: &UIView) -> Self {
        let layer = view.get_layer();
        let layer = layer.share();


        XHeyView{
            displayFramebuffer:Cell::default(),
            displayRenderbuffer:Cell::default(),
            backingSize:Cell::default(),
            layer:layer,
            orientation: ImageOrientation::portrait
        }
    }


    fn activateDisplayFramebuffer(&self) {
        unsafe {
            glBindBuffer(GL_FRAMEBUFFER,self.displayRenderbuffer.get());
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


}

