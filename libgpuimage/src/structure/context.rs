/// context 用于图的前向计算中 计算环境的初始化
/// 其实例应当对应各种图形api的具体实现 可以是空实现

pub trait Context {
    fn makeCurrentContext(&self);
    fn runOperationAsynchronously<T, F:FnOnce()->T>(&self, operation: F) -> T{
        self.makeCurrentContext();
        operation()
    }

    fn runOperationSynchronously<T, F:FnOnce()->()>(&self, operation: F){
        self.makeCurrentContext();
        operation()
    }
}