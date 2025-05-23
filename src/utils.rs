//! 这里存放系统的一些工具函数

use serde::de::DeserializeOwned;
use std::io::BufReader;
use tokio::runtime::Runtime;

#[inline]
pub fn runtime() -> Runtime {
    Runtime::new().unwrap()
}

/// 读取配置文件并反序列化为指定类型
pub async fn read_file<P: AsRef<std::path::Path>, T: DeserializeOwned>(
    path: P,
) -> crate::Result<T> {
    let file = std::fs::File::open(path).expect("Unable to open config file");
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader)?;
    Ok(data)
}
