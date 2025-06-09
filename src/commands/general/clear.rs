use crate::PoiseContext;
use crate::utils::create_ephemeral_reply;
use anyhow::anyhow;
use serenity::all::GetMessages;

/// 清除消息的命令
#[poise::command(
    slash_command,
    prefix_command,
    aliases("clear"),
    rename = "clear",
    required_permissions = "MANAGE_MESSAGES",
    guild_only
)]
pub(super) async fn clear(
    ctx: PoiseContext<'_>,
    #[description = "清除的消息数量，如果是私聊，只能删除机器人自己的消息"] count: u8,
) -> crate::Result<()> {
    let messages = ctx
        .channel_id()
        .messages(ctx.serenity_context(), GetMessages::new().limit(count))
        .await?;

    if messages.is_empty() {
        let response = create_ephemeral_reply("没有找到可删除的消息");
        ctx.send(response).await?;
        return Ok(());
    } else {
        ctx.defer_ephemeral()
            .await
            .map_err(|why| anyhow!("延迟响应时发生错误 {why}"))?;

        ctx.channel_id()
            .delete_messages(ctx, messages.into_iter().map(|message| message.id))
            .await?;

        let response = create_ephemeral_reply(format!("已删除{}条消息", count));
        ctx.send(response).await?;
    }
    Ok(())
}
