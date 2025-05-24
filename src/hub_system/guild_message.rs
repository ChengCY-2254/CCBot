//! 这里编写消息控制子系统

use crate::*;
use serenity::all::{ActivityData, CreateMessage, Message, MessageBuilder, Ready, ResumedEvent};
use serenity::async_trait;

/// 消息处理子系统
pub struct GuildMessageHandler;

#[async_trait]
impl EventHandler for GuildMessageHandler {
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
