#![deny(
    unused_imports,
    unused_variables,
    unused_parens,
    unused_qualifications,
    missing_docs
)]
//! # 定义CCBot的🤖相关
#[macro_use]
mod macros;
mod cmd_system;
mod hub_system;
mod keys;
mod model;
mod utils;

use crate::keys::BotDataKey;
pub use crate::keys::HttpKey;
pub use anyhow::Result;
pub use model::*;
pub use serenity::prelude::*;
use songbird::SerenityInit;
pub use utils::*;

/// 机器人的启动入口
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
    let http_client = reqwest::Client::new();
    let data = unsafe {
        let data = DataInner::new("config/data.json").map_err(|e| {
            log::error!("Error loading data: {:?}", e);
            anyhow::anyhow!("Error loading data from config/data.json because: {}", e)
        })?;
        UpSafeCell::new(data)
    };

    let mut client = {
        #[allow(unused_mut)]
        let mut client = Client::builder(token, gateway)
            .event_handler(hub_system::GuildMessageHandler)
            .event_handler(hub_system::ManagerHandler)
            .framework(frame_work())
            .register_songbird()
            .type_map_insert::<HttpKey>(http_client)
            .type_map_insert::<BotDataKey>(data);
        #[cfg(feature = "ai-chat")]
        #[allow(unused_mut)]
        let mut client = {
            let ai_handler = hub_system::AiHandler::new().await;
            client.event_handler(ai_handler)
        };

        client.await?
    };

    if let Err(why) = client.start().await {
        log::error!("Client error: {:?}", why);
        return Err(why.into());
    }
    Ok(())
}

/// 命令行框架程序
pub fn frame_work() -> poise::Framework<(), Error> {
    use crate::cmd_system;
    // 配置文件要在这里读取
    log::info!("create framework");
    // 导入命令行
    let mut commands = vec![];
    commands.append(&mut cmd_system::manage_export());
    commands.append(&mut cmd_system::general_export());
    commands.append(&mut cmd_system::music_export());

    let framework: poise::Framework<(), anyhow::Error> = poise::Framework::builder()
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                // 为每个缓存的公会注册命令
                let guilds = ctx.cache.guilds();
                for id in guilds {
                    poise::builtins::register_in_guild(ctx, &framework.options().commands, id)
                        .await?;
                }
                Ok(())
            })
        })
        .options(poise::FrameworkOptions {
            commands,
            manual_cooldowns: false,
            ..Default::default()
        })
        .build();
    framework
}
