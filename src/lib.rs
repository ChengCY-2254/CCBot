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
mod bot;
mod commands;
mod config;
mod keys;
mod shared;
pub mod utils;
#[cfg(feature = "yt-dlp")]
mod yt_dlp;

use crate::config::data_config::DataConfig;
use crate::keys::BotDataKey;
use crate::keys::HttpKey;
use crate::shared::UpSafeCell;
pub use anyhow::Result;
use config::*;
use serenity::all::UserId;
use serenity::prelude::*;
use songbird::SerenityInit;
use std::collections::HashSet;

/// 版本信息
pub const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

///错误类型
pub type Error = anyhow::Error;

///[poise::Context]的自定义上下文类型
pub type PoiseContext<'a> = poise::Context<'a, (), Error>;
/// 机器人的启动入口
pub async fn run(token: String) -> Result<()> {
    // 成员加入/离开/更新
    let gateway = GatewayIntents::GUILD_MEMBERS |
        // 封禁/解封成员
        GatewayIntents::GUILD_MODERATION |
        // 频道消息
        GatewayIntents::GUILD_MESSAGES |
        // 语音频道状态
        GatewayIntents::GUILD_VOICE_STATES |
        // 用户在线状态
        GatewayIntents::GUILD_PRESENCES |
        // 读取消息内容
        GatewayIntents::MESSAGE_CONTENT |
        // 直接消息
        GatewayIntents::DIRECT_MESSAGES;
    let http_client = reqwest::Client::new();
    let data: Data = unsafe {
        let mut data = DataConfig::new("config/data.json").map_err(|e| {
            log::error!("Error loading data: {:?}", e);
            anyhow::anyhow!("Error loading data from config/data.json because: {}", e)
        })?;
        // 初始化ai提示
        log::info!("开始初始化ai系统提示");
        data.aiconfig.init_prompt()?;
        UpSafeCell::new(data)
    };
    // 初始化命令框架
    let frame_work = { frame_work(data.access().owners.clone()) };

    let mut client = {
        #[allow(unused_mut)]
        let mut client = Client::builder(token, gateway)
            .register_songbird()
            .event_handler(bot::handlers::ManagerHandler)
            .event_handler(bot::handlers::AiHandler)
            .event_handler(bot::handlers::StartHandler)
            .event_handler(bot::handlers::ClearHandler)
            .framework(frame_work)
            .type_map_insert::<HttpKey>(http_client)
            .type_map_insert::<BotDataKey>(data);

        client.await?
    };

    if let Err(why) = client.start().await {
        log::error!("Client error: {:?}", why);
        return Err(why.into());
    }
    Ok(())
}

/// 命令行框架程序
pub fn frame_work(owners: HashSet<UserId>) -> poise::Framework<(), Error> {
    // 配置文件要在这里读取
    log::info!("create framework");
    // 导入命令行
    let mut commands = vec![];
    commands.append(&mut commands::manage_export());
    commands.append(&mut commands::general_export());
    commands.append(&mut commands::music_export());

    let framework: poise::Framework<(), Error> = poise::Framework::builder()
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                // 为每个缓存的公会注册命令
                // let guilds = ctx.cache.guilds();
                // for id in guilds {
                //     poise::builtins::register_in_guild(ctx, &framework.options().commands, id)
                //         .await?;
                // }
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(())
            })
        })
        .options(poise::FrameworkOptions {
            owners,
            commands,
            manual_cooldowns: false,
            ..Default::default()
        })
        .build();
    framework
}
