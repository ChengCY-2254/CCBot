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
    required_permissions = "MANAGE_MESSAGES"
)]
pub(super) async fn clear(
    ctx: PoiseContext<'_>,
    #[description = "清除的消息数量，如果是私聊，只能删除机器人自己的消息"] count: u8,
) -> crate::Result<()> {
    let bot_id = ctx.serenity_context().cache.current_user().id;
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
        let mut delete_count = 0u16;
        for message in &messages {
            // 如果是用户消息并且是私聊场景就跳过。
            // 因为机器人无法删除私聊中的用户消息
            // 但是频道场景下，消息是可以删除的。
            #[allow(deprecated)]
            if message.author.id != bot_id && message.is_private() {
                continue;
            }
            message.delete(ctx.serenity_context()).await?;
            delete_count += 1;
        }
        let response = create_ephemeral_reply(format!("已删除{}条消息", delete_count));
        ctx.send(response).await?;
    }
    Ok(())
}
