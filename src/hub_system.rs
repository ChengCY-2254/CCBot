//! 这里编写消息控制子系统

use crate::*;
use anyhow::Context as AnyHowContext;
use serde::{Deserialize, Serialize};
use serenity::all::{ActivityData, CreateMessage, Message, MessageBuilder, Ready, ResumedEvent};
use serenity::async_trait;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

/// 消息处理子系统
pub struct GuildMessagesHandler;

#[async_trait]
impl EventHandler for GuildMessagesHandler {
    async fn message(&self, ctx: Context, new_message: Message) {
        // 忽略自己发送的消息
        if new_message.author.id == ctx.cache.current_user().id {
            return;
        }
        let content = &new_message.content;
        if content == "!ping" {
            match new_message.channel_id.to_channel(&ctx).await {
                Ok(channel) => {
                    if let Some(channel) = channel.guild() {
                        log::info!(
                            "get a message from Channel name: {}, message: {}",
                            channel.name,
                            new_message.content
                        );
                        let response = MessageBuilder::new()
                            .push("User ")
                            .push_bold_safe("used the 'ping' command in the ")
                            //提及频道
                            .mention(&channel)
                            .push(" channel")
                            .build();

                        if let Err(why) = new_message.channel_id.say(&ctx.http, response).await {
                            log::error!("Error sending message: {:?}", why);
                        }
                    }
                }
                Err(why) => {
                    log::error!("Error getting channel: {:?}", why);
                }
            }
            // // 处理消息 回复消息id
            // if let Err(why) = new_message.channel_id.say(&ctx.http, "Hello, world!").await {
            //     log::error!("Error sending message: {:?}", why);
            // }
        } else if content == "!messageme" {
            // 直接向用户回复
            let builder = CreateMessage::new().content("Hello!!!");
            if let Err(why) = new_message.author.dm(&ctx, builder).await {
                log::error!("Error when direct messagin user: {:?}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        ctx.set_activity(Some(ActivityData::playing("RustRover")));
        log::info!("{} is connected!", data_about_bot.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        log::info!("Resumed");
    }
}

#[derive(Debug, Default)]
struct DataBox<T>(Arc<Mutex<T>>);
impl<T> DataBox<T> {
    fn new(data: T) -> Self {
        DataBox(Arc::new(Mutex::new(data)))
    }
}
unsafe impl<T> Send for DataBox<T> {}
unsafe impl<T> Sync for DataBox<T> {}

impl<T> Deref for DataBox<T> {
    type Target = Arc<Mutex<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct AIMessageHandler {
    client: reqwest::Client,
    inner: DataBox<AIConfig>,
}

unsafe impl Sync for AIMessageHandler {}

impl AIMessageHandler {
    pub async fn new(client: reqwest::Client) -> Self {
        let inner = DataBox::new(AIConfig::new().await);
        AIMessageHandler { client, inner }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AIConfig {
    token: String,
    /// 模型
    model: String,
    /// 请求路径
    url: String,
    max_token: u32,
    enable_thinking: bool,
    thinking_budget: u32,
    min_p: i8,
    temperature: i8,
    top_p: i8,
    top_k: i8,
    frequency_penalty: i8,
    n: i8,
    response_format: HashMap<String, String>,
}

#[async_trait]
impl EventHandler for AIMessageHandler {
    #[allow(clippy::await_holding_refcell_ref)]
    async fn message(&self, ctx: Context, new_message: Message) {
        // 忽略自己发送的消息
        if new_message.author.id == ctx.cache.current_user().id {
            return;
        }
        //如果是@机器人

        if new_message
            .mentions
            .iter()
            .map(|mentions| mentions.id)
            .any(|id| id == ctx.cache.current_user().id)
        {
            let content = &new_message.content;

            // 处理消息 回复消息id
            let response = {
                let aiconfig = self.inner.lock().await;
                aiconfig
                    .chat(&self.client, content)
                    .await
                    .context("Error when chat")
            };

            match response {
                Ok(response) => {
                    let message_resp = MessageBuilder::new()
                        .mention(&new_message.author)
                        .push_bold_safe(&response)
                        .build();
                    if let Err(why) = new_message
                        .reply(ctx, message_resp)
                        .await
                        .context("Error when sending message")
                    {
                        log::error!("Error sending message: {:?}", why);
                    }
                }
                Err(why) => {
                    log::error!("Error when chat: {:?}", why);
                }
            }
        }
    }
    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {}
}

impl AIConfig {
    async fn new() -> Self {
        read_file(".config.json")
            .await
            .unwrap()
    }
}
impl AIConfig {
    #[allow(dead_code)]
    pub fn save(&self) -> Result<()> {
        let file = std::fs::File::create(".config")?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }

    pub async fn chat(&self, http_client: &reqwest::Client, message: &str) -> Result<String> {
        let response = http_client
            .post(&self.url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model": self.model,
                "messages": [
                    {
                        "role": "user",
                        "content": message
                    }
                ],
                "max_tokens": self.max_token,
                "temperature": self.temperature,
                "top_p": self.top_p,
                "top_k": self.top_k,
                "frequency_penalty": self.frequency_penalty,
                "n": self.n,
            }))
            .send()
            .await?;

        if response.status().is_success() {
            let response_text = response.text().await.unwrap();
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
