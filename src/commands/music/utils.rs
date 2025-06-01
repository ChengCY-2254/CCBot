//! music模块的通用工具

use crate::commands::utils::get_http_client;
use crate::shared::UpSafeCell;
use crate::PoiseContext;
use anyhow::{anyhow, Context};
use lazy_static::lazy_static;
use reqwest::Client;
use serenity::all::{EditChannel, GuildChannel};
use songbird::Songbird;
use std::sync::Arc;

lazy_static! {
    /// 当前加入的语音频道id
     pub static ref CURRENT_JOIN_CHANNEL: UpSafeCell<Option<GuildChannel>> =
        unsafe { UpSafeCell::new(None) };
}

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
