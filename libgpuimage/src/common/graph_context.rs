use std::cell::{RefCell,Cell};

pub struct Context {
    node_id: Cell<u32>,
    operation_id:Cell<u32>,
}
unsafe impl Sync for Context{}

impl Context {
    pub fn new() -> Self {
        Context{
            node_id:Cell::new(0_u32),
            operation_id:Cell::new(0_u32)
        }
    }

    pub fn node_id(&self) -> u32 {
        let node_id = self.node_id.get();
        self.node_id.set(node_id+1);
        node_id
    }

    pub fn operation_id(&self) -> u32 {
        let operation_id = self.operation_id.get();
        self.operation_id.set(operation_id + 1);
        operation_id
    }

    pub fn reset(&self) {
        self.node_id.set(0_u32);
        self.operation_id.set(0_u32);
    }

}

lazy_static!{
    pub static ref sharedContext : Context = Context::new();
}