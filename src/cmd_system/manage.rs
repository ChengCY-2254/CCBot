//! 这里创建频道管理命令  
//! 首先需要的是将xx频道加入撤回列表  
//! 那么我需要的是add withdraw #channelID和remove withdraw #channelID  
//! 需要查看subcommand的写法[link](https://github.com/serenity-rs/poise/blob/current/examples/feature_showcase/subcommand_required.rs)
//! 吃了个大亏，应该把add放到withdraw的子命令中，而不是放在顶层，也就是 withdraw add #channelID

use crate::keys::BotDataKey;
use crate::{ExportVec, PoiseContext, create_ephemeral_reply};
use anyhow::{Context, anyhow};
use serenity::all::GuildChannel;

#[poise::command(
    slash_command,
    subcommands("add", "remove"),
    subcommand_required,
    required_permissions = "ADMINISTRATOR",
    prefix_command
)]
pub async fn withdraw(_ctx: PoiseContext<'_>) -> crate::Result<()> {
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn add(
    ctx: PoiseContext<'_>,
    #[description = "频道"] channel: Option<GuildChannel>,
) -> crate::Result<()> {
    if let Some(channel) = channel {
        handle_add(ctx, channel).await?;
    }
    Ok(())
}
#[poise::command(slash_command, prefix_command)]
pub async fn remove(
    ctx: PoiseContext<'_>,
    #[description = "频道"] channel: Option<GuildChannel>,
) -> crate::Result<()> {
    if let Some(channel) = channel {
        handle_remove(ctx, channel).await?;
    }
    Ok(())
}

async fn handle_add(ctx: PoiseContext<'_>, channel: GuildChannel) -> crate::Result<()> {
    let (already_exists, channel_name) = {
        let type_map = ctx.serenity_context().data.write().await;
        let data = type_map.get::<BotDataKey>();
        let mut data = data.context("app数据目录访问失败")?.exclusive_access();
        let exists = data.monitored_channels.contains(&channel.id);
        let name = channel.name.clone();
        if !exists {
            data.add_monitored_channel(channel.id);
            data.save("config/")?;
        }
        (exists, name)
    };

    if already_exists {
        let response = create_ephemeral_reply(format!(
            "频道 {}:{} 已经在撤回列表中",
            channel_name, channel.id
        ));
        ctx.send(response).await?;
    } else {
        let response = create_ephemeral_reply(format!("已将频道 {} 添加到撤回列表", channel_name));
        ctx.send(response).await.map_err(|why| anyhow!("{}", why))?;
    }
    Ok(())
}
async fn handle_remove(ctx: PoiseContext<'_>, channel: GuildChannel) -> crate::Result<()> {
    let (exists, channel_name) = {
        let type_map = ctx.serenity_context().data.write().await;
        let data = type_map.get::<BotDataKey>();
        let mut data = data.context("app数据目录访问失败")?.exclusive_access();
        let exists = data.monitored_channels.contains(&channel.id);
        let name = channel.name.clone();
        if exists {
            data.remove_monitored_channel(channel.id);
            data.save("config/")?;
        }
        (exists, name)
    };

    if exists {
        let response = create_ephemeral_reply(format!(
            "已将频道 {}:{} 从撤回列表中移除",
            channel_name, channel.id
        ));
        ctx.send(response).await?;
    } else {
        let response = create_ephemeral_reply(format!(
            "频道 {}:{} 不在撤回列表中",
            channel_name, channel.id
        ));
        ctx.send(response).await?;
    }
    Ok(())
}

pub fn manage_export() -> ExportVec {
    vec![withdraw()]
}
