
use super::*;
use gles_rust_binding::*;
use std::os::raw::c_void;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::{RefCell,Cell};
#[repr(C)]
#[derive(Debug)]
pub struct XheyPicture{
    framebuffer: Arc<Framebuffer>,
    head_node: Cell<u32>,
    tail: RefCell<Vec<u32>>,
    size: GLSize,
    rotation:Rotation,
    orientation: ImageOrientation,
    uniformSettings: ShaderUniformSettings
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


        unsafe {
            glBindTexture(GL_TEXTURE_2D, self.framebuffer.texture);
            glTexSubImage2D(GL_TEXTURE_2D,0,0,0,width,height,FORMAT,GL_UNSIGNED_BYTE,data as *const _);
            glBindTexture(GL_TEXTURE_2D,0);
        }

    }



    pub fn new_texture(textureId: GLuint, width: i32, height: i32, rotation: i32) -> Self {

        let size = GLSize::new(width,height);
        let framebuffer = Arc::new(Framebuffer::new_texture(ImageOrientation::portrait,size,textureId));
        XheyPicture{
            framebuffer,
            head_node:Cell::default(),
            tail:RefCell::default(),
            size: GLSize::new(width, height),
            rotation: Rotation::from(rotation),
            orientation: ImageOrientation::portrait,
            uniformSettings: ShaderUniformSettings::default()
        }
    }



    pub fn new(data: *const c_void, width: i32, height: i32) -> Self {



        let size = GLSize::new(width,height);

        let framebuffer = Arc::new(Framebuffer::new_default(ImageOrientation::portrait,size,true));

        unsafe {
            glBindTexture(GL_TEXTURE_2D,framebuffer.texture);
            glTexImage2D(GL_TEXTURE_2D,0,GL_RGBA as i32,width,height,0,FORMAT,GL_UNSIGNED_BYTE,data as *const _);
            glBindTexture(GL_TEXTURE_2D,0);
        }


        XheyPicture{
            framebuffer,
            head_node:Cell::default(),
            tail:RefCell::default(),
            size: GLSize::new(width, height),
            rotation: Rotation::noRotation,
            orientation:ImageOrientation::portrait,
            uniformSettings: ShaderUniformSettings::default()
        }
    }


    pub fn updateOrientation(&mut self, orient: i32) {
        self.orientation = ImageOrientation::from(orient);
    }

//    pub fn update(&mut self, textureId: GLuint){
//
//        let framebuffer = Arc::new(Framebuffer::new_texture(ImageOrientation::portrait,self.size,textureId));
//
//        self.framebuffer = framebuffer;
//    }


}



impl Edge for XheyPicture{
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

        Some(self.framebuffer.clone())

    }

    fn name(&self) -> &str {
        "picture input"
    }

}

