use crate::{read_file, UpSafeCell};
use serde::{Deserialize, Serialize};
use serenity::all::ChannelId;
use std::collections::HashSet;
use std::path::Path;

pub type Data = UpSafeCell<DataInner>;

pub type ExportVec = Vec<poise::Command<(), Error>>;

#[derive(Serialize, Deserialize,Clone)]
#[serde(deny_unknown_fields)]
/// 用户数据
/// todo 要将其序列化到磁盘上存储
/// md又多了一个配置文件，这个还要反复读写
/// 明天的任务是把这帮配置文件都移动到一个文件夹中
/// 眼不见心不烦
/// 需要在创建时检查是否有配置文件夹
/// 如果没有，放出示例配置文件，然后退出。
/// 如果有，那就进入服务状态
pub struct DataInner {
    /// 需要监控的频道ID，进了这个set的频道发送消息后都会撤回。
    pub monitored_channels: HashSet<ChannelId>,
}
///错误类型
pub type Error = anyhow::Error;
///上下文类型
pub type PoiseContext<'a> = poise::Context<'a, (), Error>;

impl DataInner {
    /// 添加一个需要监控的频道
    pub fn add_monitored_channel(&mut self, channel_id: ChannelId) {
        self.monitored_channels.insert(channel_id);
    }
    /// 删除一个需要监控的频道
    pub fn remove_monitored_channel(&mut self, channel_id: ChannelId) {
        self.monitored_channels.remove(&channel_id);
    }
}

impl DataInner {
    pub fn new(path: impl AsRef<Path>) -> crate::Result<DataInner> {
        read_file(path)
    }
    /// 保存数据文件
    /// path 为配置文件夹
    pub fn save(&self, config_dir_path: impl AsRef<Path>) -> crate::Result<()> {
        let path = config_dir_path.as_ref();
        if !path.exists() {
            std::fs::create_dir_all(path)?;
        }
        let file_path = path.join("data.json");
        let file = std::fs::File::create(file_path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }
}
