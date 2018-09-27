// 用图来组合渲染计算过程太复杂，应该包装一个更好的api
use super::Graph;
use super::Tensor;
use super::VariableIndex;

struct Expression<'a,T:Tensor>{
    g: Graph<'a,T>,
    i: VariableIndex,
    graph_id: u32
}



