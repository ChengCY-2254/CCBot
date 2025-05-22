//! 这里存放系统的一些工具函数
use tokio::runtime::Runtime;

#[inline]
pub fn runtime() -> Runtime {
    Runtime::new().unwrap()
}

