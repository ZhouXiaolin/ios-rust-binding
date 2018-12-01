use gles_rust_binding::*;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::{RefCell, Cell};
use super::*;


#[repr(C)]
#[derive(Debug)]
pub struct XheyCamera<'a>{
    head_node: Cell<u32>,
    tail: RefCell<Vec<u32>>,
    shader: GLProgram,
    luminance: GLuint,
    chrominance: GLuint,
    size: GLSize,
    uniformSettings: ShaderUniformSettings,
    orientation: ImageOrientation,
    resultId: Cell<u32>,
    context: &'a GlContext
}

impl<'a> XheyCamera<'a>{
    pub fn new(context:&'a GlContext, width: i32, height: i32, orient: i32) -> Self {
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
varying highp vec2 textureCoordinate;

 uniform sampler2D inputImageTexture;
 uniform sampler2D inputImageTexture2;
 uniform mediump mat3 colorConversionMatrix;

 void main()
 {
     mediump vec3 yuv;
     lowp vec3 rgb;

     yuv.x = texture2D(inputImageTexture, textureCoordinate).r;
     yuv.yz = texture2D(inputImageTexture2, textureCoordinate).ra - vec2(0.5, 0.5);
     rgb = colorConversionMatrix * yuv;

     gl_FragColor = vec4(rgb, 1);
 }
        "#;

        let shader = GLProgram::new(vertexString, fragmentString);
        let size = GLSize::new(width, height);

        XheyCamera {
            head_node: Cell::default(),
            tail: RefCell::default(),
            shader,
            luminance:0,
            chrominance:0,
            size,
            uniformSettings:ShaderUniformSettings::default(),
            orientation:ImageOrientation::from(orient),
            resultId:Cell::from(0),
            context
        }
    }


    pub fn update_luminance(&mut self, luminance: GLuint) {
        self.luminance = luminance;
    }

    pub fn update_chrominance(&mut self, chrominance: GLuint) {
        self.chrominance = chrominance;
    }

    pub fn textureId(&self) -> GLuint {
        self.resultId.get()
    }


    pub fn updateMatrix(&mut self, matrix: Matrix3x3){
        self.uniformSettings.setValue("colorConversionMatrix", Uniform::Matrix3x3(matrix));
    }
}

impl<'a> Edge for XheyCamera<'a> {
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
        0
    }

    /// 前向计算
    fn forward(&self, xs: &Vec<Self::Item>) -> Option<Self::Item>{


        let size = self.size;

        let storage_1 = InputTextureStorageFormat::textureCoordinate(Rotation::noRotation.textureCoordinates());
        let storage_2 = InputTextureStorageFormat::textureCoordinate(Rotation::noRotation.textureCoordinates());

        let textureProperties = vec![
            InputTextureProperties::new(storage_1,self.chrominance),
            InputTextureProperties::new(storage_2,self.luminance)
        ];

        let renderFramebuffer = self.context.framebufferCache.requestFramebufferWithDefault(self.orientation, size,false);


        self.resultId.set(renderFramebuffer.texture);


        let pso = RenderPipelineState{
            framebuffer: renderFramebuffer,
            color: Color::black()
        };


        let renderFramebuffer = pso.run(||{
            let standardImageVertices:[f32;8] = [-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0];

            let vertex = InputTextureStorageFormat::textureCoordinate(standardImageVertices);

            renderQuadWithShader(&self.shader,&self.uniformSettings,&textureProperties,vertex);
        });


        Some(renderFramebuffer)



    }

    fn name(&self) -> &str {
        "camera input"
    }
}