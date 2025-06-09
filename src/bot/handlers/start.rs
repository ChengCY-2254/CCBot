use crate::config::data_config::APP_STATE_MANAGER;
use poise::async_trait;
use serenity::all::{Context, EventHandler, Ready};

pub struct StartHandler;

#[async_trait]
impl EventHandler for StartHandler {
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        log::info!("{} is connected!", data_about_bot.user.name);
        let app_state = APP_STATE_MANAGER.get_app_state();
        let app_state = app_state.access();
        let activity = app_state.bot_activity.clone();
        log::info!("{} is starting...", data_about_bot.user.name);
        log::info!("Bot ID: {}", data_about_bot.user.id);
        log::info!("开始恢复app状态");
        log::info!("机器人活动状态已恢复为 {:#?}", activity);
        ctx.set_activity(Some(activity.into()));
        log::info!("检查是否恢复语音状态");

        let voice_channel = app_state.current_voice_channel.clone();
        tokio::spawn(async move {
            if let Some(channel) = voice_channel {
                match songbird::get(&ctx).await {
                    Some(client) => {
                        if client.join(channel.guild_id, channel.id).await.is_ok() {
                            log::info!("重新加入语音频道 {} id:{} 成功", channel.name, channel.id);
                        } else {
                            log::error!("重新加入语音频道 {} id:{} 失败", channel.name, channel.id);
                        }
                    }
                    None => {
                        log::error!("恢复语音状态失败，没有找到Songbird实例");
                    }
                }
            } else {
                log::info!("不需要恢复语音状态");
            }
        });
    }
}
