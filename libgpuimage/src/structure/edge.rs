///
use super::Tensor;
pub trait Edge {
    type Item;
    /// 将tail加入这个节点的输入序列
    fn add_tail(&self, tail: u32);
    /// 返回输入序列
    fn tail_nodes(&self) -> Vec<u32>;

    fn add_head_node(&self, head_node: u32);
    /// 节点在图中的序号
    fn head_node(&self) -> u32;

    /// 指定输入最大个数
    fn arity(&self) -> u32;

    /// 前向计算
    fn forward(&self, xs: &Vec<Self::Item>) -> Self::Item;
}


/// 渲染
/// 渲染过程
pub trait Renderable{

}


/// 绘制到屏幕 或者 文件
/// 如果一个Edge是Drawable的，其forward的参数长度必定为1，返回值为PlaceHolder，
pub trait Drawable{

}

/// 计算
/// 计算过程
pub trait Computeale{

}

