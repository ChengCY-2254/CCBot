//! 机器人配置文件工具包
pub mod ai_config;
pub mod data_config;
use crate::UpSafeCell;
use lazy_static::lazy_static;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serenity::all::{ActivityType, Message};

lazy_static! {
    /// 系统提示
    static ref SYS_USER_PTOMPT_MESSAGE: AIMessage =
        AIMessage::new("system", "以下是用户的最新输入");
    /// 不使用@规则
     static ref NO_AT_PROMPT_MESSAGE:AIMessage=AIMessage::new("system","这里是用户在对你私聊，不能使用@");
    /// 在这里缓存住系统提示
     static ref SYSTEM_PROMPT_CACHE: UpSafeCell<String> = unsafe {UpSafeCell::new(String::new())};
}

#[derive(Serialize, Debug, Clone)]
/// AI消息结构体
pub struct AIMessage {
    role: String,
    content: String,
}

impl AIMessage {
    /// 创建一个新的AI消息
    pub fn new<S: Into<String>>(role: S, content: S) -> Self {
        AIMessage {
            role: role.into(),
            content: content.into(),
        }
    }

    /// 将Discord的Message转换为AIMessage
    pub fn map_ai_message(message: Message) -> AIMessage {
        let message_id = message.id;
        let message_author_id = message.author.id;
        let message_times_map = message.timestamp;
        let role = if message.author.bot {
            "assistant".to_string()
        } else {
            "user".to_string()
        };

        let content = format!(
            "SYSTEM：以下内容为补充参考信息:\r\nmessage_id:{message_id}\r\nuser_id:{message_author_id}\r\nmessage_times_map:{message_times_map}\r\n
        SYSTEM:补充说明结束，以下是用户内容\r\n\\{}",
            message.content
        );
        AIMessage { role, content }
    }
}

/// 从[serenity::gateway::ActivityData]中拷贝，因为它没实现反序列化，所以创建一个。
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ActivityData {
    pub name: String,
    pub kind: ActivityType,
    pub state: Option<String>,
    pub url: Option<Url>,
}

impl From<ActivityData> for serenity::gateway::ActivityData {
    fn from(value: ActivityData) -> Self {
        serenity::gateway::ActivityData {
            name: value.name,
            kind: value.kind,
            state: value.state,
            url: value.url,
        }
    }
}

impl From<serenity::gateway::ActivityData> for ActivityData {
    fn from(value: serenity::gateway::ActivityData) -> Self {
        ActivityData {
            name: value.name,
            kind: value.kind,
            state: value.state,
            url: value.url,
        }
    }
}