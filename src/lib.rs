mod cmd_system;
mod hub_system;
mod keys;
mod utils;

use crate::cmd_system::{Data, Error, join, leave, ping, play_music, register, set_status, stop};
pub use crate::keys::HttpKey;
pub use anyhow::Result;
pub use serenity::prelude::*;
use songbird::SerenityInit;
pub use utils::*;

pub async fn run(token: String) -> Result<()> {
    // 成员加入/离开/更新
    let gateway = GatewayIntents::GUILD_MEMBERS |
        // 封禁/解封成员
        GatewayIntents::GUILD_MODERATION |
        // 频道消息
        GatewayIntents::GUILD_MESSAGES |
        // 语音频道状态
        GatewayIntents::GUILD_VOICE_STATES|
        // 用户在线状态
        GatewayIntents::GUILD_PRESENCES|
        // 读取消息内容
        GatewayIntents::MESSAGE_CONTENT|
        // 直接消息
        GatewayIntents::DIRECT_MESSAGES;
    let client = reqwest::Client::new();
    let mut client = Client::builder(token, gateway)
        .event_handler(hub_system::GuildMessagesHandler)
        .event_handler(hub_system::AIMessageHandler::new().await)
        .framework(frame_work())
        .register_songbird()
        .type_map_insert::<HttpKey>(client)
        .await?;

    if let Err(why) = client.start().await {
        log::error!("Client error: {:?}", why);
        return Err(why.into());
    }
    Ok(())
}

/// 命令行框架程序
pub fn frame_work() -> poise::Framework<Data, Error> {
    log::info!("create framework");
    let framework: poise::Framework<Data, anyhow::Error> = poise::Framework::builder()
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(poise::FrameworkOptions {
            commands: vec![
                ping(),
                register(),
                set_status(),
                play_music(),
                join(),
                leave(),
                stop(),
            ],
            manual_cooldowns: false,
            ..Default::default()
        })
        .build();
    framework
}
