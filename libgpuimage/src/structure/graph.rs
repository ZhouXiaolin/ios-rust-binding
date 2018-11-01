use super::{Node,Tensor,Edge};
use std::cell::RefCell;
use std::rc::Rc;

/// 使用Rc来保持Tensor引用
/// Graph应当和Context分离
///

#[repr(C)]
pub struct Graph<'a,T:Tensor>{
    nodes: Vec<Node<T>>,
    edges: Vec<Box<&'a dyn Edge<Item=Rc<T>>>>,

}
pub type VariableIndex = u32;


impl<'a,T:Tensor> Drop for Graph<'a,T> {
    fn drop(&mut self){
        self.reset();
        info!("Drop Graph");
    }

}

impl<'a,T:Tensor> Graph<'a,T> {
    pub fn new() -> Self {
        Graph{
            nodes: Vec::default(),
            edges: Vec::default(),
        }
    }

    /// 清空关系图 一般用于重新构建一个图
    pub fn reset(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }

    /// 这个函数用来添加输入
    pub fn add_input(&mut self, name:&str, op: &'a dyn Edge<Item=Rc<T>>) -> VariableIndex {
        let new_node_index = self.nodes.len() as u32;
        let new_edge_index = self.edges.len() as u32;

        let node = Node::new(name,new_edge_index,new_node_index);
        let nodes = &mut self.nodes;
        nodes.push(node);

        op.add_head_node(new_node_index);
        let edges = &mut self.edges;
        edges.push(Box::new(op));

        new_node_index

    }

    /// 这个函数用来添加关系 arguments是输入节点，function是操作节点 执行的操作就是前向计算
    pub fn add_function(&mut self, name:&str, arguments: &[u32], function: &'a dyn Edge<Item=Rc<T>>) -> VariableIndex {
        let new_node_index = self.nodes.len() as u32;
        let new_edge_index = self.edges.len() as u32;

        let node = Node::new(name,new_edge_index,new_node_index);
        let nodes = &mut self.nodes;
        nodes.push(node);


        function.add_head_node(new_node_index);
        let edges = &mut self.edges;
        edges.push(Box::new(function));

        for ni in arguments.iter(){
            function.add_tail(ni.clone());
            let inner_node: &mut Node<_> = nodes.get_mut(*ni as usize).unwrap();
            inner_node.add_out_edge(new_edge_index);
        }



        new_node_index

    }


    /// 用来打印图结构
    pub fn PrintGraphviz(&self) {

        let edges = &self.edges;
        let nodes = &self.nodes;
        let mut nc = 0;

        for node in nodes.iter() {

            let mut var_names = Vec::<String>::new();
            let in_edge : &Box<&dyn Edge<Item=Rc<T>>> = edges.get(node.in_edge as usize).unwrap();

            let tail_nodes = in_edge.tail_nodes();
            for tail_node in tail_nodes.iter() {
                let inner_node: &Node<_> = nodes.get(tail_node.clone() as usize).unwrap();

                var_names.push(String::from(inner_node.var_name()));
            }

            info!("N{} [lable={} input{:?}]",nc,node.var_name(),var_names);
            nc += 1;

        }

        for edge in edges.iter() {
            for ni in edge.tail_nodes().iter() {
                info!("N{} ---> N{}",ni,edge.head_node());
            }
        }

    }



    /// 渲染过程 前向计算
    pub fn forward(&self) {

        let nodes = &self.nodes;
        let edges = &self.edges;

        for node in nodes {

            let in_edge : &Box<&Edge<Item=Rc<T>>> = edges.get(node.in_edge as usize).expect("Error, cannot get in_edge from edges");
            let mut xs = Vec::<Rc<T>>::with_capacity(in_edge.arity() as usize);
            //  如果in_edge的arity为0，为input节点，不会进入这个循环
            for (ti,tail_node_index) in in_edge.tail_nodes().iter().enumerate() {
                let inner_node : &Node<_> = nodes.get(tail_node_index.clone() as usize).expect("Error, cannot get inner node from nodes");
                let mut fbo_vec = inner_node.f.borrow_mut();
                let fbo = fbo_vec.pop().unwrap();
                xs.insert(ti,fbo.clone());
                fbo_vec.push(fbo);

            }


            if let Some(v) = in_edge.forward(&xs) {
                for _ in &node.out_edges {
                    // 从当前节点出发，有多少target,就需要lock多少次
                    v.lock();
                }

                let mut f = node.f.borrow_mut();
                if f.len() > 0 { f.pop(); }

                f.push(v)
            }

            for x in xs {
                // 这个节点的forward完毕后，unlock他的输入
                x.unlock();
            }

        }



    }

}
