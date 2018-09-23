///
use super::Framebuffer;
use super::Tensor;
pub trait Edge<T:Tensor> {
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
    fn forward(&self, xs: &Vec<T>) -> T;
    fn forward_default(&self) -> T;

    ///针对Source节点，在渲染过程中指定其Framebufer
    fn set_framebuffer(&self, value:T);
}


/// 渲染
pub trait Renderable{

}


/// 绘制到屏幕 或者 文件
pub trait Drawable{

}

/// 计算
pub trait Computeale{

}

