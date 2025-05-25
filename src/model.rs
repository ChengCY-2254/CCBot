use crate::{UpSafeCell, read_file};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serenity::all::{ChannelId, Message};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;

/// 机器人需要保存的配置
pub type Data = UpSafeCell<DataInner>;

/// [crate::macros::add_sub_mod]所使用的导出类型
pub type ExportVec = Vec<poise::Command<(), Error>>;

lazy_static! {
    pub static ref SYS_MESSAGE: AIMessage = AIMessage::new("system", include_str!("../sys.prompt"));
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
/// 用户数据
/// todo 要将其序列化到磁盘上存储
/// md又多了一个配置文件，这个还要反复读写
/// 明天的任务是把这帮配置文件都移动到一个文件夹中
/// 眼不见心不烦
/// 需要在创建时检查是否有配置文件夹
/// 如果没有，放出示例配置文件，然后退出。
/// 如果有，那就进入服务状态
pub struct DataInner {
    /// 需要监控的频道ID，进了这个set的频道发送消息后都会撤回。
    pub monitored_channels: HashSet<ChannelId>,
    /// ai配置
    pub aiconfig: AIConfig,
}
///错误类型
pub type Error = anyhow::Error;
///上下文类型
pub type PoiseContext<'a> = poise::Context<'a, (), Error>;

impl DataInner {
    /// 添加一个需要监控的频道
    pub fn add_monitored_channel(&mut self, channel_id: ChannelId) {
        self.monitored_channels.insert(channel_id);
    }
    /// 删除一个需要监控的频道
    pub fn remove_monitored_channel(&mut self, channel_id: ChannelId) {
        self.monitored_channels.remove(&channel_id);
    }
}

impl DataInner {
    /// 给定一个路径，读取数据文件并返回数据
    pub fn new(path: impl AsRef<Path>) -> crate::Result<DataInner> {
        read_file(path)
    }
    /// 保存数据文件
    /// path 为配置文件夹
    pub fn save(&self, config_dir_path: impl AsRef<Path>) -> crate::Result<()> {
        let path = config_dir_path.as_ref();
        if !path.exists() {
            std::fs::create_dir_all(path)?;
        }
        let file_path = path.join("data.json");
        let file = std::fs::File::create(file_path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// AI配置
pub struct AIConfig {
    token: String,
    /// 模型
    model: String,
    /// 请求路径
    url: String,
    max_tokens: u32,
    enable_thinking: bool,
    thinking_budget: u32,
    min_p: f32,
    temperature: f32,
    top_p: f32,
    top_k: f32,
    frequency_penalty: f32,
    n: i8,
    response_format: HashMap<String, String>,
}

// impl AIConfig {
//     /// 创建一个新的AI配置
//     pub async fn new() -> Self {
//         read_file("config/ai-config.json").unwrap()
//     }
// }
impl AIConfig {
    /// 聊天接口调用
    pub async fn chat(
        &self,
        http_client: &reqwest::Client,
        message: &str,
        history: &[Message],
    ) -> crate::Result<String> {
        let mut messages: VecDeque<AIMessage> = history.iter().map(into_ai_message).collect();
        messages.push_front(SYS_MESSAGE.clone());
        messages.push_back(AIMessage::new("user", message));
        let response = http_client
            .post(&self.url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model": self.model,
                "messages": messages,
                "max_tokens": self.max_tokens,
                "temperature": self.temperature,
                "top_p": self.top_p,
                "top_k": self.top_k,
                "frequency_penalty": self.frequency_penalty,
                "n": self.n,
            }))
            .send()
            .await?;

        if response.status().is_success() {
            let response_text = response.text().await?;
            let json_response: serde_json::Value = serde_json::from_str(&response_text)?;
            if let Some(content) = json_response["choices"][0]["message"]["content"].as_str() {
                Ok(content.to_string())
            } else {
                Err(anyhow::anyhow!("Invalid response format"))
            }
        } else {
            Err(anyhow::anyhow!(
                "Request failed with status: {}",
                response.status()
            ))
        }
    }
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
}

/// 将serenity的Message转换为AIMessage
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
