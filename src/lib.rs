mod hub_system;
mod cmd_system;
mod utils;

pub use anyhow::Result;
pub use serenity::prelude::*;
use songbird::SerenityInit;
pub use utils::*;

pub struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = reqwest::Client;
}

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
        GatewayIntents::DIRECT_MESSAGES
        ;


    let mut client =Client::builder(token,gateway)
        .event_handler(hub_system::GuildMessagesHandler)
        .framework(cmd_system::frame_work())
        .register_songbird()
        .type_map_insert::<HttpKey>(reqwest::Client::new())
        .await?;

    if let Err(why) = client.start().await {
        log::error!("Client error: {:?}", why);
        return Err(why.into());
    }
    Ok(())
}
