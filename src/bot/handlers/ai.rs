use crate::HttpKey;
use crate::config::data_config::APP_STATE_MANAGER;
use anyhow::anyhow;
use serenity::all::{EditMessage, GetMessages, Message};
use serenity::async_trait;
use serenity::prelude::{Context, EventHandler};
use std::time::Duration;
use std::vec::IntoIter;

#[derive(Debug)]
pub struct AiHandler;

#[async_trait]
impl EventHandler for AiHandler {
    #[allow(clippy::await_holding_refcell_ref)]
    async fn message(&self, ctx: Context, new_message: Message) {
        // 判断是否是私聊
        #[allow(deprecated)]
        let is_private_message = new_message.is_private();

        if is_private_message {
            Self::private_chat_handler(&ctx, &new_message)
                .await
                .unwrap();
        } else {
            Self::channel_message_handler(&ctx, &new_message)
                .await
                .unwrap();
        }
    }
}

impl AiHandler {
    async fn fetch_history(
        ctx: &Context,
        new_message: &Message,
        _user_id: serenity::model::id::UserId,
    ) -> Vec<Message> {
        let select = GetMessages::new().limit(50).before(new_message.id);
        new_message
            .channel_id
            .messages(ctx, select)
            .await
            .unwrap_or_default()
            .into_iter()
            // .filter(|msg| msg.author.id == user_id || msg.author.bot)
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

    async fn request_ai_reply(
        ctx: &Context,
        new_message: &Message,
        history: IntoIter<Message>,
        content: &str,
        is_private_chat: bool,
    ) -> crate::Result<String> {
        log::info!(
            "频道 {} 获取到的消息历史 {:?}",
            new_message.channel_id,
            history
        );
        let mut interval = tokio::time::interval(Duration::from_secs(4));
        let http_client = ctx.data.read().await.get::<HttpKey>().cloned().unwrap();
        let aiconfig = { APP_STATE_MANAGER.get_app_state().access().aiconfig.clone() };
        log::info!("开始向服务器请求回复");
        let result = tokio::select! {
            result = aiconfig.chat(&http_client, content, history,is_private_chat)=>{
                result
            }
            _ = async {
                //无限循环，所以这个分支不会结束
                loop{
                    // 广播正在思考
                    new_message.channel_id.broadcast_typing(&ctx).await.ok();
                    // 每隔4秒发送一次思考消息
                    log::info!("正在广播思考... 等待服务器返回");
                    interval.tick().await;
                }
            }=>{
                // 这里是不可达代码
                log::info!("思考超时，正在返回消息");
                Err(anyhow!("思考超时"))
            }
        };
        result
    }

    async fn send_reply(ctx: &Context, new_message: &Message, response: String) {
        new_message
            .reply(ctx, response)
            .await
            .map_err(|why| {
                log::error!("出现了错误，请联系管理员 {}", why);
                anyhow!("出现了错误，请联系管理员 {}", why)
            })
            .unwrap();
    }
    ///在频道里聊天
    async fn channel_message_handler(ctx: &Context, new_message: &Message) -> crate::Result<()> {
        let user_id = new_message.author.id;
        let bot_id = ctx.cache.current_user().id;
        // 忽略自己发送的消息
        if user_id == bot_id {
            return Ok(());
        }
        //如果是@机器人

        if new_message
            .mentions
            .iter()
            .any(|mentions| mentions.id == bot_id)
        {
            log::info!("用户 {} 提及了机器人", new_message.author);
            log::info!("内容是 {}", new_message.content);
            // 如果是注册指令，就不处理
            if new_message.content.ends_with("reg") || new_message.content.starts_with("!") {
                return Ok(());
            }
            log::trace!("开始发送思考消息");

            let mut bot_message = Self::send_thinking_message(ctx, new_message).await;

            // 获取历史消息
            let history = Self::fetch_history(ctx, new_message, user_id).await;
            log::info!("已获取历史消息记录，共计 {} 条", history.len());

            let content = &new_message.content;

            #[allow(deprecated)]
            let is_private_chat = new_message.is_private();

            // 处理消息 回复消息id
            // 在获取回复的时候，继续设置编写状态
            let response = {
                Self::request_ai_reply(
                    ctx,
                    new_message,
                    history.into_iter(),
                    content,
                    is_private_chat,
                )
                .await
            };

            match response {
                Ok(response) => {
                    //success
                    if let Err(why) = bot_message.delete(&ctx).await {
                        log::error!("Error deleting message: {:?}", why);
                    }
                    log::info!("服务器回复成功，正在返回消息");
                    // 检查文本长度，分段回复。
                    // let raw_response = format!("<@{}> {}", new_message.author.id, response);
                    let raw_response = response;

                    Self::send_ai_response(ctx, new_message, raw_response).await;
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
                        .await?;
                    log::error!("Error when chat: {:?}", why);
                }
            }
        }
        Ok(())
    }

    async fn send_ai_response(ctx: &Context, new_message: &Message, ai_response: String) {
        if ai_response.len() > 1500 {
            // 对原始相应开始分块
            let chunks = ai_response.chars().collect::<Vec<char>>();
            // 看岔了，这是Vec，不是Chunks，所以要处以1500得到消息分块数。
            let chunk_count = chunks.len() / 1500;
            // 啊啊啊，我是傻叉，怎么会有正常人从0开始数数的啊啊啊
            log::info!("开始分块回复，共计 {}块", chunk_count + 1);
            let chunks_iter = chunks.chunks(1500).enumerate();
            for (chunk_id, chunk) in chunks_iter {
                log::info!("正在发送第 {} 块", chunk_id + 1);
                let chunk_str = if chunk_id == 0 {
                    chunk.iter().collect::<String>()
                } else {
                    format!(
                        "<@{}> {}",
                        new_message.author.id,
                        chunk.iter().collect::<String>()
                    )
                };

                Self::send_reply(ctx, new_message, chunk_str).await;
            }
            log::info!("分块消息发送成功");
        } else {
            log::info!("不需要分块，开始返回消息");
            Self::send_reply(ctx, new_message, ai_response).await;
        }
    }

    /// 用户私聊
    async fn private_chat_handler(ctx: &Context, new_message: &Message) -> crate::Result<()> {
        let user_id = new_message.author.id;
        let bot_id = ctx.cache.current_user().id;
        if user_id == bot_id {
            return Ok(());
        }
        let content = &new_message.content;
        //排除掉命令
        if content.ends_with("reg") || content.starts_with("!") {
            return Ok(());
        }
        log::info!("用户 {} 正在与机器人私聊", new_message.author);
        // 发送思考状态
        log::info!("开始发送思考状态");
        let mut bot_message = Self::send_thinking_message(ctx, new_message).await;
        // 获取历史消息
        let history = Self::fetch_history(ctx, new_message, user_id).await;
        #[allow(deprecated)]
        let is_private_chat = new_message.is_private();
        log::info!("已获取私聊历史消息记录，共计 {} 条", history.len());
        let response = Self::request_ai_reply(
            ctx,
            new_message,
            history.into_iter(),
            content,
            is_private_chat,
        )
        .await;
        match response {
            Ok(response) => {
                if let Err(why) = bot_message.delete(ctx).await {
                    log::error!("error deleting message: {:?}", why);
                }
                log::info!("服务器回复成功，正在返回消息");
                Self::send_ai_response(ctx, new_message, response).await
            }
            Err(why) => {
                bot_message
                    .edit(
                        &ctx,
                        EditMessage::new()
                            .content(format!("思考失败，请联系管理员检查机器人。原因是：{}", why)),
                    )
                    .await?;
                log::error!("Error when chat: {:?}", why);
            }
        }
        Ok(())
    }
}
