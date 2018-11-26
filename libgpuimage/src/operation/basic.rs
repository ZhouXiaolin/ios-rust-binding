use gles_rust_binding::*;
use std::rc::Rc;
use std::sync::Arc;
use super::*;
use std::cell::{RefCell,Cell};
#[repr(C)]
#[derive(Debug)]
pub struct XHeyBasicFilter<'a>{
    shader : GLProgram,
    maximumInputs : u32,
    inputFramebuffers:RefCell<Vec<Framebuffer>>,
    head_node: Cell<u32>,
    tail: RefCell<Vec<u32>>,
    uniformSettings:ShaderUniformSettings,
    overriddenOutputSize: Option<Size>,
    overriddenOutputRotation: Option<Rotation>,
    resultId: Cell<u32>,
    context: &'a GlContext

}

impl<'a> XHeyBasicFilter<'a> {
    pub fn new_shader(context: &'a GlContext,vertex:&str,fragment:&str, numberOfInputs: u32, ) -> Self {


        let shader = GLProgram::new(vertex,fragment);

        XHeyBasicFilter{
            maximumInputs:numberOfInputs,
            shader,
            inputFramebuffers:RefCell::default(),
            head_node:Cell::default(),
            tail:RefCell::default(),
            uniformSettings:ShaderUniformSettings::default(),
            overriddenOutputSize: None,
            overriddenOutputRotation: None,
            resultId: Cell::from(0),
            context
        }
    }

    pub fn new_shader_with_fragment(context: &'a GlContext,fragment: &str, maximumInputs: u32) -> Self {
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

        Self::new_shader(context,vertexString,fragment,maximumInputs)


    }

    pub fn new(context: &'a GlContext) -> Self {

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
        Self::new_shader(context,vertexString,fragmentString,1)
    }


    fn sizeOfInitialStageBasedOnFramebuffer(&self, inputFramebuffer: &Framebuffer) -> GLSize {
        if let Some(outputSize) = self.overriddenOutputSize {
            GLSize::new(outputSize.width as i32, outputSize.height as i32)
        }else{
            inputFramebuffer.sizeForTargetOrientation(ImageOrientation::portrait)

        }

    }

    pub fn updateOutputSize(&mut self, width: i32, height: i32) {
        self.overriddenOutputSize = Some(Size::new(width as f32, height as f32));
    }

    pub fn updateOutputRotation(&mut self, rotation: i32){
        self.overriddenOutputRotation = Some(Rotation::from(rotation));
    }

    pub fn textureId(&self) -> GLuint {
        self.resultId.get()
    }

}



impl<'a> Edge for XHeyBasicFilter<'a> {
    type Item = Arc<Framebuffer>;
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


impl<'a> Renderable for XHeyBasicFilter<'a> {
    type Item = Arc<Framebuffer>;
    fn render(&self, inputFramebuffers:&Vec<Self::Item>) -> Self::Item {


        let inputFramebuffer = inputFramebuffers.first().unwrap();

        let size = self.sizeOfInitialStageBasedOnFramebuffer(inputFramebuffer);

        let renderFramebuffer = self.context.framebufferCache.requestFramebufferWithDefault(ImageOrientation::portrait,size,false);

        let textureProperties = {
            let mut inputTextureProperties = vec![];

            if let Some(outputRotation) = self.overriddenOutputRotation {
                for (index, inputFramebuffer) in inputFramebuffers.iter().enumerate() {
                    inputTextureProperties.push(inputFramebuffer.texturePropertiesForOutputRotation(outputRotation));
                }
            }else{
                for (index, inputFramebuffer) in inputFramebuffers.iter().enumerate() {
                    inputTextureProperties.push(inputFramebuffer.texturePropertiesForTargetOrientation(ImageOrientation::portrait));
                }
            }


            inputTextureProperties
        };

        renderFramebuffer.bindFramebufferForRendering();


        clearFramebufferWithColor(Color::black());

        let standardImageVertices:[f32;8] = [-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0];
        let vertex = InputTextureStorageFormat::textureCoordinate(standardImageVertices);

        let pso = RenderPipelineState{
            program:&self.shader
        };

        renderQuadWithShader(pso,&self.uniformSettings,&textureProperties,vertex);


        renderFramebuffer.unbindFramebufferForRendering();

        self.resultId.set(renderFramebuffer.texture);


        renderFramebuffer
    }
}
