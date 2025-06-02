//! 音量管理

use crate::PoiseContext;
use anyhow::anyhow;

#[poise::command(slash_command, rename = "设置音量")]
pub(super) async fn set_volume(
    ctx: PoiseContext<'_>,
    #[description = "音量"] volume: f32,
) -> crate::Result<()> {
    let track_handle = super::utils::get_current_track_handle();
    if let Some(track_handle) = track_handle {
        track_handle.set_volume(volume)?;
        let response = format!("当前音量是 {}", volume);
        ctx.reply(response)
            .await
            .map_err(|why| anyhow!("响应时发生错误 {why}"))?;
    } else {
        ctx.reply("没有在播放音乐，无法设置音量")
            .await
            .map_err(|why| anyhow!("响应时发生错误 {why}"))?;
    }
    Ok(())
}
