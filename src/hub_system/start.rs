use crate::keys::BotDataKey;
use anyhow::Context as AnyHowContext;
use poise::async_trait;
use serenity::all::{Context, EventHandler, Ready};

pub struct StartHandler;

#[async_trait]
impl EventHandler for StartHandler {
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        log::info!("{} is connected!", data_about_bot.user.name);
        let type_map = ctx.data.read().await;
        let bot_data = type_map.get::<BotDataKey>().context("获取机器人配置出现错误").unwrap();
        let bot_status = bot_data.access();
        let activity = bot_status.bot_activity.clone();
        log::info!("{} is starting...", data_about_bot.user.name);
        log::info!("Bot ID: {}", data_about_bot.user.id);
        log::info!("bot activity is {:#?}",activity);
        ctx.set_activity(Some(activity.into()));
    }
}
