use crate::utils::create_ephemeral_reply;
use crate::PoiseContext;
use anyhow::anyhow;
use serenity::all::{CreateMessage, GuildChannel};
use tracing::instrument;

/// 发送消息，请限制在2000字内
#[instrument(level = "trace")]
#[poise::command(slash_command, owners_only)]
pub(super) async fn send_message(
    ctx: PoiseContext<'_>,
    channel: GuildChannel,
    message: String,
) -> crate::Result<()> {
    log::trace!("机器人收到消息发送请求，正在检查……");
    if message.len() >= 2000 {
        return Err(anyhow!("消息长度超过2000字，请重新输入"));
    }
    log::trace!(
        "发送请求检查通过，正在将消息[{}]发送到频道 {}",
        &message,
        channel.name
    );

    channel
        .send_message(ctx, CreateMessage::new().content(message))
        .await?;
    log::trace!("消息发送成功");
    log::trace!("正在通知用户");
    ctx.send(create_ephemeral_reply("消息发送成功！")).await?;
    log::trace!("通知完成");
    Ok(())
}