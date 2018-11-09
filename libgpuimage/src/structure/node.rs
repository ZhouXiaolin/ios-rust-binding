use std::cell::{RefCell};
use super::Tensor;
use std::rc::Rc;
use std::sync::Arc;

pub struct Node<T:Tensor> {

    pub node_id: u32,
    pub name: String,
    pub in_edge: u32,
    pub out_edges: Vec<u32>,
    pub f:RefCell<Vec<Arc<T>>> // 应该如何修改此处？

}

impl<T:Tensor> Node<T> {
    pub fn new(name:&str,in_edge_index:u32, nid: u32) -> Self {
        Node {
            node_id: nid,
            name: String::from(name),
            in_edge:in_edge_index,
            out_edges: Vec::default(),
            f:RefCell::default()
        }
    }

    pub fn var_name(&self) -> &str {
        &self.name
    }

    pub fn add_out_edge(&mut self, out_edge: u32) {
        self.out_edges.push(out_edge);
    }
}
