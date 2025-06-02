//! 取消暂停

use crate::PoiseContext;

/// 继续播放机器人暂停的音乐
#[poise::command(slash_command, rename = "继续播放")]
pub(super) async fn music_continue(ctx: PoiseContext<'_>) -> crate::Result<()> {
    let track_handle = super::utils::get_current_track_handle();
    if let Some(track_handle) = track_handle {
        track_handle.play()?;
    } else {
        ctx.reply("没有在播放音乐，无法继续播放").await?;
    }
    Ok(())
}
