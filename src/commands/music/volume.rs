//! 音量管理

use crate::PoiseContext;
use crate::config::data_config::APP_STATE_MANAGER;
use anyhow::anyhow;

/// 为机器人设置音量，数值在100..1之间
#[poise::command(slash_command, rename = "设置音量")]
pub(super) async fn set_volume(
    ctx: PoiseContext<'_>,
    #[description = "音量 100..1"] volume: Option<f32>,
) -> crate::Result<()> {
    // 设置音量和展示音量
    // 将音量设置到文件中
    if let Some(volume) = volume {
        set_volume_handle(ctx, volume).await?;
    } else {
        show_volume(ctx).await?;
    }
    Ok(())
}

async fn set_volume_handle(ctx: PoiseContext<'_>, volume: f32) -> crate::Result<()> {
    let raw_volume = volume;
    if !(1.0..=100.0).contains(&volume) {
        ctx.reply("音量范围是 100..1")
            .await
            .map_err(|why| anyhow!("响应时发生错误 {why}"))?;
        return Ok(());
    }
    let volume = volume / 100.0;
    // 设置音量
    {
        let app_state = APP_STATE_MANAGER.get_app_state();
        let mut app_state = app_state.exclusive_access();
        app_state.volume = volume;
    }
    APP_STATE_MANAGER.save()?;

    let track_handle = super::utils::get_current_track_handle();
    if let Some(track_handle) = track_handle {
        track_handle.set_volume(volume)?;
        let response = format!("当前播放音量是 {}", raw_volume);
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

async fn show_volume(ctx: PoiseContext<'_>) -> crate::Result<()> {
    let volume = APP_STATE_MANAGER.get_app_state().access().volume;
    let volume_message = format!("当前音量为: **{}**", volume);
    ctx.say(volume_message).await?;
    Ok(())
}
