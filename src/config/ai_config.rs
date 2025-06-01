//! AI对话配置文件
use crate::config::{
    AIMessage, NO_AT_PROMPT_MESSAGE, SYSTEM_PROMPT_CACHE, SYS_USER_PTOMPT_MESSAGE,
};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serenity::all::Message;
use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::vec::IntoIter;

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
    /// 初始化系统提示
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
        history: IntoIter<Message>,
        is_private_chat: bool,
    ) -> crate::Result<String> {
        // 插入时间戳消息提示
        let mut messages: VecDeque<AIMessage> = history.map(AIMessage::map_ai_message).collect();
        messages.push_front(AIMessage::new(
            "system",
            SYSTEM_PROMPT_CACHE.access().clone().as_str(),
        ));
        // 如果是私聊，就插入一个提示
        if is_private_chat {
            messages.push_back(NO_AT_PROMPT_MESSAGE.clone());
        }
        if self.enable_thinking {
            messages.push_back(AIMessage::new("assistant", "思考中..."));
        }
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
