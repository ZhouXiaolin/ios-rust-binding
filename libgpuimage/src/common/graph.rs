use super::{Node,Framebuffer,sharedContext};

pub trait Operation {
    fn append(&self, ni: u32); // 将ni加入这个节点的输入序列
    fn inputs(&self) -> Vec<u32>; // 返回输入序列
    fn index(&self) -> u32; // 节点在图中的序号
    fn arity(&self) -> u32; // 指定输入最大个数
    fn forward(&self, xs: Vec<Framebuffer>) -> Framebuffer; // 前向计算
}


pub struct Graph{
    nodes: Vec<Node>,
    ops: Vec<Box<dyn Operation>>,

}
impl Graph {
    pub fn new() -> Self {
        Graph{
            nodes: Vec::default(),
            ops: Vec::default()
        }
    }

    // 清空关系图 一般用于重新构建一个图
    pub fn reset(&mut self) {
        sharedContext.reset();
        self.nodes.clear();
        self.ops.clear();
    }

    // 这个函数用来做输入
    pub fn placeholder(&mut self, name:&str, op: Box<dyn Operation>) -> u32 {

        let node = Node::new(name);
        let node_id = node.id;
        let nodes = &mut self.nodes;
        nodes.push(node);

        let ops = &mut self.ops;
        ops.push(op);

        node_id

    }


    pub fn forward(&mut self) {

        //渲染过程 前向渲染的计算
        let nodes = &mut self.nodes;
        let ops = &mut self.ops;
        for node in nodes.iter() {
            let op: &Box<dyn Operation> = ops.get(node.id as usize).unwrap();

            let mut xs = Vec::<Framebuffer>::with_capacity(op.arity() as usize);
            for (ti,input) in op.inputs().iter().enumerate(){

                let n: &Node = nodes.get(input.clone() as usize).unwrap();

                // Framebuffer是一个标记类，标记了一个帧缓冲所需要的数据，可以使用clone语义
                xs.insert(ti,n.f.take());

            }

            node.f.set(op.forward(xs));
        }
    }

    pub fn add_function(&mut self, name:&str, arguments: &[u32], function: Box<dyn Operation>) -> u32 {
        let node = Node::new(name);
        let node_id = node.id;
        let nodes = &mut self.nodes;
        nodes.push(node);


        for ni in arguments.iter(){
            function.append(ni.clone());
            let inner_node: &mut Node = nodes.get_mut(*ni as usize).unwrap();
            inner_node.append(node_id);
        }

        let ops = &mut self.ops;
        ops.push(function);

        node_id

    }


    pub fn PrintGraphviz(&self) {

        let ops = &self.ops;
        let nodes = &self.nodes;

        for node in nodes.iter() {

            let mut var_names = Vec::<String>::new();
            let op:&Box<dyn Operation> = ops.get(node.id as usize).unwrap();
            let inputs = op.inputs();
            for input in inputs.iter() {
                let inner_node: &Node = nodes.get(input.clone() as usize).unwrap();

                var_names.push(String::from(inner_node.var_name()));
            }


            println!("N{} [lable={} input{:?}]",node.id,node.var_name(),var_names);


        }

        for op in ops.iter() {
            for ni in op.inputs().iter() {
                println!("N{} -> N{}",ni,op.index())
            }
        }
    }

}