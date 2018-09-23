/// Tensor 张量 抽象概念 所有矩阵 帧缓冲 图像 采样器都是张量这个概念
/// lock和unlock用来实现RC，缓存机制，也可以是空实现

pub trait Tensor {
    fn lock(&self);
    fn unlock(&self);
}

