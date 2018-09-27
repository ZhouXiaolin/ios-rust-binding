use std::cell::{RefCell,Cell};
use gles_rust_binding::*;
use std::rc::Rc;
use super::*;

#[repr(C)]
#[derive(Debug)]
pub struct XHeyCombineFilter{
    shader : GLProgram,
    maximumInputs : u32,
    inputFramebuffers:RefCell<Vec<Framebuffer>>,
    head_node: Cell<u32>,
    tail: RefCell<Vec<u32>>,
    uniformSettings:ShaderUniformSettings

}


impl XHeyCombineFilter {

    pub fn new() -> Self {
        sharedImageProcessingContext.makeCurrentContext();
        let vertexString = r#"
 attribute vec4 position;
 attribute vec4 inputTextureCoordinate;
 attribute vec4 inputTextureCoordinate2;
 varying vec2 textureCoordinate;
 varying vec2 textureCoordinate2;

 void main()
 {
     gl_Position = position;
     textureCoordinate = inputTextureCoordinate.xy;
     textureCoordinate2 = inputTextureCoordinate2.xy;

 }
    "#;

        let fragmentString = r#"
 precision mediump float;

 varying highp vec2 textureCoordinate;
 varying highp vec2 textureCoordinate2;
 uniform sampler2D inputImageTexture;
 uniform sampler2D inputImageTexture2;

 uniform float value;
 void main()
 {
     vec4 color1 = texture2D(inputImageTexture, textureCoordinate);
     vec4 color2 = texture2D(inputImageTexture2, textureCoordinate2);

     if(textureCoordinate.x > value) {
        gl_FragColor = color1;
     }else{
        gl_FragColor = color2;
     }
 }
    "#;
        let shader = GLProgram::new(vertexString,fragmentString);

        XHeyCombineFilter{
            maximumInputs:2,
            shader: shader,
            inputFramebuffers: RefCell::default(),
            head_node:Cell::default(),
            tail:RefCell::default(),
            uniformSettings:ShaderUniformSettings::default()

        }
    }


    fn sizeOfInitialStageBasedOnFramebuffer(&self, inputFramebuffer: &Framebuffer) -> GLSize {
        inputFramebuffer.sizeForTargetOrientation(ImageOrientation::portrait)
    }

    pub fn set_value(&mut self, v : f32){
        self.uniformSettings.setValue("value",Uniform::Float(v));
    }
}



impl Edge for XHeyCombineFilter {
    type Item = Rc<Framebuffer>;
    fn add_head_node(&self, edge: u32){
        self.head_node.set(edge);
    }

    /// 将ni加入这个节点的输入序列
    fn add_tail(&self, node: u32){
        self.tail.borrow_mut().push(node);
    }

    /// 返回输入序列
    fn tail_nodes(&self) -> Vec<u32>{
        self.tail.borrow().clone()
    }

    /// 节点在图中的序号
    fn head_node(&self) -> u32{
        self.head_node.get()
    }

    /// 指定输入最大个数
    fn arity(&self) -> u32{
        self.maximumInputs
    }

    /// 前向计算 根据xs渲染到FBO FBO可以复用，图构造后，根据拓扑序可以计算需要的最大Framebuffer个数，并提前准备
    /// 所有关系都由Graph来控制 Framebuffer
    fn forward(&self, inputFramebuffers: &Vec<Self::Item>) -> Option<Self::Item>{

        let renderFramebuffer= self.render(inputFramebuffers);
        Some(renderFramebuffer)
    }

    fn name(&self) -> &str {
        "combine"
    }

}



impl Renderable for XHeyCombineFilter {
    type Item = Rc<Framebuffer>;
    fn render(&self, inputFramebuffers:&Vec<Self::Item>) -> Self::Item {

        sharedImageProcessingContext.makeCurrentContext();

        let inputFramebuffer = inputFramebuffers.first().unwrap();

        let size = self.sizeOfInitialStageBasedOnFramebuffer(inputFramebuffer);

        let renderFramebuffer = sharedImageProcessingContext.framebufferCache.requestFramebufferWithDefault(ImageOrientation::portrait,size,false);
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

        renderQuadWithShader(&self.shader,&self.uniformSettings,&textureProperties,vertex);

        renderFramebuffer
    }
}