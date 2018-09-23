/// Tensor 张量 抽象概念 所有矩阵 帧缓冲 图像 采样器都是张量这个概念
/// lock和unlock用来实现RC，缓存机制，也可以是空实现

pub trait Tensor {
    fn lock(&self);
    fn unlock(&self);
}

/// 缓存机制应该写到structure模块吗？
/// request 根据条件，生成或者找出一个Tensor
/// pull 拉取这个Tensor
/// push 使用完毕后回传这个Tensor

pub trait TensorCache<T>{
    fn pull(&self) -> T;
    fn push(&self,t:T);
}
