
use super::*;
use gles_rust_binding::*;
use std::os::raw::c_void;
use std::rc::Rc;
use std::cell::{RefCell,Cell};
#[repr(C)]
#[derive(Debug)]
pub struct XheyPicture{
    framebuffer: Rc<Framebuffer>,
    head_node: Cell<u32>,
    tail: RefCell<Vec<u32>>,
    width: i32,
    height: i32
}

impl Drop for XheyPicture {
    fn drop(&mut self){
        println!("Drop XheyPicture");
    }

}

#[cfg(target_os="ios")]
static  FORMAT : GLenum = GL_BGRA;
#[cfg(target_os="android")]
static  FORMAT : GLenum = GL_RGBA;

impl XheyPicture {

    pub fn update(&self, data: *const c_void, width: i32, height: i32){
        if self.width != width || self.height != height {
            panic!("Error!");
        }

        unsafe {
            glBindTexture(GL_TEXTURE_2D, self.framebuffer.texture);
            glTexSubImage2D(GL_TEXTURE_2D,0,0,0,width,height,FORMAT,GL_UNSIGNED_BYTE,data as *const _);
            glBindTexture(GL_TEXTURE_2D,0);
        }

    }



    pub fn new_texture(textureId: GLuint, width: i32, height: i32) -> Self {
        sharedImageProcessingContext.makeCurrentContext();
        let size = GLSize::new(width,height);
        let framebuffer = Rc::new(Framebuffer::new_texture(ImageOrientation::portrait,size,textureId));
        XheyPicture{
            framebuffer,
            head_node:Cell::default(),
            tail:RefCell::default(),
            width,
            height
        }
    }



    pub fn new(data: *const c_void, width: i32, height: i32) -> Self {


        sharedImageProcessingContext.makeCurrentContext();

        let size = GLSize::new(width,height);

        let framebuffer = sharedImageProcessingContext.framebufferCache.requestFramebufferWithDefault(ImageOrientation::portrait,size,true);

        unsafe {
            glBindTexture(GL_TEXTURE_2D,framebuffer.texture);
            glTexImage2D(GL_TEXTURE_2D,0,GL_RGBA as i32,width,height,0,FORMAT,GL_UNSIGNED_BYTE,data as *const _);
            glBindTexture(GL_TEXTURE_2D,0);
        }


        XheyPicture{
            framebuffer,
            head_node:Cell::default(),
            tail:RefCell::default(),
            width,
            height
        }
    }

}



impl Edge for XheyPicture{
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
        Some(self.framebuffer.clone())

    }

    fn name(&self) -> &str {
        "picture input"
    }

}

