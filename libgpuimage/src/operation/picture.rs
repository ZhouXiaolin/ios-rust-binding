
use super::*;

use gles_rust_binding::*;
use std::os::raw::c_void;
use std::cell::{RefCell,Cell};
#[repr(C)]
pub struct XheyPicture{
    framebuffer: Framebuffer,
    head_node: Cell<u32>,
    tail: RefCell<Vec<u32>>,
}

impl Drop for XheyPicture {
    fn drop(&mut self){
        println!("Drop XheyPicture");
    }

}

impl XheyPicture {
    pub fn new(data: *const c_void, width: i32, height: i32) -> Self {



        sharedImageProcessingContext.makeCurrentContext();
        let size = GLSize::new(width,height);
        let framebuffer = sharedImageProcessingContext.frameubfferCache.requestFramebufferWithDefault(ImageOrientation::portrait,size,true);

        unsafe {
            glBindTexture(GL_TEXTURE_2D,framebuffer.texture);
            glTexImage2D(GL_TEXTURE_2D,0,GL_RGBA as i32,width,height,0,GL_BGRA,GL_UNSIGNED_BYTE,data as *const _);
            glBindTexture(GL_TEXTURE_2D,0);
        }

        XheyPicture{
            framebuffer: framebuffer,
            head_node:Cell::default(),
            tail:RefCell::default()
        }
    }

}



impl Edge for XheyPicture{
    type Item = Framebuffer;

    fn add_head_node(&self, edge: u32){
        self.head_node.set(edge);
    }

    /// 将ni加入这个节点的输入序列
    fn add_tail(&self, node: u32){
        self.tail.borrow_mut().push(node);
    }

    /// 返回输入序列
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
        0
    }

    /// 前向计算
    fn forward(&self, xs: &Vec<Self::Item>) -> Self::Item{
        self.framebuffer.clone()

    }

    fn name(&self) -> &str {
        "picture input"
    }

}

