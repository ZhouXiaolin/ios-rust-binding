use std::cell::{Cell,RefCell};
use super::sharedContext;
use super::Framebuffer;
pub struct Node {
    pub id: u32,
    pub name: String,
    pub output: Vec<u32>,
    pub f:Cell<Framebuffer>
}

impl Node {
    pub fn new(name:&str) -> Self {
        Node {
            id: sharedContext.node_id(),
            name: String::from(name),
            output: Vec::default(),
            f:Cell::default()
        }
    }

    pub fn var_name(&self) -> &str {
        &self.name
    }

    pub fn append(&mut self, id: u32) {
        self.output.push(id);
    }
}
