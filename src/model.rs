use crate::{UpSafeCell, read_file};
use anyhow::anyhow;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serenity::all::{ChannelId, Message};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};

/// 机器人需要保存的配置
pub type Data = UpSafeCell<DataInner>;

/// [crate::macros::add_sub_mod]所使用的导出类型
pub type ExportVec = Vec<poise::Command<(), Error>>;

lazy_static! {
    pub static ref SYS_USER_PTOMPT_MESSAGE: AIMessage =
        AIMessage::new("system", "以下是用户的最新输入");
    /// 在这里缓存住系统提示
    pub static ref SYSTEM_PROMPT_CACHE: UpSafeCell<String> = unsafe {UpSafeCell::new(String::new())};
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
/// 用户数据
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
    /// 系统提示的路径
    system_prompt_file: PathBuf,
}

impl AIConfig {
    pub fn init_prompt(&mut self) -> crate::Result<()> {
        if let Some(file) = self.system_prompt_file.file_name() {
            let file = file.to_str().unwrap().to_string();
            self.use_others_prompt(&file)?;
            log::info!("提示 {} 加载成功", file);
        }
        Ok(())
    }
    /// 聊天接口调用
    pub async fn chat(
        &self,
        http_client: &reqwest::Client,
        message: &str,
        history: &[Message],
    ) -> crate::Result<String> {
        // 插入时间戳消息提示
        let mut messages: VecDeque<AIMessage> = history.iter().map(into_ai_message).collect();
        messages.push_front(AIMessage::new(
            "system",
            SYSTEM_PROMPT_CACHE.access().clone().as_str(),
        ));
        messages.push_back(SYS_USER_PTOMPT_MESSAGE.clone());
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
    /// 第一个是文件名，第二个是内容
    pub fn get_system_prompt(&self) -> crate::Result<(String, String)> {
        let file_name = self
            .system_prompt_file
            .as_path()
            .file_name()
            .map(|os_str| os_str.to_string_lossy().into_owned());
        let prompt_content = SYSTEM_PROMPT_CACHE.access().clone();
        if let Some(file_name) = file_name {
            Ok((file_name, prompt_content))
        } else {
            Err(anyhow!("未设置系统提示！"))
        }
    }
    /// 切换成其它系统提示，并重新读取内容。
    pub fn use_others_prompt(&mut self, file_name: &str) -> crate::Result<()> {
        self.system_prompt_file = PathBuf::new().join(format!("config/{}", file_name));
        let sys_prompt_content = std::fs::read_to_string(self.system_prompt_file.clone())?;
        let mut prompt = SYSTEM_PROMPT_CACHE.exclusive_access();
        prompt.clear();
        prompt.push_str(&sys_prompt_content);
        prompt.shrink_to_fit();
        Ok(())
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
    let message_id = message.id;
    let message_author_id = message.author.id;
    let message_times_map = message.timestamp;
    let role = if message.author.bot {
        "assistant".to_string()
    } else {
        "user".to_string()
    };

    let content = format!(
        "SYSTEM：以下内容为补充参考信息\r\n```message_id:{message_id}\r\nmessage_author_id:{message_author_id}\r\nmessage_times_map:{message_times_map}\r\n```\
        SYSTEM:补充说明结束，以下是用户内容\r\n\
        {}",
        message.content
    );
    AIMessage { role, content }
}
