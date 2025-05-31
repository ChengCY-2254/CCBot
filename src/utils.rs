//! 这里存放系统的一些工具函数

use anyhow::Context;
use chrono::{DateTime, FixedOffset, Utc};
use lazy_static::lazy_static;
use poise::CreateReply;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::cell::{RefCell, RefMut};
use std::fs::File;
use std::io::BufReader;
use std::ops::Deref;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use tracing::instrument;

lazy_static! {
    static ref UTC8: FixedOffset = FixedOffset::east_opt(8 * 3600).unwrap();
}

/// 格式化时间为UTC8
pub fn with_time_to_utc8(time: DateTime<Utc>) -> DateTime<FixedOffset> {
    time.with_timezone(UTC8.deref())
}
#[inline]
/// 创建一个新的 Tokio 运行时
pub fn runtime() -> Runtime {
    Runtime::new().unwrap()
}

/// 读取配置文件并反序列化为指定类型
#[instrument]
pub fn read_file<P: AsRef<std::path::Path> + std::fmt::Debug, T: DeserializeOwned>(path: P) -> crate::Result<T> {
    let path = path.as_ref();
    let file = File::open(path)
        .context(format!("Unable to open {:?}", path))?;
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
pub fn create_file_and_process_if_missing<F>(path: impl AsRef<std::path::Path>, processor: F) -> crate::Result<()>
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

#[derive(Debug, Default)]
/// 线程安全的[Mutex]封装
pub struct DataBox<T>(Arc<Mutex<T>>);

impl<T> DataBox<T> {
    /// 创建一个新的[DataBox]
    pub fn new(data: T) -> Self {
        DataBox(Arc::new(Mutex::new(data)))
    }
}
unsafe impl<T> Send for DataBox<T> {}
unsafe impl<T> Sync for DataBox<T> {}

impl<T> Deref for DataBox<T> {
    type Target = Arc<Mutex<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// UpSafeCell 是一个线程安全的 RefCell 封装
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct UpSafeCell<T>(RefCell<T>);

unsafe impl<T> Sync for UpSafeCell<T> {}
unsafe impl<T> Send for UpSafeCell<T> {}

impl<T> UpSafeCell<T>
where
    T: Send + Sync,
{
    /// 创建一个新的 UpSafeCell
    /// ## Safety
    /// 自行保证安全使用，与[RefCell]一致，这只是一个警告。
    pub unsafe fn new(data: T) -> Self {
        Self(RefCell::new(data))
    }
    /// 获取一个可变引用
    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        self.0.borrow_mut()
    }
    /// 获取一个不可变引用
    pub fn access(&self) -> std::cell::Ref<'_, T> {
        self.0.borrow()
    }
}
