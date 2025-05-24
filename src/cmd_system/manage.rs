//! 这里创建频道管理命令  
//! 首先需要的是将xx频道加入撤回列表  
//! 那么我需要的是add withdraw #channelID和remove withdraw #channelID  
//! 需要查看subcommand的写法[link](https://github.com/serenity-rs/poise/blob/current/examples/feature_showcase/subcommand_required.rs)

use crate::keys::BotDataKey;
use crate::{create_ephemeral_reply, ExportVec, PoiseContext};
use anyhow::Context;
use serenity::all::GuildChannel;

#[poise::command(
    slash_command,
    subcommands("withdraw"),
    subcommand_required,
    required_permissions = "ADMINISTRATOR",
    prefix_command
)]
pub async fn add(_ctx: PoiseContext<'_>) -> crate::Result<()> {
    Ok(())
}

/// 监控一个频道，对其中的消息进行撤回。
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR", prefix_command)]
pub async fn withdraw(
    ctx: PoiseContext<'_>,
    #[description = "频道"] channel: GuildChannel,
) -> crate::Result<()> {
    let add_monitored_channel = Box::pin(async || {
        let type_map = ctx.serenity_context().data.write().await;
        let data = type_map.get::<BotDataKey>();
        let mut data = data.context("app数据目录访问失败")?.exclusive_access();
        data.add_monitored_channel(channel.id);
        data.save("config/")
    });
    add_monitored_channel().await?;
    let response = create_ephemeral_reply(format!("已将频道 {} 添加到撤回列表", channel.name));
    ctx.send(response).await?;
    Ok(())
}

pub fn manage_export() -> ExportVec {
    vec![add()]
}
