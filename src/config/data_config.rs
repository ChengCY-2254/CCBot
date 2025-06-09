//! 机器人配置文件

use crate::config::{ActivityData, ai_config};
use crate::shared::UpSafeCell;
use crate::utils::read_file;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serenity::all::{ChannelId, GuildChannel, UserId};
use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;

/// 机器人需要保存的配置类型
pub type Data = UpSafeCell<DataConfig>;

lazy_static! {
    pub static ref APP_STATE_MANAGER: GlobalConfigManager = GlobalConfigManager::new()
        .map_err(|e| {
            log::error!("Error loading data: {:?}", e);
            anyhow::anyhow!("Error loading data from config/data.json because: {}", e)
        })
        .unwrap();
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(deny_unknown_fields)]
/// 用户数据
/// md又多了一个配置文件，这个还要反复读写
/// 明天的任务是把这帮配置文件都移动到一个文件夹中
/// 眼不见心不烦
/// 需要在创建时检查是否有配置文件夹
/// 如果没有，放出示例配置文件，然后退出。
/// 如果有，那就进入服务状态
pub struct DataConfig {
    /// 需要监控的频道ID，进了这个set的频道发送消息后都会撤回。
    pub monitored_channels: HashSet<ChannelId>,
    /// ai配置
    pub aiconfig: ai_config::AIConfig,
    /// 所有的owner
    pub owners: HashSet<UserId>,
    /// 机器人的活动
    pub bot_activity: ActivityData,
    /// 当前正在播放的频道
    pub current_voice_channel:Option<GuildChannel>
}

impl DataConfig {
    /// 添加一个需要监控的频道
    pub fn add_monitored_channel(&mut self, channel_id: ChannelId) {
        self.monitored_channels.insert(channel_id);
    }
    /// 删除一个需要监控的频道
    pub fn remove_monitored_channel(&mut self, channel_id: ChannelId) {
        self.monitored_channels.remove(&channel_id);
    }
}

impl DataConfig {
    /// 给定一个路径，读取数据文件并返回数据
    pub fn new(path: impl AsRef<Path> + std::fmt::Debug) -> crate::Result<DataConfig> {
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
    /// 保存到config目录下
    pub fn save_to_config(&self) -> crate::Result<()> {
        self.save("config")
    }
}

/// 全局配置管理器，不用库自带的管理器。
/// 在管理器中创建并读取配置文件，然后通过管理器将配置文件映射出去。
/// 该管理器将采用LazyStatic进行初始化，然后通过get_global_data获取数据。
/// 也可以使用lambda来对作用域进行限制。
pub struct GlobalConfigManager {
    inner: Arc<Data>,
}

impl GlobalConfigManager {
    pub fn new() -> crate::Result<Self> {
        let mut inner = DataConfig::new("config/data.json").map_err(|e| {
            log::error!("Error loading data: {:?}", e);
            anyhow::anyhow!("Error loading data from config/data.json because: {}", e)
        })?;
        inner.aiconfig.init_prompt()?;
        unsafe {
            Ok(Self {
                inner: Arc::new(UpSafeCell::new(inner)),
            })
        }
    }

    pub fn get_app_state(&self) -> Arc<Data> {
        Arc::clone(&self.inner)
    }

    /// 保存数据需要注意把之前借用的数据释放掉
    pub fn save(&self) -> crate::Result<()> {
        self.inner.access().save_to_config()
    }
}
