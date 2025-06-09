//!这个模块用于处理消息撤回

use crate::config::data_config::APP_STATE_MANAGER;
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
        let app_state = APP_STATE_MANAGER.get_app_state();
        let channel_name = new_message.channel_id.name(&ctx).await.unwrap();
        //如果在监控的频道中
        let need_to_withdraw = app_state.access().monitored_channels.contains(&channel_id);
        if need_to_withdraw {
            log::trace!(
                "获取到需要撤回的消息: {} 频道 {}",
                new_message.content,
                channel_name
            );
            //删除消息
            if let Err(why) = new_message.delete(&ctx).await {
                log::error!("Error deleting message: {:?}", why);
            }
            log::trace!("已删除消息: {} 频道 {}", new_message.content, channel_name);
        }
    }
}
