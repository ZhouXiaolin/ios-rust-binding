use ios_rust_binding::{UIView,NSUInteger,ShareId,CALayer};

use gles_rust_binding::*;

use super::{RenderNode,GLSize,Consumer,Source,Framebuffer,Color,NodeType,ImageOrientation,InputTextureProperties,InputTextureStorageFormat};
use super::GLRender::*;
use super::FillMode;
use super::sharedImageProcessingContext;
use super::GLRender::*;
use std::cell::Cell;
use std::ptr;
#[repr(C)]
pub struct XHeyView {
    _type: RenderNode,
    displayFramebuffer: Cell<GLuint>,
    displayRenderbuffer: Cell<GLuint>,
    backingSize: Cell<GLSize>,
    layer: ShareId<CALayer>,
    orientation: ImageOrientation
}




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


        renderQuadWithShader(program,&vec![inputTexture],scaledVertices);

        unsafe {
            glBindRenderbuffer(GL_RENDERBUFFER,self.displayRenderbuffer.get());
        }
        sharedImageProcessingContext.presentBufferForDisplay();

    }

}




impl XHeyView {
    fn new(view: &UIView) -> Self {
        let layer = view.get_layer();
        let layer = layer.share();


        XHeyView{
            _type:RenderNode::new(NodeType::View),
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


    pub fn textureUnitForIndex(index: usize) -> GLenum {
        match index {
            0 => GL_TEXTURE0,
            1 => GL_TEXTURE1,
            2 => GL_TEXTURE2,
            3 => GL_TEXTURE3,
            4 => GL_TEXTURE4,
            5 => GL_TEXTURE5,
            6 => GL_TEXTURE6,
            7 => GL_TEXTURE7,
            8 => GL_TEXTURE8,
            _ => panic!("Attempted to address too high a texture unit")
        }
    }

    pub fn renderQuadWithShader(program: &GLProgram, vertices:Option<[f32;8]>, vertexBufferObject:Option<GLuint>, inputTextures: &Vec<InputTextureProperties>) {
        sharedImageProcessingContext.makeCurrentContext();
        unsafe {

            program.bind();


            let position = program.get_attribute("position").unwrap();

            if let Some(boundVBO) = vertexBufferObject {

            }else{
                glVertexAttribPointer(position.location() as u32,2,GL_FLOAT,GL_FALSE,0,vertices.unwrap().as_ptr() as *const _);
                glEnableVertexAttribArray(position.location() as u32);

            }

            for (index, inputTextureProperty) in inputTextures.iter().enumerate() {
                let (textureCoordinateString,inputTextureString) = if index == 0 {
                    (format!("inputTextureCoordinate"),format!("inputImageTexture"))
                }else{
                    (format!("inputTextureCoordinate{}",index),format!("inputImageTexture{}",index))
                };



                let inputTexture = program.get_uniform(&inputTextureString);



                if let Some(textureCoordinate) = program.get_attribute(&textureCoordinateString){

                    match inputTextureProperty.textureStorage {
                        InputTextureStorageFormat::textureCoordinate(texCoord) => {
                            glVertexAttribPointer(textureCoordinate.location() as u32,2,GL_FLOAT,GL_FALSE,0,texCoord.as_ptr() as *const _);
                            glEnableVertexAttribArray(textureCoordinate.location() as u32);
                        },
                        InputTextureStorageFormat::textureVBO(texVBO) => {
                            glBindBuffer(GL_ARRAY_BUFFER,texVBO);
                            glVertexAttribPointer(textureCoordinate.location() as u32, 2, GL_FLOAT,0,0,ptr::null());
                            glEnableVertexAttribArray(textureCoordinate.location() as u32);
                        }
                    }

                };





                glActiveTexture(Self::textureUnitForIndex(index));
                glBindTexture(GL_TEXTURE_2D,inputTextures[index].texture);
                glUniform1i(index as i32,inputTexture.location() as i32);

            }




            glDrawArrays(GL_TRIANGLE_STRIP,0,4);
        }
    }


}


#[no_mangle]
pub extern "C" fn xhey_init_view(source: *const UIView) -> *mut XHeyView{
    let _source = unsafe{source.as_ref().unwrap()};
    let view = XHeyView::new(_source);
    Box::into_raw(Box::new(view))

}