use poise::async_trait;
use serenity::all::{Context, EventHandler, Ready};
use crate::config::data_config::APP_STATE_MANAGER;

pub struct StartHandler;

#[async_trait]
impl EventHandler for StartHandler {
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        log::info!("{} is connected!", data_about_bot.user.name);
        let app_state = APP_STATE_MANAGER.get_app_state();
        let bot_status = app_state.access();
        let activity = bot_status.bot_activity.clone();
        log::info!("{} is starting...", data_about_bot.user.name);
        log::info!("Bot ID: {}", data_about_bot.user.id);
        log::info!("bot activity is {:#?}",activity);
        ctx.set_activity(Some(activity.into()));
    }
}
