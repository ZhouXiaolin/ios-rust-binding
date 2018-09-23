use std::cell::{RefCell,Cell};
use gles_rust_binding::*;

use super::*;

#[repr(C)]
pub struct XHeyBasicFilter{
    _shader : GLProgram,
    _maximumInputs : u32,
    _inputFramebuffers:RefCell<Vec<Framebuffer>>,
    _renderFramebuffer: RefCell<Framebuffer>,
    head_node: Cell<u32>,
    tail: RefCell<Vec<u32>>,

}

impl Renderable for XHeyBasicFilter {

}

impl XHeyBasicFilter {
    pub fn new_shader(vertex:&str,fragment:&str, numberOfInputs: u32) -> Self {
        sharedImageProcessingContext.makeCurrentContext();
        let shader = GLProgram::new(vertex,fragment);
        XHeyBasicFilter{
            _maximumInputs:numberOfInputs,
            _shader: shader,
            _inputFramebuffers:RefCell::default(),
            _renderFramebuffer: RefCell::default(),
            head_node:Cell::default(),
            tail:RefCell::default()
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
            _maximumInputs:1,
            _shader: shader,
            _inputFramebuffers: RefCell::default(),
            _renderFramebuffer: RefCell::default(),
            head_node:Cell::default(),
            tail:RefCell::default()
        }
    }


    pub fn renderFrame(&self, inputFramebuffers:&Vec<Framebuffer>) -> Framebuffer {


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

    fn getTexId(&self) -> u32 {
        self._renderFramebuffer.borrow().texture
    }

    fn sizeOfInitialStageBasedOnFramebuffer(&self, inputFramebuffer: &Framebuffer) -> GLSize {
        inputFramebuffer.sizeForTargetOrientation(ImageOrientation::portrait)
    }
}



impl Edge for XHeyBasicFilter {
    type Item = Framebuffer;
    fn add_head_node(&self, edge: u32){
        self.head_node.set(edge);
    }

    /// 将ni加入这个节点的输入序列
    fn add_tail(&self, node: u32){
        self.tail.borrow_mut().push(node);
    }

    /// 返回输入序列
    fn tail_nodes(&self) -> Vec<u32>{

        let inputs = self.tail.borrow();
        let mut outputs = Vec::new();
        for input in inputs.iter() {
            outputs.push(input.clone());
        }
        outputs
    }

    /// 节点在图中的序号
    fn head_node(&self) -> u32{
        self.head_node.get()
    }

    /// 指定输入最大个数
    fn arity(&self) -> u32{
        self._maximumInputs
    }

    /// 前向计算 根据xs渲染到FBO FBO可以复用，图构造后，根据拓扑序可以计算需要的最大Framebuffer个数，并提前准备
    /// 所有关系都由Graph来控制 Framebuffer
    fn forward(&self, inputFramebuffers: &Vec<Self::Item>) -> Self::Item{

        let renderFramebuffer= self.renderFrame(inputFramebuffers);
        renderFramebuffer
    }


}
