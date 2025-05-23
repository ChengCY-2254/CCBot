//! 数据模型

use serde::Serialize;
use serenity::all::Message;

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

pub fn into_ai_message(message: &Message) -> AIMessage {
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
