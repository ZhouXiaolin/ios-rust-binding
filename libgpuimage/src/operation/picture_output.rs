// Android Drawable

use gles_rust_binding::*;
use std::rc::Rc;
use std::cell::{RefCell, Cell};
use super::*;

#[repr(C)]
#[derive(Debug)]
pub struct XheyPictureOutput{
    head_node: Cell<u32>,
    tail: RefCell<Vec<u32>>,
    uniformSettings: ShaderUniformSettings,
    rotation: Rotation,
    backingSize:GLSize,
    textureId: Cell<GLuint>

}

impl Drop for XheyPictureOutput {
    fn drop(&mut self){
        info!("Drop XheyPictureOutput");
    }
}
impl XheyPictureOutput {
    pub fn new(width: i32, height: i32, orient: i32) -> Self {

        XheyPictureOutput{
            head_node:Cell::default(),
            tail:RefCell::default(),
            uniformSettings:ShaderUniformSettings::default(),
            rotation: Rotation::from(orient),
            backingSize: GLSize::new(width,height),
            textureId:Cell::default()
        }
    }


    fn sizeOfInitialStageBasedOnFramebuffer(&self, inputFramebuffer: &Framebuffer) -> GLSize {
        inputFramebuffer.sizeForTargetOrientation(ImageOrientation::portrait)
    }

    pub fn textureId(&self) -> GLuint {
        self.textureId.get()
    }
}


impl Edge for XheyPictureOutput {
    type Item = Rc<Framebuffer>;

    fn add_head_node(&self, edge: u32){
        self.head_node.set(edge);
    }

    /// 将ni加入这个节点的输入序列
    fn add_tail(&self, node: u32){
        self.tail.borrow_mut().push(node);
    }

    /// 返回输入序列 这里的实现很奇怪，应该有其他更好的办法？
    fn tail_nodes(&self) -> Vec<u32>{
        self.tail.borrow().clone()
    }

    /// 节点在图中的序号
    fn head_node(&self) -> u32{
        self.head_node.get()
    }

    /// 指定输入最大个数
    fn arity(&self) -> u32{
        1
    }

    /// 前向计算 在XheyView中实现这个Trait，应该做的是将xs的Framebuffer绘制到View上，返
    fn forward(&self, xs: &Vec<Self::Item>) -> Option<Self::Item>{
        self.render(&xs[0]);
        None
    }

    fn name(&self) -> &str {
        "surface view"
    }
}

impl Drawable for XheyPictureOutput {
    type Item = Framebuffer;
    fn render(&self, framebuffer:&Self::Item){


        let inputFramebuffer: &Framebuffer = framebuffer;

        let size = self.sizeOfInitialStageBasedOnFramebuffer(inputFramebuffer);

        let renderFramebuffer = sharedImageProcessingContext.framebufferCache.requestFramebufferWithDefault(ImageOrientation::portrait,size,false);

        self.textureId.set(renderFramebuffer.texture);

        renderFramebuffer.bindFramebufferForRendering();

        clearFramebufferWithColor(Color::red());

        let program = &sharedImageProcessingContext.passthroughShader;

        let verticallyInvertedImageVertices: [f32;8] = [-1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0];

        let scaledVertices = FillMode::preserveAspectRatio.transformVertices(verticallyInvertedImageVertices,framebuffer.sizeForTargetOrientation(ImageOrientation::portrait),self.backingSize);

        let storage = InputTextureStorageFormat::textureVBO(sharedImageProcessingContext.textureVBO(self.rotation));
        let inputTexture = InputTextureProperties::new(storage,inputFramebuffer.texture);

        let vertex = InputTextureStorageFormat::textureCoordinate(scaledVertices);

        renderQuadWithShader(program,&self.uniformSettings,&vec![inputTexture],vertex);


        info!("picture output finish");

    }
}