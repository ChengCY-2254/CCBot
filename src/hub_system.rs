//! 这里编写消息控制子系统
use crate::*;
use serenity::all::{
    Activity, ActivityData, CommandPermissions, CreateMessage, Message, MessagePollVoteAddEvent,
    Ready,
};
use serenity::async_trait;

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
            // 处理消息 回复消息id
            if let Err(why) = new_message.channel_id.say(&ctx.http, "Hello, world!").await {
                log::error!("Error sending message: {:?}", why);
            }
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
}
