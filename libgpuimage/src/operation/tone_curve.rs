use gles_rust_binding::*;
use super::*;
use std::cell::{RefCell,Cell};
use std::rc::Rc;
use std::collections::HashMap;
use std::sync::Arc;
#[repr(C)]
#[derive(Debug)]
pub struct XheyToneCurveFilter<'a>{
    shader: GLProgram,
    maximumInputs : u32,
    inputFramebuffers: RefCell<Vec<Framebuffer>>,
    head_node: Cell<u32>,
    tail: RefCell<Vec<u32>>,
    uniformSettings: ShaderUniformSettings,
    context: &'a GlContext
}

impl<'a> XheyToneCurveFilter<'a>{
    pub fn new(context: &'a GlContext) -> Self {
        let vertexString = r#"
attribute vec4 position;
 attribute vec2 inputTextureCoordinate;
 attribute vec2 inputTextureCoordinate2;
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
varying highp vec2 textureCoordinate;
varying highp vec2 textureCoordinate2;

 uniform sampler2D inputImageTexture;
 uniform sampler2D inputImageTexture2;
 uniform highp float saturation;

 const highp vec3 luminanceWeighting = vec3(0.2125, 0.7154, 0.0721);

 void main()
 {
     highp vec4 textureColor = texture2D(inputImageTexture, textureCoordinate);
     highp float redCurveValue = texture2D(inputImageTexture2, vec2(textureColor.r, 0.0)).r;
     highp float greenCurveValue = texture2D(inputImageTexture2, vec2(textureColor.g, 0.0)).g;
     highp float blueCurveValue = texture2D(inputImageTexture2, vec2(textureColor.b, 0.0)).b;

     highp vec4 color = vec4(redCurveValue, greenCurveValue, blueCurveValue, textureColor.a);


    gl_FragColor = vec4(color.rgb, 1.0);

 }
        "#;


        let shader = GLProgram::new(vertexString,fragmentString);

        let mut uniformSettings = ShaderUniformSettings::default();
        uniformSettings.setValue("saturation",Uniform::Float(1.0));

        Self{
            maximumInputs:2,
            shader,
            inputFramebuffers: RefCell::default(),
            head_node: Cell::default(),
            tail: RefCell::default(),
            uniformSettings,
            context
        }

    }



    fn sizeOfInitialStageBasedOnFramebuffer(&self, inputFramebuffer: &Framebuffer) -> GLSize {
        inputFramebuffer.sizeForTargetOrientation(ImageOrientation::portrait)
    }

}


impl<'a> Edge for XheyToneCurveFilter<'a> {
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
        "tone curve"
    }
}

impl<'a> Renderable for XheyToneCurveFilter<'a> {
    type Item = Rc<Framebuffer>;
    fn render(&self, inputFramebuffers:&Vec<Self::Item>) -> Self::Item {

        let inputFramebuffer: &Framebuffer = inputFramebuffers.first().unwrap();

        let size = self.sizeOfInitialStageBasedOnFramebuffer(inputFramebuffer);

        let renderFramebuffer = self.context.framebufferCache.requestFramebufferWithDefault(ImageOrientation::portrait,size,false);
        let textureProperties = {
            let mut inputTextureProperties = vec![];
            for (index, inputFramebuffer) in inputFramebuffers.iter().enumerate() {
                let inputFramebuffer: &Framebuffer = inputFramebuffer;

                inputTextureProperties.push(inputFramebuffer.texturePropertiesForTargetOrientation(ImageOrientation::portrait));
            }
            inputTextureProperties
        };

        let pso = RenderPipelineState{
            framebuffer:renderFramebuffer.clone(),
            color:Color::black()
        };

        pso.run(|| {

            let standardImageVertices: [f32; 8] = [-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0];

            let vertex = InputTextureStorageFormat::textureCoordinate(standardImageVertices);

            renderQuadWithShader(&self.shader, &self.uniformSettings, &textureProperties, vertex);
        })


    }
}