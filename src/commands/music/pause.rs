//! 音乐暂停

use crate::PoiseContext;
/// 暂停机器人播放的音频
#[poise::command(slash_command, rename = "暂停音乐")]
pub(super) async fn pause(ctx: PoiseContext<'_>) -> crate::Result<()> {
    let track_handle = super::utils::get_current_track_handle();
    if let Some(track_handle) = track_handle {
        track_handle.pause()?;
        ctx.say("已暂停").await?;
    } else {
        ctx.say("当前没有播放的音频").await?;
    }
    Ok(())
}
