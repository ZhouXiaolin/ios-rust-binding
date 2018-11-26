use gles_rust_binding::*;
use super::*;
use std::cell::{RefCell,Cell};
use std::rc::Rc;
use std::sync::Arc;
#[repr(C)]
#[derive(Debug)]
pub struct XHeyLookupFilter<'a>{
    shader : GLProgram,
    maximumInputs : u32,
    inputFramebuffers:RefCell<Vec<Framebuffer>>,
    head_node: Cell<u32>,
    tail: RefCell<Vec<u32>>,
    uniformSettings:ShaderUniformSettings,
    resultId: Cell<u32>,
    context: &'a GlContext
}


impl<'a> XHeyLookupFilter<'a> {

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

 uniform highp float intensity;

 void main()
 {
     highp vec4 textureColor = texture2D(inputImageTexture, textureCoordinate);

     highp float blueColor = textureColor.b * 63.0;

     highp vec2 quad1;
     quad1.y = floor(floor(blueColor) / 8.0);
     quad1.x = floor(blueColor) - (quad1.y * 8.0);

     highp vec2 quad2;
     quad2.y = floor(ceil(blueColor) / 8.0);
     quad2.x = ceil(blueColor) - (quad2.y * 8.0);

     highp vec2 texPos1;
     texPos1.x = (quad1.x * 0.125) + 0.5/512.0 + ((0.125 - 1.0/512.0) * textureColor.r);
     texPos1.y = (quad1.y * 0.125) + 0.5/512.0 + ((0.125 - 1.0/512.0) * textureColor.g);

     highp vec2 texPos2;
     texPos2.x = (quad2.x * 0.125) + 0.5/512.0 + ((0.125 - 1.0/512.0) * textureColor.r);
     texPos2.y = (quad2.y * 0.125) + 0.5/512.0 + ((0.125 - 1.0/512.0) * textureColor.g);

     lowp vec4 newColor1 = texture2D(inputImageTexture2, texPos1);
     lowp vec4 newColor2 = texture2D(inputImageTexture2, texPos2);

     lowp vec4 newColor = mix(newColor1, newColor2, fract(blueColor));
     gl_FragColor = mix(textureColor, vec4(newColor.rgb, textureColor.w), intensity);
 }
    "#;
        let shader = GLProgram::new(vertexString,fragmentString);
        let mut uniformSettings = ShaderUniformSettings::default();
        uniformSettings.setValue("intensity",Uniform::Float(1.0));


        Self{
            maximumInputs:2,
            shader,
            inputFramebuffers: RefCell::default(),
            head_node:Cell::default(),
            tail:RefCell::default(),
            uniformSettings,
            resultId: Cell::from(0),
            context

        }
    }


    fn sizeOfInitialStageBasedOnFramebuffer(&self, inputFramebuffer: &Framebuffer) -> GLSize {
        inputFramebuffer.sizeForTargetOrientation(ImageOrientation::portrait)
    }

    pub fn set_intensity(&mut self, v : f32){
        self.uniformSettings.setValue("intensity",Uniform::Float(v));
    }

    pub fn textureId(&self) -> GLuint {
        self.resultId.get()
    }

}



impl<'a> Edge for XHeyLookupFilter<'a> {
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
        "lookup"
    }



}



impl<'a> Renderable for XHeyLookupFilter<'a> {
    type Item = Arc<Framebuffer>;
    fn render(&self, inputFramebuffers:&Vec<Self::Item>) -> Self::Item {


        let inputFramebuffer = inputFramebuffers.first().unwrap();

        let size = self.sizeOfInitialStageBasedOnFramebuffer(inputFramebuffer);

        let renderFramebuffer = self.context.framebufferCache.requestFramebufferWithDefault(ImageOrientation::portrait,size,false);
        let textureProperties = {
            let mut inputTextureProperties = vec![];
            for (index, inputFramebuffer) in inputFramebuffers.iter().enumerate() {
                inputTextureProperties.push(inputFramebuffer.texturePropertiesForTargetOrientation(ImageOrientation::portrait));
            }
            inputTextureProperties
        };

        renderFramebuffer.bindFramebufferForRendering();

        clearFramebufferWithColor(Color::green());

        let standardImageVertices:[f32;8] = [-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0];
        let vertex = InputTextureStorageFormat::textureCoordinate(standardImageVertices);

        let pso = RenderPipelineState{
            program:&self.shader
        };
        renderQuadWithShader(pso,&self.uniformSettings,&textureProperties,vertex);


        renderFramebuffer.unbindFramebufferForRendering();

        self.resultId.set(renderFramebuffer.texture);

        unsafe {
            let error = glGetError();
            if error != GL_NO_ERROR {
                info!("lookup ------------> {}",error);
            }
        }

        renderFramebuffer
    }
}