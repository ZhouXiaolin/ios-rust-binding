use ios_rust_binding::{UIView,NSUInteger,ShareId,CALayer};
use gles_rust_binding::*;
use std::cell::{Cell,RefCell};
use std::rc::Rc;
use super::*;



#[repr(C)]
#[derive(Debug)]
pub struct XHeyView {
    displayFramebuffer: Cell<GLuint>,
    displayRenderbuffer: Cell<GLuint>,
    backingSize: Cell<GLSize>,
    layer: ShareId<CALayer>,
    orientation: ImageOrientation,
    head_node: Cell<u32>,
    tail: RefCell<Vec<u32>>,
    uniformSettings:ShaderUniformSettings,
}

impl Drop for XHeyView {
    fn drop(&mut self){
        println!("Drop XHeyView");
        unsafe {
            let mut displayFramebuffer = self.displayFramebuffer.get();
            if displayFramebuffer > 0{

                glDeleteFramebuffers(1,&mut displayFramebuffer);
            }

            let mut displayRenderbuffer = self.displayRenderbuffer.get();
            if displayRenderbuffer > 0 {
                glDeleteRenderbuffers(1,&mut displayRenderbuffer);
            }
        }

    }

}

impl XHeyView {
    pub fn new(view: &UIView) -> Self {
        let layer = view.get_layer();
        let layer = layer.share();


        Self{
            displayFramebuffer:Cell::default(),
            displayRenderbuffer:Cell::default(),
            backingSize:Cell::default(),
            layer,
            orientation: ImageOrientation::portrait,
            head_node:Cell::default(),
            tail:RefCell::default(),
            uniformSettings:ShaderUniformSettings::default(),
        }
    }


    fn activateDisplayFramebuffer(&self) {
        unsafe {
            glBindFramebuffer(GL_FRAMEBUFFER,self.displayFramebuffer.get());
            glViewport(0,0,self.backingSize.get().width,self.backingSize.get().height);
        }
    }


    fn createDisplayFramebuffer(&self){
        unsafe {
            let mut frameBuffer : GLuint = 0;
            glGenFramebuffers(1,&mut frameBuffer);
            self.displayFramebuffer.set(frameBuffer);
            glBindFramebuffer(GL_FRAMEBUFFER, frameBuffer);


            let mut colorRenderBuffer : GLuint = 0;
            glGenRenderbuffers(1,&mut colorRenderBuffer);
            self.displayRenderbuffer.set(colorRenderBuffer);
            glBindRenderbuffer(GL_RENDERBUFFER,colorRenderBuffer);

            sharedImageProcessingContext.context.renderBufferStorage(GL_RENDERBUFFER as NSUInteger,&self.layer);


            let mut backingWidth : GLint = 0;
            let mut backingHeight : GLint = 0;

            glGetRenderbufferParameteriv(GL_RENDERBUFFER, GL_RENDERBUFFER_WIDTH, &mut backingWidth);
            glGetRenderbufferParameteriv(GL_RENDERBUFFER, GL_RENDERBUFFER_HEIGHT, &mut backingHeight);

            self.backingSize.set(GLSize::new(backingWidth,backingHeight));

            glFramebufferRenderbuffer(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0,GL_RENDERBUFFER, colorRenderBuffer);


        }
    }


}


impl Edge for XHeyView {
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
        "view"
    }

}



impl Drawable for XHeyView{
    type Item = Framebuffer;
    fn render(&self, framebuffer:&Self::Item){
        sharedImageProcessingContext.makeCurrentContext();

        if self.displayFramebuffer.get() == 0 {
            self.createDisplayFramebuffer()
        }


        self.activateDisplayFramebuffer();

        clearFramebufferWithColor(Color::black());

        let program = &sharedImageProcessingContext.passthroughShader;

        let verticallyInvertedImageVertices: [f32;8] = [-1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0];

        let scaledVertices = FillMode::preserveAspectRatio.transformVertices(verticallyInvertedImageVertices,framebuffer.sizeForTargetOrientation(self.orientation),self.backingSize.get());


        let inputTexture = framebuffer.texturePropertiesForTargetOrientation(self.orientation);

        let vertex = InputTextureStorageFormat::textureCoordinate(scaledVertices);

        renderQuadWithShader(program,&self.uniformSettings,&vec![inputTexture],vertex);

        unsafe {
            glBindRenderbuffer(GL_RENDERBUFFER,self.displayRenderbuffer.get());
        }

        sharedImageProcessingContext.presentBufferForDisplay();

    }

}
