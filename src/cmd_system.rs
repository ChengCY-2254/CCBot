//! This file contains the implementation of the HubSystem struct and its associated methods.

use crate::HttpKey;
use anyhow::Context;
use chrono::FixedOffset;
use futures::{Stream, StreamExt};
use lazy_static::lazy_static;
use poise::CreateReply;
use serenity::all::{ActivityData, GuildChannel, MessageBuilder};
use serenity::async_trait;
use songbird::input::{YoutubeDl};
use songbird::{Event, EventContext, EventHandler, TrackEvent};
use std::ops::Deref;

lazy_static! {
    pub static ref UTC8: FixedOffset = FixedOffset::east_opt(8 * 3600).unwrap();
}

///用户数据
pub struct Data {}
///错误类型
type Error = anyhow::Error;
///上下文类型
type PoiseContext<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn ping(
    ctx: PoiseContext<'_>,
    #[description = "选择一个用户"] user: Option<poise::serenity_prelude::User>,
) -> crate::Result<()> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    let response = MessageBuilder::new()
        .mention(u)
        .push("创建于")
        .push_bold_safe(ToString::to_string(
            &u.created_at()
                .to_utc()
                .with_timezone(UTC8.deref())
                .format("%Y-%M-%d %H:%M:%S"),
        ))
        .build();

    ctx.say(response).await?;
    Ok(())
}

/// 注册命令的命令，需要使用@`[yourbot]` register来进行使用
#[poise::command(prefix_command, aliases("reg"))]
pub async fn register(ctx: PoiseContext<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

///机器人状态转换命令
#[poise::command(
    slash_command,
    aliases("status"),
    required_permissions = "ADMINISTRATOR"
)]
pub async fn set_status(
    ctx: PoiseContext<'_>,
    #[autocomplete = "autocomplete_activity_type"]
    #[description = "状态类型"]
    activity_type: String,
    #[description = "内容"] activity_name: String,
    #[description = "活动网址"] url: Option<String>,
) -> Result<(), Error> {
    {
        let activity_name = activity_name.clone();
        match activity_type.as_str() {
            "playing" => {
                ctx.serenity_context()
                    .set_activity(Some(ActivityData::playing(activity_name)));
            }
            "streaming" => {
                ctx.serenity_context()
                    .set_activity(Some(ActivityData::streaming(
                        activity_name,
                        url.unwrap_or_else(|| "https://ys.mihoyo.com".to_owned()),
                    )?));
            }
            "listening" => {
                ctx.serenity_context()
                    .set_activity(Some(ActivityData::listening(activity_name)));
            }
            "watching" => {
                ctx.serenity_context()
                    .set_activity(Some(ActivityData::watching(activity_name)));
            }
            _ => {
                ctx.serenity_context()
                    .set_activity(Some(ActivityData::custom(activity_name)));
            }
        };
    }
    //发送仅自己可见的消息
    let reply = CreateReply::default().ephemeral(true).content("状态已更新");
    ctx.send(reply).await?;
    Ok(())
}

/// 状态补全程序
pub async fn autocomplete_activity_type(
    _ctx: PoiseContext<'_>,
    partial: &str,
) -> impl Stream<Item = String> {
    futures::stream::iter(&["playing", "listening", "streaming", "watching", "custom"])
        .filter(move |name| futures::future::ready(name.starts_with(partial)))
        .map(|name| name.to_string())
}

/// 加入语音频道
#[poise::command(slash_command)]
pub async fn play_music(
    ctx: PoiseContext<'_>,
    #[description = "语音频道"]
    #[channel_types("Voice")]
    channel: GuildChannel,
    #[description = "播放链接，支持BiliBili"] url: String,
) -> Result<(), Error> {
    let guild_id = channel.guild_id;

    let http_client = ctx
        .serenity_context()
        .data
        .read()
        .await
        .get::<HttpKey>()
        .cloned()
        .with_context(|| "get http client failed")?;
    log::info!("http客户端获取成功");

    let manager = songbird::get(ctx.serenity_context())
        .await
        .with_context(|| "获取语音客户端失败")?
        .clone();
    log::info!("获取语音客户端成功");
    // 加入语音频道
    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        log::info!("获取语音频道成功，即将开始推流");
        let src = YoutubeDl::new(http_client, url);
        
        log::info!("获取YoutubeDl成功");

        let _ = handler.play_input(src.clone().into());

        ctx.say("开始播放".to_string()).await?;
        return Ok(());
    }
    Err(anyhow::anyhow!("出现异常，无法播放。"))
}

#[poise::command(slash_command)]
async fn join(
    ctx: PoiseContext<'_>,
    #[channel_types("Voice")] channel: GuildChannel,
) -> crate::Result<()> {
    let (guild_id, channel_id) = {
        let guild = channel.guild_id;
        let channel_id = channel.id;
        (guild, channel_id)
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .with_context(|| "语音客户端初始化中")?
        .clone();
    let handler_lock = manager.join(guild_id, channel_id).await?;
    let mut handler = handler_lock.lock().await;
    handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);
    let reply = CreateReply::default()
        .ephemeral(true)
        .content(format!("已加入语音频道: {}", channel.name));
    ctx.send(reply).await?;
    Ok(())
}

struct TrackErrorNotifier;

#[async_trait]
impl EventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                println!(
                    "Track {:?} encountered an error: {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        }

        None
    }
}

#[poise::command(slash_command)]
async fn leave(
    ctx: PoiseContext<'_>,
    #[channel_types("Voice")] channel: GuildChannel,
) -> crate::Result<()> {
    let guild_id = channel.guild_id;
    let manager = songbird::get(ctx.serenity_context())
        .await
        .with_context(|| "语音客户端初始化中")?
        .clone();

    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(why) = manager.remove(guild_id).await {
            log::error!("Error leaving channel: {:?}", why);
            ctx.say("离开语音频道失败").await?;
        };
    }

    Ok(())
}

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
            ],
            manual_cooldowns: false,
            ..Default::default()
        })
        .build();
    framework
}
