use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
/// 用户数据
/// todo 要将其序列化到磁盘上存储
/// md又多了一个配置文件，这个还要反复读写
/// 明天的任务是把这帮配置文件都移动到一个文件夹中
/// 眼不见心不烦
/// 需要在创建时检查是否有配置文件夹
/// 如果没有，放出示例配置文件，然后退出。
/// 如果有，那就进入服务状态
pub struct Data {}
///错误类型
pub type Error = anyhow::Error;
///上下文类型
pub type PoiseContext<'a> = poise::Context<'a, Data, Error>;