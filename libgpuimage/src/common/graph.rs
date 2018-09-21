use super::{Node,Framebuffer,sharedContext};

pub trait Operation {
    /// 将ni加入这个节点的输入序列
    fn append(&self, ni: u32);

    /// 返回输入序列
    fn inputs(&self) -> Vec<u32>;

    /// 节点在图中的序号
    fn index(&self) -> u32;

    /// 指定输入最大个数
    fn arity(&self) -> u32;

    /// 前向计算
    fn forward(&self, xs: Vec<Framebuffer>) -> Framebuffer;

    ///针对Source节点，在渲染过程中指定其Framebufer
    fn set_framebuffer(&self, value:Framebuffer);
}

#[repr(C)]
pub struct Graph<'a>{
    nodes: Vec<Node>,
    ops: Vec<Box<&'a dyn Operation>>,

}

impl<'a> Graph<'a> {
    pub fn new() -> Self {
        Graph{
            nodes: Vec::default(),
            ops: Vec::default()
        }
    }

    /// 清空关系图 一般用于重新构建一个图
    pub fn reset(&mut self) {
        sharedContext.reset();
        self.nodes.clear();
        self.ops.clear();
    }

    /// 这个函数用来做输入
    pub fn placeholder(&mut self, name:&str, op: &'a dyn Operation) -> u32 {

        let node = Node::new(name);
        let node_id = node.id;
        let nodes = &mut self.nodes;
        nodes.push(node);

        let ops = &mut self.ops;
        ops.push(Box::new(op));

        node_id

    }

    /// 这个函数用来添加关系 arguments是输入节点，function是操作节点 执行的操作就是前向计算
    pub fn add_function(&mut self, name:&str, arguments: &[u32], function: &'a dyn Operation) -> u32 {
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
        ops.push(Box::new(function));

        node_id

    }


    /// 用来打印图结构
    pub fn PrintGraphviz(&self) {

        let ops = &self.ops;
        let nodes = &self.nodes;

        for node in nodes.iter() {

            let mut var_names = Vec::<String>::new();
            let op:&Box<&dyn Operation> = ops.get(node.id as usize).unwrap();
            let inputs = op.inputs();
            for input in inputs.iter() {
                let inner_node: &Node = nodes.get(input.clone() as usize).unwrap();

                var_names.push(String::from(inner_node.var_name()));
            }


            println!("N{} [lable={} input{:?}]",node.id,node.var_name(),var_names);


        }

        for node in nodes.iter() {
            let inputs: Vec<u32> = ops.get(node.id as usize).unwrap().inputs();
            for input in inputs.iter(){
                let inner_node: &Node = nodes.get(input.clone() as usize).unwrap();
                println!("N{} -> N{}",inner_node.id, node.id);
            }
        }
    }



    /// 渲染过程 前向计算  这个体系是计算图模型，在这种渲染中
    pub fn forward(&mut self) {

        let nodes = &mut self.nodes;
        let ops = &mut self.ops;
        for node in nodes.iter() {
            let op: &Box<&dyn Operation> = ops.get(node.id as usize).unwrap();

            let mut xs = Vec::<Framebuffer>::with_capacity(op.arity() as usize);
            for (ti,input) in op.inputs().iter().enumerate(){

                let n: &Node = nodes.get(input.clone() as usize).unwrap();

                xs.insert(ti,n.f.take());

            }

            node.f.set(op.forward(xs));
        }

    }

    pub fn add_feed(&self, index:u32, value:Framebuffer){
        let ops = &self.ops;
        let op:&Box<&dyn Operation> = ops.get(index as usize).expect("Error to get op from ops");
        op.set_framebuffer(value);

    }

}