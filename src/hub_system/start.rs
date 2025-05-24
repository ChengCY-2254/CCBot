use poise::async_trait;
use serenity::all::{ActivityData, Context, EventHandler, Ready};

pub struct StartHandler;

#[async_trait]
impl EventHandler for StartHandler {
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        ctx.set_activity(Some(ActivityData::playing("RustRover")));
        log::info!("{} is connected!", data_about_bot.user.name);
        log::info!("Bot ID: {}", data_about_bot.user.id);
    }
}
