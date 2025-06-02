//! 清除dm消息

use poise::async_trait;
use serenity::all::{Context, Message};
use serenity::builder::GetMessages;
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
        if new_message.content.starts_with("!clear") {
            //获取参数
            // !clear 5
            let args = new_message.content.split_whitespace().collect::<Vec<_>>();
            if args.len() == 2 {
                let count = args[1].parse::<u8>();
                if count.is_ok() {
                    let count = count.unwrap();
                    if count > 0 {
                        let messages = new_message
                            .channel_id
                            .messages(&ctx, GetMessages::new().before(new_message.id).limit(count))
                            .await;
                        if messages.is_ok() {
                            let messages = messages.unwrap();
                            if !messages.is_empty() {
                                log::info!("正在删除消息");
                                #[allow(deprecated)]
                                let messages = messages
                                    .into_iter()
                                    .filter(|m| m.author.bot)
                                    .map(|m| m.id)
                                    .collect::<Vec<_>>();
                                new_message
                                    .channel_id
                                    .delete_messages(&ctx, messages)
                                    .await
                                    .expect("无法删除消息");
                            }
                        }
                    }
                }
            }
        }
    }
}
