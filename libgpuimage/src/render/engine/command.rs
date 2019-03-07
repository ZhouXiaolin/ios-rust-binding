use std::fmt;
use std::rc::Rc;

pub enum PrimitiveType {
    Point,
    Line,
    LineStrip,
    Triangle,
    TriangleStrip
}

pub enum BufferType{
    VertexBuffer,
    IndexBuffer
}

pub enum Usage{
    Immutable,
    Dynamic,
    Stream
}


pub struct BufferDesc{}

use libc::c_void;
use std::ptr;
pub struct Buffer{
    pub size: i32,
    pub type_: BufferType,
    pub usage: Usage,
    pub content: *mut c_void
}
impl Buffer {
    pub fn new() -> Self {
        Buffer{
            size:0,
            type_:BufferType::IndexBuffer,
            usage: Usage::Immutable,
            content: ptr::null_mut()
        }
    }
}


use gles_rust_binding::GLProgram;


pub struct LayoutDesc{

}
pub enum IndexType {
    None,
    UInt16,
    UInt32
}


pub struct Pipeline<'a>{
    pub shader: &'a GLProgram,
    pub layout_desc: LayoutDesc,
    pub primitive_type: PrimitiveType,
    pub index_type: IndexType
}

pub struct DrawState<'a>{
    pub pipeline: Pipeline<'a>,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub vs_image: Image,
    pub fs_image: Image
}

pub enum Action{
    Clear,
    Load,
    Dontcare
}

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let action = match self {
            Action::Clear => "clear",
            Action::Load => "load",
            Action::Dontcare => "dontcare"
        };
        write!(f, "({:?})", action)
    }
}

pub struct ColorAttachmentAction{
    pub action: Action,
    pub val: [f32;4]
}

impl Default for ColorAttachmentAction {
    fn default() -> Self{
        ColorAttachmentAction{
            action: Action::Clear,
            val:[0.0, 1.0, 0.0, 1.0]
        }
    }
}
impl ColorAttachmentAction {
    pub fn run(&self){
        unsafe {
            gles_rust_binding::glClearColor(self.val[0],self.val[1],self.val[2],self.val[3]);
            gles_rust_binding::glClear(gles_rust_binding::GL_COLOR_BUFFER_BIT);
        }
    }
}

impl fmt::Debug for ColorAttachmentAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "action:{:?} val:{:?}", self.action, self.val)
    }
}


#[derive(Default, Debug)]
pub struct PassAction{
    pub color: ColorAttachmentAction
}

impl PassAction {
    pub fn run(&self) {

    }
}


pub enum ImageType{}
pub enum PixelFormat{}
pub enum Filter{}
pub struct Image{}

pub struct AttachmentDesc{
    image: Image,
    mip_level: i32,
}

pub struct PassDesc{
    pub color_attachments: [AttachmentDesc;4],
    pub depth_stencil_attachment: AttachmentDesc
}


use crate::structure::Tensor;
use std::sync::Arc;
pub struct Pass<T:Tensor>{
    pub color_attachment: Rc<T>
}




pub enum ShaderStage{}


pub struct Command{

}
use crate::render::RenderTarget;
impl Command {
    pub fn new() -> Self {
        Command{}
    }


    pub fn run_pass<T:Tensor + RenderTarget,F>(
        &self,
        pass_id: &Pass<T>,
        pass_action: &PassAction,
        operation: F)
        where F : FnOnce() -> (){

        pass_id.color_attachment.bindFramebufferForRendering();
        pass_action.color.run();

        operation();

        pass_id.color_attachment.unbindFramebufferForRendering();
    }

}

pub struct CommandEncoder {}

impl CommandEncoder {
    pub fn apply_draw_state(&self, draw_state: &DrawState){

        draw_state.pipeline.shader.bind();


    }

    pub fn apply_uniform_block(&self, stage: ShaderStage, ub_index: i32, data: *mut i32, num_bytes: i32){

    }

    pub fn draw(&self, base_element: i32, num_elements: i32, num_instances: i32){

    }

}
