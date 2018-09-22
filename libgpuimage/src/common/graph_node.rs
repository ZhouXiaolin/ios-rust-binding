use std::cell::{Cell};
use super::Framebuffer;

pub struct Node {
    pub id: u32,
    pub name: String,
    pub output: Vec<u32>,
    pub f:Cell<Option<Framebuffer>>
}

impl Node {
    pub fn new(name:&str,id:u32) -> Self {
        Node {
            id: id,
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
