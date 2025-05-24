use crate::HttpKey;
use crate::keys::BotDataKey;
use anyhow::{Context as AnyHowContext, anyhow};
use serenity::all::{
    ButtonStyle, CreateActionRow, CreateButton, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, EditMessage, GetMessages, Message, Ready,
};
use serenity::async_trait;
use serenity::prelude::{CacheHttp, Context, EventHandler};
use std::time::Duration;

#[derive(Debug)]
pub struct AiHandler;

#[async_trait]
impl EventHandler for AiHandler {
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
            let response = { Self::request_ai_reply(&ctx, &new_message, &history, content).await };

            match response {
                Ok(response) => {
                    //success
                    if let Err(why) = bot_message.delete(&ctx).await {
                        log::error!("Error deleting message: {:?}", why);
                    }
                    log::info!("服务器回复成功，正在返回消息");
                    let response = format!("<@{}> {}", new_message.author.id, response);

                    let components = CreateActionRow::Buttons(vec![
                        CreateButton::new("re.generate")
                            .label("重新生成")
                            .style(ButtonStyle::Primary)
                            .emoji('🔁'),
                    ]);

                    let message_resp = CreateMessage::new()
                        .components(vec![components])
                        .content(response);

                    let interaction = ctx
                        .http()
                        .send_message(new_message.channel_id, vec![], &message_resp)
                        .await
                        .unwrap()
                        .await_component_interaction(&ctx.shard)
                        .timeout(Duration::from_secs(60 * 5))
                        .author_id(new_message.author.id)
                        .await;
                    if let Some(i) = interaction {
                        if i.data.custom_id == "re.generate" {
                            i.create_response(
                                &ctx,
                                CreateInteractionResponse::UpdateMessage(
                                    CreateInteractionResponseMessage::default().content(
                                        Self::request_ai_reply(
                                            &ctx,
                                            &new_message,
                                            &history,
                                            &new_message.content,
                                        )
                                        .await
                                        .unwrap(),
                                    ),
                                ),
                            )
                            .await
                            .unwrap()
                        }
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

impl AiHandler {
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

    async fn request_ai_reply(
        ctx: &Context,
        new_message: &Message,
        history: &[Message],
        content: &str,
    ) -> crate::Result<String> {
        let mut interval = tokio::time::interval(Duration::from_secs(4));
        let http_client = ctx.data.read().await.get::<HttpKey>().cloned().unwrap();
        let aiconfig = {
            ctx.data
                .read()
                .await
                .get::<BotDataKey>()
                .context("获取Bot配置文件出现异常")?
                .access()
                .aiconfig
                .clone()
        };
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
    }
}
