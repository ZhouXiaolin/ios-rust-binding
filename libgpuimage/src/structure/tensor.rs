/// Tensor 抽象概念
/// lock和unlock用来实现RC，缓存机制，也可以是空实现

pub trait Tensor {
    fn lock(&self);
    fn unlock(&self);
}
