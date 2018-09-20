use std::mem;
use std::cell::{RefCell,Cell};
use gles_rust_binding::*;
use super::{Consumer,Framebuffer,Source,Node,NodeType,ImageOrientation,GLSize,Color,InputTextureStorageFormat};
use super::GLRender::*;
use super::RenderNode;
use super::sharedImageProcessingContext;

#[repr(C)]
pub struct XHeyBasicFilter<'a>{
    _node : RenderNode,
    _targets: RefCell<Vec<Box<&'a dyn Consumer>>>,
    _shader : GLProgram,
    _maximumInputs : i32,
    _inputFramebuffers:RefCell<Vec<Framebuffer>>,
    _renderFramebuffer: RefCell<Framebuffer>,

}



impl<'a> XHeyBasicFilter<'a> {
    pub fn new_shader(vertex:&str,fragment:&str, numberOfInputs: i32) -> Self {
        sharedImageProcessingContext.makeCurrentContext();
        let shader = GLProgram::new(vertex,fragment);
        XHeyBasicFilter{
            _node:RenderNode::new(NodeType::BasicFilter),
            _targets:RefCell::new(Vec::new()),
            _maximumInputs:numberOfInputs,
            _shader: shader,
            _inputFramebuffers:RefCell::default(),
            _renderFramebuffer: RefCell::default(),
        }
    }
    pub fn new() -> Self {
        sharedImageProcessingContext.makeCurrentContext();
        let vertexStr = r#"
 attribute vec4 position;
 attribute vec4 inputTextureCoordinate;

 varying vec2 textureCoordinate;

 void main()
 {
     gl_Position = position;
     textureCoordinate = inputTextureCoordinate.xy;
 }
    "#;

        let fragmentStr = r#"
 precision mediump float;

 varying highp vec2 textureCoordinate;
 uniform sampler2D inputImageTexture;

 void main()
 {
     vec4 color = texture2D(inputImageTexture, textureCoordinate);
     gl_FragColor = vec4(color.r, 0.0, 0.0, 1.0);
 }
    "#;
        let shader = GLProgram::new(vertexStr,fragmentStr);

        XHeyBasicFilter{
            _node:RenderNode::new(NodeType::BasicFilter),
            _targets:RefCell::new(Vec::new()),
            _maximumInputs:1,
            _shader: shader,
            _inputFramebuffers: RefCell::default(),
            _renderFramebuffer: RefCell::default()
        }
    }


    pub fn renderFrame(&self){



    }

    fn sizeOfInitialStageBasedOnFramebuffer(&self, inputFramebuffer: &Framebuffer) -> GLSize {
        inputFramebuffer.sizeForTargetOrientation(ImageOrientation::portrait)
    }
}

impl<'a> Node for XHeyBasicFilter<'a>{
    fn get_type_name(&self) -> NodeType {
        NodeType::BasicFilter
    }
}



impl<'a> Source<'a> for XHeyBasicFilter<'a> {
    fn addTarget(&self, target: &'a dyn Consumer, _location: u32) {
        let mut targets = self._targets.borrow_mut();
        targets.push(Box::new(target));
        target.setSource(self,_location);
    }

    fn removeAllTargets(&self){

    }
    fn updateTargetsWithFramebuffer(&self, framebuffer:&Framebuffer){
        for (index,target) in self._targets.borrow_mut().iter().enumerate() {
            target.newFramebufferAvailable(framebuffer,index);
        }
    }
}

impl<'a> Consumer for XHeyBasicFilter<'a> {
    fn setSource(&self, _source: &dyn Source, _location: u32) {

    }

    fn newFramebufferAvailable(&self, framebuffer: &Framebuffer, fromSourceIndex: usize){


        let mut inputFramebuffers = self._inputFramebuffers.borrow_mut();


        inputFramebuffers.insert(fromSourceIndex,framebuffer.clone());

        let len = inputFramebuffers.len();

        if len >= self._maximumInputs as usize {



            let inputFramebuffer = inputFramebuffers.first().unwrap();

            let size = self.sizeOfInitialStageBasedOnFramebuffer(inputFramebuffer);

            let renderFramebuffer = sharedImageProcessingContext.frameubfferCache.requestFramebufferWithDefault(ImageOrientation::portrait,size,false);

            let textureProperties = {
                let mut inputTextureProperties = vec![];
                for (index, inputFramebuffer) in inputFramebuffers.iter().enumerate() {
                    inputTextureProperties.push(inputFramebuffer.texturePropertiesForTargetOrientation(ImageOrientation::portrait));
                }
                inputTextureProperties
            };

            renderFramebuffer.activateFramebufferForRendering();

            clearFramebufferWithColor(Color::black());

            let vertex = InputTextureStorageFormat::textureVBO(sharedImageProcessingContext.standardImageVBO);

            renderQuadWithShader(&self._shader,&textureProperties,vertex);

            self.updateTargetsWithFramebuffer(&renderFramebuffer);
        }

    }

}

