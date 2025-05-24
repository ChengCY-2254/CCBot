use serenity::prelude::TypeMapKey;

/// http请求的key
pub struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = reqwest::Client;
}

pub struct BotDataKey;

impl TypeMapKey for BotDataKey {
    type Value = crate::model::Data;
}