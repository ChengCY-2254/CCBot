//! 语音模块

use crate::HttpKey;
use crate::cmd_system::{Error, PoiseContext};
use anyhow::Context;
use lazy_static::lazy_static;
use poise::{CreateReply, async_trait};
use serenity::all::{EmbedMessageBuilding, GuildChannel, MessageBuilder};
use songbird::input::YoutubeDl;
use songbird::{Event, EventContext, EventHandler, TrackEvent};

lazy_static! {
    ///播放状态，false为没有播放，true为播放
    pub static ref PLAY_STATE: tokio::sync::RwLock<bool> = tokio::sync::RwLock::new(false);
}

/// 播放音乐或视频，可播放网站以yt-dlp支持的网站为准
#[poise::command(slash_command)]
pub async fn play_music(
    ctx: PoiseContext<'_>,
    #[description = "播放链接，支持BiliBili，更多网站请参见yt-dlp开源项目的支持列表"] url: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().with_context(|| "没有在服务器中")?;
    {
        let state = PLAY_STATE.read().await;
        if *state {
            ctx.send(
                CreateReply::default()
                    .ephemeral(true)
                    .content("正在播放中，请稍后再试"),
            )
            .await?;
            return Ok(());
        }
    }
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
        let src = YoutubeDl::new(http_client, url.clone());

        log::info!("获取YoutubeDl成功");

        let _ = handler.play_input(src.clone().into());

        let response = MessageBuilder::new()
            .push_bold_safe("开始播放")
            .push_named_link("奶龙", &url)
            .build();

        ctx.reply(response).await?;
        return Ok(());
    }
    Err(anyhow::anyhow!("出现异常，无法播放。"))
}

///加入一个频道
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn join(
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

pub struct TrackErrorNotifier;
// /// 检查播放状态
// pub(super) struct PlayEnd;
// /// 检查播放状态
// pub(super) struct PlayStart;

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

#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn leave(
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
            return Err(anyhow::anyhow!("离开语音频道失败"));
        };
    }
    ctx.send(
        CreateReply::default()
            .ephemeral(true)
            .content(format!("已离开频道 {}", channel.name)),
    )
    .await?;
    Ok(())
}

#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn stop(ctx: PoiseContext<'_>) -> crate::Result<()> {
    let guild_id = ctx.guild_id().with_context(|| "没有在服务器中")?;
    let manager = songbird::get(ctx.serenity_context())
        .await
        .with_context(|| "语音客户端初始化中")?
        .clone();

    let cell = manager.get(guild_id).with_context(|| "找不到对应的频道")?;
    {
        let mut handler = cell.lock().await;
        handler.stop();
        let response = CreateReply::default().content("已停止播放").ephemeral(true);
        ctx.send(response).await?;
    }

    Ok(())
}
