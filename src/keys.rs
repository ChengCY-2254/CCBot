//! 定义类型映射键用于 Serenity 的 TypeMap

use serenity::prelude::TypeMapKey;

/// http请求的key
pub struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = reqwest::Client;
}
