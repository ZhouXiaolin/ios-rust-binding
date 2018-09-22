use super::{Node,Framebuffer};

pub trait Edge {
    /// 将ni加入这个节点的输入序列
    fn add_tail(&self, tail: u32);

    fn add_head_node(&self, head_node: u32);

    /// 返回输入序列
    fn tail_nodes(&self) -> Vec<u32>;

    /// 节点在图中的序号
    fn head_node(&self) -> u32;

    /// 指定输入最大个数
    fn arity(&self) -> u32;

    /// 前向计算
    fn forward(&self, xs: &Vec<Framebuffer>) -> Framebuffer;

    ///针对Source节点，在渲染过程中指定其Framebufer
    fn set_framebuffer(&self, value:Framebuffer);
}

#[repr(C)]
pub struct Graph<'a>{
    nodes: Vec<Node>,
    edges: Vec<Box<&'a dyn Edge>>,

}
pub type VariableIndex = u32;

impl<'a> Graph<'a> {
    pub fn new() -> Self {
        Graph{
            nodes: Vec::default(),
            edges: Vec::default()
        }
    }

    /// 清空关系图 一般用于重新构建一个图
    pub fn reset(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }

    /// 这个函数用来添加输入
    pub fn add_input(&mut self, name:&str, op: &'a dyn Edge) -> VariableIndex {
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
    pub fn add_function(&mut self, name:&str, arguments: &[u32], function: &'a dyn Edge) -> VariableIndex {
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
            let inner_node: &mut Node = nodes.get_mut(*ni as usize).unwrap();
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
            let in_edge : &Box<&dyn Edge> = edges.get(node.in_edge as usize).unwrap();

            let tail_nodes = in_edge.tail_nodes();
            for tail_node in tail_nodes.iter() {
                let inner_node: &Node = nodes.get(tail_node.clone() as usize).unwrap();

                var_names.push(String::from(inner_node.var_name()));
            }

            println!("N{} [lable={} input{:?}]",nc,node.var_name(),var_names);
            nc += 1;

        }

        for edge in edges.iter() {
            for ni in edge.tail_nodes().iter() {
                println!("N{} ---> N{}",ni,edge.head_node());
            }
        }

    }



    /// 渲染过程 前向计算  这个体系是计算图模型，在这种渲染中
    pub fn forward(&mut self) {

//        let nodes = &mut self.nodes;
//        let edges = &mut self.edges;
//        for node in nodes.iter() {
//            let op: &Box<&dyn Edge> = edges.get(node.id as usize).unwrap();
//
//            println!("op arity{}",op.arity());
//
//            let mut xs = Vec::<Framebuffer>::with_capacity(op.arity() as usize);
//            for (ti,input) in op.inputs().iter().enumerate(){
//
//                let n: &Node = nodes.get(input.clone() as usize).unwrap();
//
//                let framebuffer = n.f.take();
//                framebuffer.lock();
//                xs.insert(ti,framebuffer);
//
//            }
//
//            node.f.set(op.forward(&xs));
//
//
//            for x in xs.iter() {
//                x.unlock();
//            }
//
//        }

    }

    pub fn add_feed(&self, index:u32, value:Framebuffer){
//        let edges = &self.edges;
//        let op:&Box<&dyn Edge> = edges.get(index as usize).expect("Error to get op from edges");
//        op.set_framebuffer(value);

    }

}
