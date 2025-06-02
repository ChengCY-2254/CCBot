//! music模块的通用工具

use crate::commands::utils::get_http_client;
use crate::shared::UpSafeCell;
use crate::PoiseContext;
use anyhow::{anyhow, Context};
use lazy_static::lazy_static;
use reqwest::Client;
use serenity::all::{EditChannel, GuildChannel};
use songbird::tracks::TrackHandle;
use songbird::Songbird;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

lazy_static! {
    /// 当前加入的语音频道id
     pub static ref CURRENT_JOIN_CHANNEL: UpSafeCell<Option<GuildChannel>> =
        unsafe { UpSafeCell::new(None) };
    /// 当前播放的TrackHandle
    static ref CURRENT_TRACK_HANDLE: UpSafeCell<Option<TrackHandle>> =
        unsafe { UpSafeCell::new(None) };
}

/// 更新当前语音频道的状态
pub(super) async fn update_channel_state(ctx: PoiseContext<'_>, title: &str) -> crate::Result<()> {
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
