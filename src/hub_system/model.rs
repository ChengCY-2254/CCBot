//! 一些数据模型和工具方法
#![allow(dead_code)]
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
/// AI消息结构体
pub struct AIMessage {
    role: String,
    content: String,
}

impl AIMessage {
    pub fn new<S: Into<String>>(role: S, content: S) -> Self {
        AIMessage {
            role: role.into(),
            content: content.into(),
        }
    }
}

#[cfg(feature = "ai-chat")]
pub fn into_ai_message(message: &serenity::all::Message) -> AIMessage {
    let role = if message.author.bot {
        "assistant".to_string()
    } else {
        "user".to_string()
    };
    AIMessage {
        role,
        content: message.content.clone(),
    }
}
