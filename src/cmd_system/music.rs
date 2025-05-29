//! # 音乐模块
//! 语音方向的内容

use crate::cmd_system::utils::get_http_client;
use crate::utils::UpSafeCell;
use crate::{ExportVec, PoiseContext};
use anyhow::{Context, anyhow};
use lazy_static::lazy_static;
use poise::{CreateReply, async_trait};
use reqwest::Client;
use serenity::all::{EditChannel, GuildChannel};
use songbird::input::{Compose, YoutubeDl};
use songbird::{Event, EventContext, EventHandler, Songbird, TrackEvent};
use std::sync::Arc;

lazy_static! {
    /// 当前加入的语音频道id
     static ref CURRENT_JOIN_CHANNEL: UpSafeCell<Option<GuildChannel>> =
        unsafe { UpSafeCell::new(None) };
}

/// 音乐相关命令
#[poise::command(slash_command, subcommands("play", "join", "leave", "stop"))]
pub async fn music(_ctx: PoiseContext<'_>) -> crate::Result<()> {
    Ok(())
}

/// 播放音乐，支持列表请查看yt-dlp的支持网站。
#[poise::command(slash_command, rename = "play")]
pub async fn play(
    ctx: PoiseContext<'_>,
    #[description = "[关键词|AV|BV]定位B站资源|直接链接]"] text: String,
) -> crate::Result<()> {
    let guild_id = ctx.guild_id().context("没有在服务器中")?;
    let (http_client, manager) = get_http_and_songbird(ctx).await?;

    ctx.defer()
        .await
        .map_err(|why| anyhow!("延迟响应时发生错误 {why}"))?;

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        log::info!("获取语音频道成功，正在搜索内容");
        let (source_url, title) = if text.starts_with("https://")|| text.starts_with("http://") {
            let data = YoutubeDl::new(http_client.clone(), text)
                .aux_metadata()
                .await?;
            let source_url = data.source_url.unwrap_or("获取链接失败".into());
            let title = data.title.unwrap_or("原神".into());
            log::info!("获取到标题 {title} link {source_url}");
            (source_url, title)
        } else {
            let mut src = YoutubeDl::new_search(http_client.clone(), text);
            let mut src = src.search(Some("bilisearch"), Some(5)).await?;
            let src = src.next().context("好像没有结果哦")?;
            let source_url = src.source_url.unwrap_or("获取链接失败".into());
            let title = src.title.unwrap_or("原神".into());
            log::info!("获取到标题 {title} link {source_url}");
            (source_url, title)
        };
        handler.stop();
        log::info!("停止指令发布成功");
        let _ = handler.play_input(YoutubeDl::new(http_client, source_url.clone()).into());
        log::info!("开始播放 {}", title);
        log::info!("开始响应信息");
        let response = format!("开始播放 [{title}]({source_url})");
        // 更新频道状态
        update_channel_state(ctx, &title).await?;

        ctx.reply(response)
            .await
            .map_err(|why| anyhow!("响应时发生错误 {why}"))?;
        return Ok(());
    }

    Err(anyhow::anyhow!("播放失败，可能没有加入语音频道"))
}

async fn update_channel_state(ctx: PoiseContext<'_>, title: &str) -> crate::Result<()> {
    {
        //因为这里前面已经加入了频道，所以一定不会空
        let mut voice_channel = CURRENT_JOIN_CHANNEL
            .access()
            .clone()
            .context("更新语音频道状态失败，可能没有加入语音频道")?;
        voice_channel
            .edit(
                ctx,
                EditChannel::new().status(format!("正在播放 [{title}]")),
            )
            .await
            .map_err(|why| anyhow!("编辑语音状态时发生错误 {why}"))?;
    };
    Ok(())
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
    {
        let mut current_join_channel = CURRENT_JOIN_CHANNEL.exclusive_access();
        if current_join_channel.is_none() {
            let _ = current_join_channel.replace(channel.clone());
        }
    }
    let mut handler = handler_lock.lock().await;
    handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);
    // handler.add_global_event(Event::Track(TrackEvent::End))
    let reply = CreateReply::default()
        .ephemeral(true)
        .content(format!("已加入语音频道: {}", channel.name));
    ctx.send(reply).await?;
    Ok(())
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
        // 暂停播放就重置状态
        update_channel_state(ctx, "").await?;
        {
            // 离开了频道，所以可以丢弃值
            let mut current_join_channel = CURRENT_JOIN_CHANNEL.exclusive_access();
            if current_join_channel.is_some() {
                let _ = current_join_channel.take();
            }
        }
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

    let cell = manager.get(guild_id).context("找不到对应的频道")?;
    {
        let mut handler = cell.lock().await;
        handler.stop();
        let response = CreateReply::default().content("已停止播放").ephemeral(true);
        ctx.send(response).await?;
    }

    Ok(())
}

struct TrackErrorNotifier;

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

pub fn music_export() -> ExportVec {
    vec![music()]
}
