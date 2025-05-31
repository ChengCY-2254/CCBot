//!这个模块用于处理消息撤回

use crate::keys::BotDataKey;
use poise::async_trait;
use serenity::all::{Context, EventHandler, Message};

/// 这个模块用于处理消息撤回，与[crate::cmd_system::manage_export]联动
pub struct ManagerHandler;

#[async_trait]
impl EventHandler for ManagerHandler {
    async fn message(&self, ctx: Context, new_message: Message) {
        if new_message.author.id == ctx.cache.current_user().id {
            return;
        }
        let channel_id = new_message.channel_id;
        let app_data_lock = ctx.data.read().await;
        if let Some(app_data) = app_data_lock.get::<BotDataKey>() {
            let channel_name = new_message.channel_id.name(&ctx).await.unwrap();
            let need_to_withdraw = app_data.access().monitored_channels.contains(&channel_id);
            if need_to_withdraw {
                log::trace!(
                    "获取到需要撤回的消息: {} 频道 {}",
                    new_message.content,
                    channel_name
                );
                if let Err(why) = new_message.delete(&ctx).await {
                    log::error!("Error deleting message: {:?}", why);
                }
                log::trace!("已删除消息: {} 频道 {}", new_message.content, channel_name);
            }
        }
    }
}
