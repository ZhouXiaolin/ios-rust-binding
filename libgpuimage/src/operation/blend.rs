use gles_rust_binding::*;
use std::rc::Rc;
use std::sync::Arc;
use super::*;
use std::cell::{RefCell,Cell};


#[derive(Debug)]
pub struct WaterMark {
    pub textureId : u32,
    pub rect: Rect,
    pub rotation: Rotation
}

#[repr(C)]
#[derive(Debug)]
pub struct XHeyBlendFilter<'a>{
    shader : GLProgram,
    maximumInputs : u32,
    inputFramebuffers:RefCell<Vec<Framebuffer>>,
    head_node: Cell<u32>,
    tail: RefCell<Vec<u32>>,
    uniformSettings:ShaderUniformSettings,
    overriddenOutputSize: Option<Size>,
    overriddenOutputRotation: Option<Rotation>,
    watermarks: Vec<WaterMark>,
    context: &'a GlContext,
    resultId: Cell<u32>,

}

impl<'a> XHeyBlendFilter<'a> {
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

        let shader = GLProgram::new(vertexString,fragmentString);

        XHeyBlendFilter{
            maximumInputs:1,
            shader,
            inputFramebuffers:RefCell::default(),
            head_node:Cell::default(),
            tail:RefCell::default(),
            uniformSettings:ShaderUniformSettings::default(),
            overriddenOutputSize: None,
            overriddenOutputRotation: None,
            watermarks: Vec::default(),
            context,
            resultId: Cell::from(0)
        }
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

    pub fn appendWaterMark(&mut self, texId: u32, xoffset: f32, yoffset: f32, width: f32, height: f32, rotation: i32){
        let watermark = WaterMark{
            textureId:texId,
            rect:Rect::new(xoffset,yoffset,width,height),
            rotation: Rotation::from(rotation)
        };
        self.watermarks.push(watermark);

    }

    pub fn textureId(&self) -> GLuint {
        self.resultId.get()
    }
}



impl<'a> Edge for XHeyBlendFilter<'a> {
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


impl<'a> Renderable for XHeyBlendFilter<'a> {
    type Item = Arc<Framebuffer>;
    fn render(&self, inputFramebuffers:&Vec<Self::Item>) -> Self::Item {


        // 读取输入fbo
        let inputFramebuffer: &Framebuffer = inputFramebuffers.first().unwrap();

        // 计算当前fbo的大小
        let size = self.sizeOfInitialStageBasedOnFramebuffer(inputFramebuffer);

        // 生成FBO
        let renderFramebuffer = self.context.framebufferCache.requestFramebufferWithDefault(ImageOrientation::portrait,size,false);

        // 激活FBO
        renderFramebuffer.bindFramebufferForRendering();

        // 开启blend
        enableBlending(GL_ONE,GL_ONE_MINUS_SRC_ALPHA);

        // 清零
        clearFramebufferWithColor(Color::black());

        // 首先渲染传入的framebuffer

        let textureProperties = vec![inputFramebuffer.texturePropertiesForTargetOrientation(ImageOrientation::portrait)];
        self.resultId.set(renderFramebuffer.texture);

        let standardImageVertices:[f32;8] = [-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0];
        let vertex = InputTextureStorageFormat::textureCoordinate(standardImageVertices);

        renderQuadWithShader(&self.shader,&self.uniformSettings,&textureProperties,vertex);

        // 然后遍历渲染watermark

        for watermark in self.watermarks.iter() {

            let storage = InputTextureStorageFormat::textureCoordinate(watermark.rotation.textureCoordinates());
            let textureProperties = vec![InputTextureProperties::new(storage,watermark.textureId)];


            let x = watermark.rect.position.x;
            let y = watermark.rect.position.y;
            let w = watermark.rect.size.width;
            let h = watermark.rect.size.height;



            let standardImageVertices:[f32;8] = [x,   y,
                                                 x+w, y,
                                                 x,   y+h,
                                                 x+w, y+h];

            let vertex = InputTextureStorageFormat::textureCoordinate(standardImageVertices);



            renderQuadWithShader(&self.shader,&self.uniformSettings,&textureProperties,vertex);

        }
        // 关闭blend
        disableBlending();

        renderFramebuffer.unbindFramebufferForRendering();
        renderFramebuffer
    }
}
