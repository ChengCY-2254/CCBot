use crate::PoiseContext;
use anyhow::Context;
use poise::CreateReply;

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