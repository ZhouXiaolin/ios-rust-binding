use super::gles_rust_binding::*;
use super::std::rc::Rc;
use super::std::cell::{RefCell, Cell};
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

}

impl Drop for XheyOESTexture {
    fn drop(&mut self){
        println!("Drop XheyPicture");
    }
}

impl XheyOESTexture {

    pub fn new(width: i32, height: i32) -> Self {

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
            uniformSettings:ShaderUniformSettings::default()
        }
    }

    pub fn update(&mut self, textureId: GLuint){
        self.textureId = textureId;
    }
}


impl Edge for XheyOESTexture{
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
        0
    }

    /// 前向计算
    fn forward(&self, xs: &Vec<Self::Item>) -> Option<Self::Item>{
        Some(self.render(xs))

    }

    fn name(&self) -> &str {
        "oes texture input"
    }
}


impl Renderable for XheyOESTexture {
    type Item = Rc<Framebuffer>;
    fn render(&self, inputFramebuffers:&Vec<Self::Item>) -> Self::Item {

        let size = self.size;
        let storage = InputTextureStorageFormat::textureVBO(sharedImageProcessingContext.textureVBO(Rotation::rotateClockwise));

        let textureProperties = vec![InputTextureProperties::new(storage,self.textureId)];
        let renderFramebuffer = sharedImageProcessingContext.framebufferCache.requestFramebufferWithDefault(ImageOrientation::portrait,size,false);
        renderFramebuffer.activateFramebufferForRendering();

        clearFramebufferWithColor(Color::red());

        let vertex = InputTextureStorageFormat::textureVBO(sharedImageProcessingContext.standardImageVBO);

        renderQuadWithShader(&self.shader,&self.uniformSettings,&textureProperties,vertex);

        renderFramebuffer

    }
}