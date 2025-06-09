use crate::PoiseContext;
use crate::commands::music::utils::{get_http_and_songbird, update_channel_state};
use anyhow::{Context, anyhow};
use songbird::input::{Compose, YoutubeDl};

/// 播放音乐，支持列表请查看yt-dlp的支持网站。
#[poise::command(slash_command, rename = "play")]
pub(super) async fn play(
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
        let (source_url, title, duration) =
            if text.starts_with("https://") || text.starts_with("http://") {
                let data = YoutubeDl::new(http_client.clone(), text)
                    .aux_metadata()
                    .await?;
                let source_url = data.source_url.unwrap_or("获取链接失败".into());
                let title = data.title.unwrap_or("原神".into());
                let duration = data.duration.unwrap();
                log::info!("获取到标题 {title} 链接 {source_url} 时长 {}", duration.as_secs());
                (source_url, title, duration)
            } else {
                let mut src = YoutubeDl::new_search(http_client.clone(), text);
                let mut src = src.search(Some("bilisearch"), Some(5)).await?;
                let src = src.next().context("好像没有结果哦")?;
                let source_url = src.source_url.unwrap_or("获取链接失败".into());
                let title = src.title.unwrap_or("原神".into());
                let duration = src.duration.unwrap();
                log::info!("获取到标题 {title} link {source_url} 时长 {}", duration.as_secs());
                (source_url, title, duration)
            };
        handler.stop();
        log::info!("停止指令发布成功");

        let track_handle =
            handler.play_input(YoutubeDl::new(http_client, source_url.clone()).into());
        
        let chinese_time = super::utils::format_chinese_time(duration);

        super::utils::set_track_handle(track_handle);

        log::info!("开始播放 {}", title);
        log::info!("开始响应信息");
        let response = format!("🎵 开始播放 [{title}]({source_url}) 时长 **{chinese_time}**");
        // 更新频道状态
        update_channel_state(ctx, &title).await?;

        ctx.reply(response)
            .await
            .map_err(|why| anyhow!("响应时发生错误 {why}"))?;
        return Ok(());
    }

    Err(anyhow::anyhow!("播放失败，可能没有加入语音频道"))
}
