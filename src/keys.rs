use serenity::prelude::TypeMapKey;

/// http请求的key
pub struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = reqwest::Client;
}

/// 机器人数据的key
pub struct BotDataKey;

impl TypeMapKey for BotDataKey {
    type Value = crate::config::Data;
}