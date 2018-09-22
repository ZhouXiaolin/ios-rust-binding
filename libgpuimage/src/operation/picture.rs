
use super::*;

use gles_rust_binding::*;
use std::os::raw::c_void;
use std::cell::{RefCell,Cell};
#[repr(C)]
pub struct XheyPicture{
    _framebuffer: Cell<Framebuffer>,
    index:Cell<u32>,
    inputs: RefCell<Vec<u32>>
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
            _framebuffer: Cell::new(framebuffer),
            index:Cell::default(),
            inputs:RefCell::default()
        }
    }

}



impl Operation for XheyPicture{
    fn append_edge(&self, edge: u32){
        self.index.set(edge);
    }
    /// 将ni加入这个节点的输入序列
    fn append_node(&self, node: u32){
        self.inputs.borrow_mut().push(node);
    }

    /// 返回输入序列
    fn inputs(&self) -> Vec<u32>{
        let inputs = self.inputs.borrow();
        let mut outputs = Vec::new();
        for input in inputs.iter() {
            outputs.push(input.clone());
        }
        outputs
    }

    /// 节点在图中的序号
    fn index(&self) -> u32{
        self.index.get()
    }

    /// 指定输入最大个数
    fn arity(&self) -> u32{
        1
    }

    /// 前向计算
    fn forward(&self, xs: &Vec<Framebuffer>) -> Framebuffer{
        let fb = self._framebuffer.take();
        self._framebuffer.set(fb.clone());
        fb
    }

    ///针对Source节点，在渲染过程中指定其Framebufer
    fn set_framebuffer(&self, value:Framebuffer){
        self._framebuffer.set(value)
    }
}

