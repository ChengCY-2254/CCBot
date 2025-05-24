//! 数据模型
#![allow(dead_code)]
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
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
