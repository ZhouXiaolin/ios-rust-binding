use ios_rust_binding::{UIView,NSUInteger,ShareId,CALayer};

use gles_rust_binding::*;

use std::cell::{Cell,RefCell};

use super::*;



#[repr(C)]
pub struct XHeyView {
    displayFramebuffer: Cell<GLuint>,
    displayRenderbuffer: Cell<GLuint>,
    backingSize: Cell<GLSize>,
    layer: ShareId<CALayer>,
    orientation: ImageOrientation,

    head_node: Cell<u32>,
    tail: RefCell<Vec<u32>>,
}

impl Drawable for XHeyView{}

impl Drop for XHeyView {
    fn drop(&mut self){
        println!("Drop XHeyView");
    }

}

impl XHeyView {
    pub fn new(view: &UIView) -> Self {
        let layer = view.get_layer();
        let layer = layer.share();


        XHeyView{
            displayFramebuffer:Cell::default(),
            displayRenderbuffer:Cell::default(),
            backingSize:Cell::default(),
            layer:layer,
            orientation: ImageOrientation::portrait,
            head_node:Cell::default(),
            tail:RefCell::default()
        }
    }


    fn renderFrame(&self, framebuffers: &Vec<Framebuffer>){



        sharedImageProcessingContext.makeCurrentContext();

        if self.displayFramebuffer.get() == 0 {
            self.createDisplayFramebuffer()
        }

        let framebuffer = &framebuffers[0];

        self.activateDisplayFramebuffer();
        clearFramebufferWithColor(Color::black());

        let program = &sharedImageProcessingContext.passthroughShader;

        let verticallyInvertedImageVertices: [f32;8] = [-1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0];

        let scaledVertices = FillMode::preserveAspectRatio.transformVertices(verticallyInvertedImageVertices,framebuffer.sizeForTargetOrientation(self.orientation),self.backingSize.get());


        let inputTexture = framebuffer.texturePropertiesForTargetOrientation(self.orientation);

        let vertex = InputTextureStorageFormat::textureCoordinate(scaledVertices);

        renderQuadWithShader(program,&vec![inputTexture],vertex);

        unsafe {
            glBindRenderbuffer(GL_RENDERBUFFER,self.displayRenderbuffer.get());
        }

        sharedImageProcessingContext.presentBufferForDisplay();



    }

    fn activateDisplayFramebuffer(&self) {
        unsafe {
            glBindBuffer(GL_FRAMEBUFFER,self.displayRenderbuffer.get());
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


impl Edge<Framebuffer> for XHeyView {
    fn add_head_node(&self, edge: u32){
        self.head_node.set(edge);
    }

    /// 将ni加入这个节点的输入序列
    fn add_tail(&self, node: u32){
        self.tail.borrow_mut().push(node);
    }

    /// 返回输入序列 这里的实现很奇怪，应该有其他更好的办法？
    fn tail_nodes(&self) -> Vec<u32>{
        let inputs = self.tail.borrow();
        let mut outputs = Vec::new();
        for input in inputs.iter() {
            outputs.push(input.clone());
        }
        outputs
    }

    /// 节点在图中的序号
    fn head_node(&self) -> u32{
        self.head_node.get()
    }

    /// 指定输入最大个数
    fn arity(&self) -> u32{
        1
    }

    /// 前向计算 在XheyView中实现这个Trait，应该做的是将xs的Framebuffer绘制到View上，返回一个占位符占位符
    fn forward(&self, xs: &Vec<Framebuffer>) -> Framebuffer{
        self.renderFrame(xs);
        PlaceHolder::new()
    }

    fn forward_default(&self) -> Framebuffer{
        PlaceHolder::new()
    }


    ///针对Source节点，在渲染过程中指定其Framebufer
    fn set_framebuffer(&self, value:Framebuffer){}
}
