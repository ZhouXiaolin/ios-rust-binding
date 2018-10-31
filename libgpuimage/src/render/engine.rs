

/// shader抽象
///

#[derive(Debug,Default)]
pub struct PipelineState {

}

impl PipelineState {

}

// Device是个空实现
#[derive(Debug, Default)]
pub struct RenderDevice {

}

impl RenderDevice {
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
    pub fn makeRenderCommandEncoder(&self, renderPass: &RenderPass) -> RenderEncoder {
        RenderEncoder{}
    }
}


type Texture = u32;
type Color = u32;

enum LoadAction {

}

///  渲染流
///
#[derive(Debug,Default)]
pub struct RenderPass {
    pub texture: Texture,
    pub clearColor: Color
}

impl RenderPass {

}

///

#[derive(Debug,Default)]
pub struct RenderEncoder {

}

impl RenderEncoder {
    pub fn setRenderPipelineState(&self, pipelineState: &PipelineState) {

    }

    pub fn setVertexBuffer(&self){

    }

    pub fn setFragmentTexture(&self){

    }

    pub fn drawPrimitives(&self){

    }

    pub fn endEncoding(&self){

    }

}



