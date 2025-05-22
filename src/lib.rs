mod hub_system;
mod utils;

pub use anyhow::Result;
use serenity::all::StandardFramework;
pub use serenity::prelude::*;
pub use utils::*;

/// Discord bot token
pub const TOKEN: &str = include_str!("../.token");

pub async fn run() -> Result<()> {
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
        GatewayIntents::MESSAGE_CONTENT;
    // todo 占位符
    let command_system = StandardFramework::new();
    let mut client = Client::builder(TOKEN, gateway)
        .event_handler(hub_system::GuildMessagesHandler)
        .framework(command_system)
        .await?;

    if let Err(why) = client.start().await {
        log::error!("Client error: {:?}", why);
        return Err(why.into());
    }
    Ok(())
}
