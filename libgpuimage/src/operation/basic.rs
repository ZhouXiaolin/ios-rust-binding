use std::cell::{RefCell,Cell};
use gles_rust_binding::*;
use std::rc::Rc;
use super::*;

#[repr(C)]
#[derive(Debug)]
pub struct XHeyBasicFilter{
    shader : GLProgram,
    maximumInputs : u32,
    inputFramebuffers:RefCell<Vec<Framebuffer>>,
    head_node: Cell<u32>,
    tail: RefCell<Vec<u32>>,
    uniformSettings:ShaderUniformSettings,
    value:Cell<f32>


}

impl XHeyBasicFilter {
    pub fn new_shader(vertex:&str,fragment:&str, numberOfInputs: u32) -> Self {
        sharedImageProcessingContext.makeCurrentContext();
        let shader = GLProgram::new(vertex,fragment);
        XHeyBasicFilter{
            maximumInputs:numberOfInputs,
            shader,
            inputFramebuffers:RefCell::default(),
            head_node:Cell::default(),
            tail:RefCell::default(),
            uniformSettings:ShaderUniformSettings::default(),
            value:Cell::from(0.0)

        }
    }
    pub fn new() -> Self {
        sharedImageProcessingContext.makeCurrentContext();
        let vertexString = r#"
 attribute vec4 position;
 attribute vec2 inputTextureCoordinate;

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
     gl_FragColor = color;
 }
    "#;
        let shader = GLProgram::new(vertexString,fragmentString);

        XHeyBasicFilter{
            maximumInputs:1,
            shader,
            inputFramebuffers: RefCell::default(),
            head_node:Cell::default(),
            tail:RefCell::default(),
            uniformSettings:ShaderUniformSettings::default(),
            value:Cell::from(0.0)

        }
    }


    fn sizeOfInitialStageBasedOnFramebuffer(&self, inputFramebuffer: &Framebuffer) -> GLSize {
        inputFramebuffer.sizeForTargetOrientation(ImageOrientation::portrait)
    }
}



impl Edge for XHeyBasicFilter {
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
        Some(self.render(inputFramebuffers))
    }

    fn name(&self) -> &str {
        "basic filter"
    }

}


impl Renderable for XHeyBasicFilter {
    type Item = Rc<Framebuffer>;
    fn render(&self, inputFramebuffers:&Vec<Self::Item>) -> Self::Item {
        sharedImageProcessingContext.makeCurrentContext();


        let inputFramebuffer = inputFramebuffers.first().unwrap();

        let size = self.sizeOfInitialStageBasedOnFramebuffer(inputFramebuffer);

        let renderFramebuffer = sharedImageProcessingContext.framebufferCache.requestFramebufferWithDefault(ImageOrientation::portrait,size,false);
        println!("solaren current renderFramebuffer texture {}",renderFramebuffer.texture);

        let textureProperties = {
            let mut inputTextureProperties = vec![];
            for (index, inputFramebuffer) in inputFramebuffers.iter().enumerate() {
                println!("solaren current inputFramebuffer : {}  texture_id {}",inputFramebuffer.framebuffer, inputFramebuffer.texture);
                inputTextureProperties.push(inputFramebuffer.texturePropertiesForTargetOrientation(ImageOrientation::portrait));
            }
            inputTextureProperties
        };

        renderFramebuffer.activateFramebufferForRendering();

        let v = self.value.get();
        if v > 1.0 {
            self.value.set(0.0);
        }else{
            self.value.set(v+0.1);
        }

        let v = self.value.get();


        clearFramebufferWithColor(Color::new(v,1.0,0.0,1.0));

        let vertex = InputTextureStorageFormat::textureVBO(sharedImageProcessingContext.standardImageVBO);

        renderQuadWithShader(&self.shader,&self.uniformSettings,&textureProperties,vertex);

        renderFramebuffer
    }
}
