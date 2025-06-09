//! 这里存放系统的一些工具函数

use anyhow::Context;
use chrono::{DateTime, FixedOffset, Utc};
use poise::CreateReply;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufReader;
use tokio::runtime::Runtime;
use tracing::instrument;

const UTF8: FixedOffset = FixedOffset::east_opt(8 * 3600).unwrap();

/// 格式化时间为UTC8
pub fn with_time_to_utc8(time: DateTime<Utc>) -> DateTime<FixedOffset> {
    time.with_timezone(&UTF8)
}
#[inline]
/// 创建一个新的 Tokio 运行时
pub fn runtime() -> Runtime {
    Runtime::new().unwrap()
}

/// 读取配置文件并反序列化为指定类型
#[instrument]
pub fn read_file<P: AsRef<std::path::Path> + std::fmt::Debug, T: DeserializeOwned>(
    path: P,
) -> crate::Result<T> {
    let path = path.as_ref();
    let file = File::open(path).context(format!("Unable to open {:?}", path))?;
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader)?;
    Ok(data)
}

/// 创建一个仅用户可见的消息
#[inline]
pub fn create_ephemeral_reply(content: impl Into<String>) -> CreateReply {
    CreateReply::default().content(content).ephemeral(true)
}

/// 检查是否存在配置目录
pub fn check_config_dir_exists() -> crate::Result<()> {
    let config_dir = std::path::Path::new("config");
    if config_dir.join(".env").is_file()
        && config_dir.join("data.json").is_file()
        && config_dir.join("奶盖波波糖.md").is_file()
    {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "config dir exists but config file not found"
        ))
    }
}

/// 检查给定文件是否存在，如果不存在，则尝试创建它并调用给定函数对其写入
pub fn create_file_and_process_if_missing<F>(
    path: impl AsRef<std::path::Path>,
    processor: F,
) -> crate::Result<()>
where
    F: FnOnce(File) -> crate::Result<()>,
{
    let path = path.as_ref();
    #[allow(clippy::needless_return)]
    if path.exists() {
        return Ok(());
    } else {
        let file = File::create(path)
            .map_err(|why| format!("Unable to create {:?}, because: {}", path, why))
            .unwrap();
        processor(file)
    }
}
