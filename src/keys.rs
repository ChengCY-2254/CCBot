//! 定义类型映射键用于 Serenity 的 TypeMap

use std::sync::Arc;
use serenity::prelude::TypeMapKey;

/// http请求的key
pub struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = reqwest::Client;
}

/// 机器人数据的key
pub struct BotDataKey;

impl TypeMapKey for BotDataKey {
    type Value = Arc<crate::config::Data>;
}