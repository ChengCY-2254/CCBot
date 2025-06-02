//! 清除dm消息

use poise::async_trait;
use serenity::all::{Context, Message};
use serenity::builder::{CreateMessage, GetMessages};
use serenity::prelude::EventHandler;

pub struct ClearHandler;

#[async_trait]
impl EventHandler for ClearHandler {
    async fn message(&self, ctx: Context, new_message: Message) {
        //排除机器人自己的消息
        if ctx.cache.current_user().id == new_message.author.id {
            return;
        }
        //判定如果是消息
        if !new_message.content.starts_with("!clear") {
            return;
        }

        //获取参数
        // !clear 5
        let args = new_message.content.split_whitespace().collect::<Vec<_>>();
        if args.len() == 2 {
            let count = args[1].parse::<u8>().unwrap();
            if count != 0 {
                let messages = new_message
                    .channel_id
                    .messages(&ctx, GetMessages::new().before(new_message.id).limit(count))
                    .await;
                if let Ok(messages) = messages {
                    if messages.is_empty() {
                        log::info!("用户发出了清除消息，但没有找到历史消息可清除");
                        return;
                    }
                    let mut delete_count = 0u8;
                    log::info!("正在删除消息");
                    #[allow(deprecated)]
                    let messages = messages.into_iter().filter(|m| m.author.bot).map(|m| m.id);
                    for id in messages {
                        if new_message
                            .channel_id
                            .delete_message(&ctx, id)
                            .await
                            .is_ok()
                        {
                            delete_count += 1;
                        }
                    }
                    new_message
                        .channel_id
                        .send_message(
                            &ctx,
                            CreateMessage::new().content(format!("已删除 {} 条消息", delete_count)),
                        )
                        .await
                        .expect("无法向频道发送消息内容");
                    log::info!("已删除 {} 条消息", delete_count);
                }
            }
        }
    }
}
