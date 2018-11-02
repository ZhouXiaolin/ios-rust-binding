

/// shader抽象
///

#[derive(Debug,Default)]
pub struct RenderPipelineState {

}

impl RenderPipelineState {

}

// Device是个空实现
#[derive(Debug, Default)]
pub struct Device {

}

impl Device {
    pub fn makeCommandQueue(&self) -> CommandQueue {
        CommandQueue{}
    }
}


// 这个实现不确定 暂为空
#[derive(Debug, Default)]
pub struct CommandQueue {

}

impl CommandQueue {
    pub fn new() -> CommandQueue {
        CommandQueue{}
    }

    pub fn makeCommandBuffer(&self) -> CommandBuffer {
        CommandBuffer{}
    }
}



/// 命令队列抽象
///

#[derive(Debug,Default)]
pub struct CommandBuffer{

}

impl CommandBuffer {
    pub fn makeRenderCommandEncoder(&self, renderPass: &RenderPassDescriptor) -> RenderCommandEncoder {
        RenderCommandEncoder{}
    }
}

trait CommandEncoder {}


type Texture = u32;
type Color = u32;

enum LoadAction {

}

/// colorattachments
/// 可能存在的fbo --> framebuffer + texture
/// 清屏操作 glClear glClearColor
#[derive(Debug,Default)]
pub struct RenderPassDescriptor {
    pub texture: Texture,
    pub clearColor: Color
}

impl RenderPassDescriptor {

}

///

#[derive(Debug,Default)]
pub struct RenderCommandEncoder {

}

impl CommandEncoder for RenderCommandEncoder {}

impl RenderCommandEncoder {
    pub fn setRenderPipelineState(&self, pipelineState: &RenderPipelineState) {

    }

    pub fn setVertexBuffer(&self){

    }

    pub fn setFragmentTexture(&self){

    }

    pub fn setViewport(&self){

    }

    pub fn drawPrimitives(&self){

    }

    pub fn endEncoding(&self){

    }

}



