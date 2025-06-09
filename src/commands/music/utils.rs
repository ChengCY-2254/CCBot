//! music模块的通用工具

use crate::PoiseContext;
use crate::commands::utils::get_http_client;
use crate::config::data_config::APP_STATE_MANAGER;
use crate::shared::UpSafeCell;
use anyhow::{Context, anyhow};
use lazy_static::lazy_static;
use reqwest::Client;
use serenity::all::{EditChannel, GuildChannel};
use songbird::Songbird;
use songbird::tracks::TrackHandle;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

lazy_static! {
    /// 当前播放的TrackHandle
    static ref CURRENT_TRACK_HANDLE: UpSafeCell<Option<TrackHandle>> =
        unsafe { UpSafeCell::new(None) };
}

/// 更新当前语音频道的状态
pub(super) async fn update_channel_state(ctx: PoiseContext<'_>, title: &str) -> crate::Result<()> {
    //因为这里前面已经加入了频道，所以一定不会空
    let mut voice_channel = APP_STATE_MANAGER
        .get_app_state()
        .access()
        .current_voice_channel
        .clone()
        .context("更新语音频道状态失败，可能没有加入语音频道")?;
    voice_channel
        .edit(
            ctx,
            EditChannel::new().status(format!("正在播放 [{title}]")),
        )
        .await
        .map_err(|why| anyhow!("编辑语音状态时发生错误 {why}"))?;
    Ok(())
}

pub(super) async fn get_http_and_songbird(
    ctx: PoiseContext<'_>,
) -> crate::Result<(Client, Arc<Songbird>)> {
    let http_client = get_http_client(&ctx).await?;
    log::info!("http客户端获取成功");

    let manager = songbird::get(ctx.serenity_context())
        .await
        .with_context(|| "获取语音客户端失败")?
        .clone();
    log::info!("获取语音客户端成功");
    Ok((http_client, manager))
}

/// 设置当前播放的TrackHandle
pub(super) fn set_track_handle(track_handle: TrackHandle) {
    let mut current_track_handle = CURRENT_TRACK_HANDLE.deref().exclusive_access();
    current_track_handle.replace(track_handle);
}

pub(super) fn get_current_track_handle() -> Option<TrackHandle> {
    CURRENT_TRACK_HANDLE.access().clone()
}

pub(super) fn clear_current_track_handle() {
    let mut current_track_handle = CURRENT_TRACK_HANDLE.deref().exclusive_access();
    current_track_handle.take();
}

/// 如果有正在播放的音频，就执行future，否则返回错误
/// # argument
/// ## `future`: 一个异步函数，参数为TrackHandle，用于直接管理音频跟踪处理器
#[allow(unused)]
pub(super) async fn track_handle_scope<F, R>(
    future: F,
) -> crate::Result<impl Future<Output = R> + Send>
where
    F: FnOnce(TrackHandle) -> Pin<Box<dyn Future<Output = R> + Send>>,
{
    let track_handle = get_current_track_handle().context("没有正在播放的音频");
    if let Ok(handle) = track_handle {
        Ok(future(handle))
    } else {
        Err(anyhow::anyhow!("没有正在播放的音频"))
    }
}

///  设置当前语音频道
pub(super) fn set_current_voice_channel(channel: GuildChannel) -> crate::Result<()> {
    let app_state = APP_STATE_MANAGER.get_app_state();
    app_state
        .exclusive_access()
        .current_voice_channel
        .replace(channel);
    Ok(())
}

/// 获取当前语音频道
pub(super) fn get_current_voice_channel() -> crate::Result<GuildChannel> {
    APP_STATE_MANAGER
        .get_app_state()
        .access()
        .current_voice_channel
        .clone()
        .context("当前没有加入任何语音频道")
}

/// 删除当前语音频道
pub(super) fn clear_voice_channel() {
    let app_state = APP_STATE_MANAGER.get_app_state();
    app_state.exclusive_access().current_voice_channel.take();
}
