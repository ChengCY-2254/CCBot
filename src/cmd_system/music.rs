//! # 音乐模块
//! 语音方向的内容

use crate::cmd_system::utils::get_http_client;
use crate::utils::UpSafeCell;
use crate::{ExportVec, PoiseContext};
use anyhow::Context;
use lazy_static::lazy_static;
use poise::{CreateReply, async_trait};
use reqwest::Client;
use serenity::all::{GuildChannel, MessageBuilder};
use songbird::input::YoutubeDl;
use songbird::{Event, EventContext, EventHandler, Songbird, TrackEvent};
use std::sync::Arc;

lazy_static! {
    /// 当前加入的语音频道id
    pub static ref CURRENT_JOIN_CHANNEL: UpSafeCell<Option<GuildChannel>> =
        unsafe { UpSafeCell::new(None) };
}

/// 音乐相关命令
#[poise::command(
    slash_command,
    subcommands("search_bilibili", "play", "join", "leave", "stop")
)]
pub async fn music(_ctx: PoiseContext<'_>) -> crate::Result<()> {
    Ok(())
}

/// 播放音乐或视频，可播放网站以yt-dlp支持的网站为准
#[poise::command(slash_command)]
pub async fn play(
    ctx: PoiseContext<'_>,
    #[description = "播放链接，支持BiliBili，更多网站请参见yt-dlp开源项目的支持列表"] url: String,
) -> crate::Result<()> {
    let guild_id = ctx.guild_id().context("没有在服务器中")?;
    let (http_client, manager) = get_http_and_songbird(ctx).await?;
    log::info!("获取语音客户端成功");
    // 加入语音频道
    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        log::info!("获取语音频道成功，即将开始推流");
        let src = YoutubeDl::new(http_client, url.clone());

        log::info!("获取YoutubeDl成功");
        handler.stop();
        let _ = handler.play_input(src.clone().into());

        let response = MessageBuilder::new()
            .push_bold_safe("开始播放")
            .push(&url)
            .build();

        ctx.reply(response).await?;
        return Ok(());
    }
    Err(anyhow::anyhow!("播放失败，可能没有加入语音频道"))
}

/// 从bilibili搜索音乐并播放第一个结果
#[poise::command(slash_command, rename = "play_for_bilibili")]
pub async fn search_bilibili(
    ctx: PoiseContext<'_>,
    #[description = "搜索内容"] key_word: String,
) -> crate::Result<()> {
    let guild_id = ctx.guild_id().context("没有在服务器中")?;
    let (http_client, manager) = get_http_and_songbird(ctx).await?;
    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        log::info!("获取语音频道成功，正在搜索内容");
        let (source_url, title) = {
            let mut src = YoutubeDl::new_search(http_client.clone(), key_word);
            let mut src = src.search(Some("bilisearch"), Some(5)).await?;
            let src = src.next().context("好像没有结果哦")?;
            let source_url = src.source_url.context("获取链接失败")?;
            let title = src.title.unwrap_or_default();
            log::info!("获取到标题 {title} link {source_url}");
            (source_url, title)
        };
        handler.stop();
        log::info!("停止指令发布成功");
        let _ = handler.play_input(YoutubeDl::new(http_client, source_url.clone()).into());
        log::info!("开始播放 {}", title);
        log::info!("开始响应信息");
        let response = format!("开始播放 [{title}]({source_url})");
        ctx.send(CreateReply::default().content(response)).await?;

        log::info!("响应完成");
        return Ok(());
    }

    Err(anyhow::anyhow!("播放失败，可能没有加入语音频道"))
}

async fn get_http_and_songbird(ctx: PoiseContext<'_>) -> crate::Result<(Client, Arc<Songbird>)> {
    let http_client = get_http_client(&ctx).await?;
    log::info!("http客户端获取成功");

    let manager = songbird::get(ctx.serenity_context())
        .await
        .with_context(|| "获取语音客户端失败")?
        .clone();
    log::info!("获取语音客户端成功");
    Ok((http_client, manager))
}
///加入一个语音频道
#[poise::command(slash_command, owners_only)]
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

#[async_trait]
impl EventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                log::error!(
                    "Track {:?} encountered an error: {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        }

        None
    }
}
/// 离开一个语音频道
#[poise::command(slash_command, owners_only)]
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

/// 停止播放当前音乐
#[poise::command(slash_command)]
pub async fn stop(ctx: PoiseContext<'_>) -> crate::Result<()> {
    let guild_id = ctx.guild_id().context("没有在服务器中")?;
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

pub fn music_export() -> ExportVec {
    vec![music()]
}
