use serenity::prelude::TypeMapKey;

/// http请求的key
pub struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = reqwest::Client;
}

/// 机器人数据的key
pub struct BotDataKey;

impl TypeMapKey for BotDataKey {
    type Value = crate::model::Data;
}
/// 机器人的数据库Key
#[allow(unused)]
pub struct BotDB;

impl TypeMapKey for BotDB {
    type Value = sqlx::SqlitePool;
}
