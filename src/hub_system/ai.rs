#![cfg(feature = "ai-chat")]
use crate::hub_system::model::{AIMessage, into_ai_message};
use crate::{HttpKey, read_file};
use anyhow::{Context as AnyHowContext, anyhow};
use serde::{Deserialize, Serialize};
use serenity::all::{EditMessage, GetMessages, Message, MessageBuilder, Ready};
use serenity::async_trait;
use serenity::prelude::{Context, EventHandler};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

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
    inner: DataBox<AIConfig>,
}

impl AIMessageHandler {
    pub async fn new() -> Self {
        let inner = DataBox::new(AIConfig::new().await);
        AIMessageHandler { inner }
    }
}

#[derive(Debug, Deserialize, Serialize)]
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

#[async_trait]
impl EventHandler for AIMessageHandler {
    #[allow(clippy::await_holding_refcell_ref)]
    async fn message(&self, ctx: Context, new_message: Message) {
        let user_id = new_message.author.id;
        let bot_id = ctx.cache.current_user().id;
        // 忽略自己发送的消息
        if user_id == bot_id {
            return;
        }
        //如果是@机器人

        if new_message
            .mentions
            .iter()
            .map(|mentions| mentions.id)
            .any(|id| id == ctx.cache.current_user().id)
        {
            log::info!("用户 {} 提及了机器人", new_message.author);
            log::info!("内容是 {}", new_message.content);
            // 如果是注册指令，就不处理
            if new_message.content.ends_with("reg") {
                return;
            }
            log::trace!("开始发送思考消息");

            let mut bot_message = Self::send_thinking_message(&ctx, &new_message).await;

            // 获取历史消息
            let history = Self::fetch_history(&ctx, &new_message, user_id).await;
            log::info!("已获取历史消息记录，共计 {} 条", history.len());

            let content = &new_message.content;

            // 处理消息 回复消息id
            // 在获取回复的时候，继续设置编写状态
            let response = {
                let mut interval = tokio::time::interval(Duration::from_secs(4));
                let http_client = ctx.data.read().await.get::<HttpKey>().cloned().unwrap();
                let aiconfig = self.inner.lock().await;
                log::info!("开始向服务器请求回复");
                let result = tokio::select! {
                    result = aiconfig.chat(&http_client, content, history)=>{
                        result
                    }
                    _ = async {
                        //无限循环，所以这个分支不会结束
                        loop{
                            // 广播正在思考
                            new_message.channel_id.broadcast_typing(&ctx).await.ok();
                            interval.tick().await;
                        }
                    }=>{
                        // 这里是不可达代码
                        log::info!("思考超时，正在返回消息");
                        Err(anyhow!("思考超时"))
                    }
                };
                result
            };

            match response {
                Ok(response) => {
                    //success
                    if let Err(why) = bot_message.delete(&ctx).await {
                        log::error!("Error deleting message: {:?}", why);
                    }
                    log::info!("服务器回复成功，正在返回消息");
                    let message_resp = MessageBuilder::new()
                        .mention(&new_message.author)
                        .push_bold_safe(&response)
                        .build();
                    if let Err(why) = new_message
                        .reply(&ctx, message_resp)
                        .await
                        .context("Error when sending message")
                    {
                        log::error!("Error sending message: {:?}", why);
                    }
                }
                Err(why) => {
                    bot_message
                        .edit(
                            &ctx,
                            EditMessage::new().content(format!(
                                "思考失败，请联系管理员检查机器人。原因是：{}",
                                why
                            )),
                        )
                        .await
                        .unwrap();
                    log::error!("Error when chat: {:?}", why);
                }
            }
        }
    }
    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {}
}

impl AIMessageHandler {
    async fn fetch_history(
        ctx: &Context,
        new_message: &Message,
        user_id: serenity::model::id::UserId,
    ) -> Vec<Message> {
        let select = GetMessages::new().limit(50).before(new_message.id);
        new_message
            .channel_id
            .messages(ctx, select)
            .await
            .unwrap_or_default()
            .into_iter()
            .filter(|msg| msg.author.id == user_id || msg.author.bot)
            //获取开头不为`/`的消息，也就是排除命令内容
            .filter(|msg| !msg.content.starts_with("/"))
            .collect()
    }

    async fn send_thinking_message(ctx: &Context, new_message: &Message) -> Message {
        new_message
            .channel_id
            .say(ctx, "请稍等，我正在思考...")
            .await
            .map_err(|e| anyhow!("Error sending message: {:?}", e))
            .unwrap()
    }
}

impl AIConfig {
    async fn new() -> Self {
        read_file("config/ai-config.json").unwrap()
    }
}
impl AIConfig {
    pub async fn chat(
        &self,
        http_client: &reqwest::Client,
        message: &str,
        history: Vec<Message>,
    ) -> crate::Result<String> {
        let mut messages: Vec<AIMessage> = history.iter().map(into_ai_message).collect();
        messages.push(AIMessage::new("user", message));
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
