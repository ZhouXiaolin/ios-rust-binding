use std::mem;
use std::cell::{RefCell,Cell,RefMut};
use gles_rust_binding::*;

use super::*;
#[repr(C)]
pub struct XHeyBasicFilter<'a>{
    _targets: RefCell<Vec<Box<&'a dyn Consumer>>>,
    _shader : GLProgram,
    _maximumInputs : i32,
    _inputFramebuffers:RefCell<Vec<Framebuffer>>,
    _renderFramebuffer: RefCell<Framebuffer>,

    index:u32,
    inputs: RefCell<Vec<u32>>

}



impl<'a> XHeyBasicFilter<'a> {
    pub fn new_shader(vertex:&str,fragment:&str, numberOfInputs: i32) -> Self {
        sharedImageProcessingContext.makeCurrentContext();
        let shader = GLProgram::new(vertex,fragment);
        XHeyBasicFilter{
            _targets:RefCell::new(Vec::new()),
            _maximumInputs:numberOfInputs,
            _shader: shader,
            _inputFramebuffers:RefCell::default(),
            _renderFramebuffer: RefCell::default(),
            index:sharedContext.operation_id(),
            inputs:RefCell::default()
        }
    }
    pub fn new() -> Self {
        sharedImageProcessingContext.makeCurrentContext();
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
     gl_FragColor = vec4(color.r, 0.0, 0.0, 1.0);
 }
    "#;
        let shader = GLProgram::new(vertexString,fragmentString);

        XHeyBasicFilter{
            _targets:RefCell::new(Vec::new()),
            _maximumInputs:1,
            _shader: shader,
            _inputFramebuffers: RefCell::default(),
            _renderFramebuffer: RefCell::default(),
            index:sharedContext.operation_id(),
            inputs:RefCell::default()
        }
    }


    pub fn renderFrameWithFramebuffers(&self, inputFramebuffers:&Vec<Framebuffer>) -> Framebuffer {
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

        renderFramebuffer
    }

    pub fn renderFrame(&self,framebuffer: &Framebuffer, fromSourceIndex: usize){

        let mut inputFramebuffers = self._inputFramebuffers.borrow_mut();
        inputFramebuffers.insert(fromSourceIndex,framebuffer.clone());

        let len = inputFramebuffers.len();

        if len >= self._maximumInputs as usize {
            let renderFramebuffer= self.renderFrameWithFramebuffers(inputFramebuffers.as_mut());

            self.updateTargetsWithFramebuffer(&renderFramebuffer);
        }


    }

    fn sizeOfInitialStageBasedOnFramebuffer(&self, inputFramebuffer: &Framebuffer) -> GLSize {
        inputFramebuffer.sizeForTargetOrientation(ImageOrientation::portrait)
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

        self.renderFrame(framebuffer,fromSourceIndex);

    }

}


#[cfg(feature = "new")]
impl<'a> Operation for XHeyBasicFilter<'a> {

    /// 将ni加入这个节点的输入序列
    fn append(&self, ni: u32){
        self.inputs.borrow_mut().push(ni)
    }

    /// 返回输入序列
    fn inputs(&self) -> Vec<u32>{

        let inputs = self.inputs.borrow();
        let mut outputs = Vec::new();
        for input in inputs.iter() {
            outputs.push(input.clone());
        }
        outputs
    }

    /// 节点在图中的序号
    fn index(&self) -> u32{
        self.index
    }

    /// 指定输入最大个数
    fn arity(&self) -> u32{
        self._maximumInputs as u32
    }

    /// 前向计算 根据xs渲染到FBO FBO可以复用，图构造后，根据拓扑序可以计算需要的最大Framebuffer个数，并提前准备，
    fn forward(&self, inputFramebuffers: Vec<Framebuffer>) -> Framebuffer{

        let renderFramebuffer= self.renderFrameWithFramebuffers(&inputFramebuffers);


        renderFramebuffer
    }

    ///针对Source节点，在渲染过程中指定其Framebufer
    fn set_framebuffer(&self, value:Framebuffer){

    }

}