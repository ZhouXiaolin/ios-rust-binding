
pub enum PrimitiveType {
    Point,
    Line,
    LineStrip,
    Triangle,
    TriangleStrip
}


pub trait PassData<'a> {}

pub trait CommandEncoder: for<'a> PassData<'a> {}



struct BufferDesc{}
struct Buffer{

}
impl Buffer {
    fn new(desc: BufferDesc) -> Self {
        Buffer{}
    }
}
struct Shader{}
struct Pipeline{}
struct DrawState{}
struct PassAction{}
struct Pass{}
enum ShaderStage{}


struct Encoder{

}

impl Encoder {
    fn new() -> Self {
        Encoder{}
    }

    fn begin_default_pass(&self, pass_action: &PassAction, width: i32, height: i32){

    }

    fn begin_pass(&self, pass_id: &Pass, pass_action: &PassAction){

    }

    fn apply_draw_state(&self, draw_state: &DrawState){

    }

    fn apply_uniform_block(&self, stage: ShaderStage, ub_index: i32, data: *mut i32, num_bytes: i32){

    }

    fn draw(&self, base_element: i32, num_elements: i32, num_instances: i32){

    }

    fn end_pass(&self){

    }

    fn commit(&self){

    }
}

use std::ptr;
struct Pool{
    size: i32,
    unique_counter: u32,
    queue_top: i32,
    free_queue: Vec<i32>
}

impl Pool {
    fn init(num: i32) -> Self {
        Pool {
            size: num+1,
            queue_top:0,
            unique_counter:0,
            free_queue: Vec::with_capacity(num as usize)
        }
    }
}