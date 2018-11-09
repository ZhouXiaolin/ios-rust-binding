use gles_rust_binding::*;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::{RefCell, Cell};
use super::*;

// 这个类需要update一个外部纹理，并使用oes扩展，单独绘制

#[repr(C)]
#[derive(Debug)]
pub struct XheyOESTexture{
    head_node: Cell<u32>,
    tail: RefCell<Vec<u32>>,
    shader: GLProgram,
    textureId:GLuint,
    size: GLSize,
    uniformSettings:ShaderUniformSettings,
    orientation: ImageOrientation,
    resultId: Cell<u32>

}

impl Drop for XheyOESTexture {
    fn drop(&mut self){
        info!("Drop XheyPicture");
    }
}

impl XheyOESTexture {

    pub fn new(width: i32, height: i32, orient: i32) -> Self {


        let vertexString = r#"
 attribute vec4 position;
 attribute vec4 inputTextureCoordinate;

 varying vec2 textureCoordinate;

 uniform mat4 uTexMatrix;
 void main()
 {
     gl_Position = position;
     textureCoordinate = (uTexMatrix * inputTextureCoordinate).xy;
 }
        "#;
        let fragmentString = r#"
 #extension GL_OES_EGL_image_external : require
 precision mediump float;
 varying highp vec2 textureCoordinate;
 uniform samplerExternalOES inputImageTexture;

 void main(){
    vec4 color = texture2D(inputImageTexture, textureCoordinate);
    gl_FragColor = color;
 }

        "#;

        let shader = GLProgram::new(vertexString,fragmentString);

        let size = GLSize::new(width,height);

        XheyOESTexture{
            head_node:Cell::default(),
            tail: RefCell::default(),
            shader,
            textureId:0,
            size,
            uniformSettings:ShaderUniformSettings::default(),
            orientation: ImageOrientation::from(orient),
            resultId:Cell::from(0)
        }
    }

    pub fn update(&mut self, textureId: GLuint){
        self.textureId = textureId;
    }
    pub fn updateMatrix(&mut self, matrix: Matrix4x4){
        self.uniformSettings.setValue("uTexMatrix", Uniform::Matrix4x4(matrix));
    }

    pub fn textureId(&self) -> GLuint {
        self.resultId.get()
    }


}


impl Edge for XheyOESTexture{
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

        let storage = InputTextureStorageFormat::textureCoordinate(Rotation::noRotation.textureCoordinates());

        let textureProperties = vec![InputTextureProperties::new(storage,self.textureId)];

        let renderFramebuffer = sharedImageProcessingContext.framebufferCache.requestFramebufferWithDefault(self.orientation, size,false);

        renderFramebuffer.bindFramebufferForRendering();

        clearFramebufferWithColor(Color::red());

        let standardImageVertices:[f32;8] = [-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0];

        let vertex = InputTextureStorageFormat::textureCoordinate(standardImageVertices);

        renderQuadWithShader(&self.shader,&self.uniformSettings,&textureProperties,vertex);

        renderFramebuffer.unbindFramebufferForRendering();

        self.resultId.set(renderFramebuffer.texture);

        unsafe {
            let error = glGetError();
            if error != GL_NO_ERROR {
                info!("texture ------------> {}",error);
            }
        }


        Some(renderFramebuffer)



    }

    fn name(&self) -> &str {
        "oes texture input"
    }
}


