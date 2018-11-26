use gles_rust_binding::*;
use super::*;
use std::cell::{RefCell,Cell};
use std::rc::Rc;
use std::sync::Arc;

#[repr(C)]
#[derive(Debug)]
pub struct XHeyUnsharpMaskFilter<'a>{
    shader : GLProgram,
    maximumInputs : u32,
    head_node: Cell<u32>,
    tail: RefCell<Vec<u32>>,
    uniformSettings:ShaderUniformSettings,
    context:&'a GlContext

}


impl<'a> XHeyUnsharpMaskFilter<'a> {

    pub fn new(context: &'a GlContext) -> Self {

        let vertexString = r#"
attribute vec4 position;
attribute vec4 inputTextureCoordinate;

uniform float texelWidth;
uniform float texelHeight;

varying vec2 blurCoordinates[9];
const highp float radius = 1.0;

void main(){
    vec2 singleStepOffset = vec2(texelWidth, texelHeight);
    blurCoordinates[0] = inputTextureCoordinate.xy;
    blurCoordinates[1] = inputTextureCoordinate.xy + singleStepOffset * vec2(1.0, 0.0) * radius;
    blurCoordinates[2] = inputTextureCoordinate.xy + singleStepOffset * vec2(-1.0, 0.0) * radius;
    blurCoordinates[3] = inputTextureCoordinate.xy + singleStepOffset * vec2(0.0, 1.0) * radius;
    blurCoordinates[4] = inputTextureCoordinate.xy + singleStepOffset * vec2(0.0, -1.0) * radius;
    blurCoordinates[5] = inputTextureCoordinate.xy + singleStepOffset * vec2(1.0, 1.0) * radius;
    blurCoordinates[6] = inputTextureCoordinate.xy + singleStepOffset * vec2(1.0, -1.0) * radius;
    blurCoordinates[7] = inputTextureCoordinate.xy + singleStepOffset * vec2(-1.0, 1.0) * radius;
    blurCoordinates[8] = inputTextureCoordinate.xy + singleStepOffset * vec2(-1.0, -1.0) * radius;

    gl_Position = position;
}
    "#;

        let fragmentString = r#"
 uniform sampler2D inputImageTexture;
uniform sampler2D inputImageTexture2;

varying highp vec2 blurCoordinates[9];
uniform highp float intensity;
uniform highp float saturation;
const highp vec3 luminanceWeighting = vec3(0.2125, 0.7154, 0.0721);
void main()
{
    highp vec4 sum = vec4(0.0);
    sum += texture2D(inputImageTexture2, blurCoordinates[0]);
    sum += texture2D(inputImageTexture2, blurCoordinates[1]);
    sum += texture2D(inputImageTexture2, blurCoordinates[2]);
    sum += texture2D(inputImageTexture2, blurCoordinates[3]);
    sum += texture2D(inputImageTexture2, blurCoordinates[4]);
    sum += texture2D(inputImageTexture2, blurCoordinates[5]);
    sum += texture2D(inputImageTexture2, blurCoordinates[6]);
    sum += texture2D(inputImageTexture2, blurCoordinates[7]);
    sum += texture2D(inputImageTexture2, blurCoordinates[8]);


    highp vec3 blurredImageColor = sum.rgb / 9.0;
    highp vec4 sharpImageColor = texture2D(inputImageTexture2, blurCoordinates[0]);


    highp vec3 highPass = sharpImageColor.rgb - blurredImageColor;

    highp float luminance = dot(sharpImageColor.rgb, luminanceWeighting);
    highp vec3 greyScaleColor = vec3(luminance);

    luminance = luminance<0.5 ?  2.0*luminance*luminance : 1.0-2.0*(1.0-luminance)*(1.0-luminance);
    highp vec4 currentColor = texture2D(inputImageTexture, blurCoordinates[0]);
    highp vec3 color = currentColor.rgb + highPass * intensity;

    gl_FragColor = vec4(mix(greyScaleColor, color, saturation), 1.0);

}
    "#;
        let shader = GLProgram::new(vertexString,fragmentString);

        XHeyUnsharpMaskFilter{
            maximumInputs:2,
            shader,
            head_node:Cell::default(),
            tail:RefCell::default(),
            uniformSettings:ShaderUniformSettings::default(),
            context

        }
    }


    fn sizeOfInitialStageBasedOnFramebuffer(&self, inputFramebuffer: &Framebuffer) -> GLSize {
        inputFramebuffer.sizeForTargetOrientation(ImageOrientation::portrait)
    }

    pub fn set_value(&mut self, v : f32){
        self.uniformSettings.setValue("value",Uniform::Float(v));
    }
}



impl<'a> Edge for XHeyUnsharpMaskFilter<'a> {
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
        "combine"
    }

}



impl<'a> Renderable for XHeyUnsharpMaskFilter<'a> {
    type Item = Arc<Framebuffer>;
    fn render(&self, inputFramebuffers:&Vec<Self::Item>) -> Self::Item {

        let inputFramebuffer : &Framebuffer = inputFramebuffers.first().unwrap();

        let size = self.sizeOfInitialStageBasedOnFramebuffer(inputFramebuffer);

        let renderFramebuffer = self.context.framebufferCache.requestFramebufferWithDefault(ImageOrientation::portrait,size,false);
        let textureProperties = {
            let mut inputTextureProperties = vec![];
            for (index, inputFramebuffer) in inputFramebuffers.iter().enumerate() {
                inputTextureProperties.push(inputFramebuffer.texturePropertiesForTargetOrientation(ImageOrientation::portrait));
            }
            inputTextureProperties
        };

        let outputRotation = inputFramebuffer.orientation.get().rotationNeededForOrientation(ImageOrientation::portrait);
        let texelSize = inputFramebuffer.texelSize(outputRotation);

        let mut uniformSettings = ShaderUniformSettings::default();
        uniformSettings.setValue("texelWidth",Uniform::Float(texelSize.width));
        uniformSettings.setValue("texelHeight",Uniform::Float(texelSize.height));
        uniformSettings.setValue("intensity",Uniform::Float(1.3));
        uniformSettings.setValue("saturation",Uniform::Float(1.1));


        renderFramebuffer.bindFramebufferForRendering();

        clearFramebufferWithColor(Color::red());

        let standardImageVertices:[f32;8] = [-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0];
        let vertex = InputTextureStorageFormat::textureCoordinate(standardImageVertices);

        let pso = RenderPipelineState{
            program:&self.shader
        };

        renderQuadWithShader(pso,&uniformSettings,&textureProperties,vertex);


        renderFramebuffer.unbindFramebufferForRendering();

        renderFramebuffer
    }
}