/// Tensor 张量 抽象概念 所有 Framebuffer Image Sampler 数据

pub trait Tensor {
    fn lock(&self);
    fn unlock(&self);
}

