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
        log::info!("用户发出了清除消息命令，正在解析命令参数");

        //获取参数
        // !clear 5
        let args = new_message.content.split_whitespace().collect::<Vec<_>>();
        log::info!( "用户发送的清除消息命令参数为：{}", args[1]);
        if args.len() != 2 {
            log::info!("用户发送的清除消息命令参数不正确");
            return;
        }
        if args.len() == 2 {
            let count = args[1].parse::<u8>().unwrap();
            if count > 100 {
                log::info!("用户发送的清除消息命令参数不正确");
                new_message
                    .channel_id
                    .send_message(ctx, CreateMessage::new().content("参数错误，必须在100以下"))
                    .await
                    .expect("无法向频道发送消息");
                return;
            }
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
                    log::info!("已删除 {} 条消息", delete_count);
                }
            }
        }
    }
}
