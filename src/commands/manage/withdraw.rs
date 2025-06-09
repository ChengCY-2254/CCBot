use crate::utils::create_ephemeral_reply;
use crate::PoiseContext;
use anyhow::{anyhow, Context};
use serenity::all::{CreateMessage, GuildChannel, MessageBuilder};
use crate::config::data_config::APP_STATE_MANAGER;

#[poise::command(
    slash_command,
    subcommands("add", "remove", "list"),
    subcommand_required,
    required_permissions = "ADMINISTRATOR",
    prefix_command,
    owners_only
)]
/// 管理撤回频道，机器人自动删除该频道中的消息
pub(super) async fn withdraw(_ctx: PoiseContext<'_>) -> crate::Result<()> {
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
/// 添加一个频道到撤回列表
pub(super) async fn add(
    ctx: PoiseContext<'_>,
    #[description = "频道"] channel: Option<GuildChannel>,
) -> crate::Result<()> {
    if let Some(channel) = channel {
        handle_add(ctx, channel).await?;
    }
    Ok(())
}
#[poise::command(slash_command, prefix_command)]
/// 从撤回列表中移除一个频道
pub(super) async fn remove(
    ctx: PoiseContext<'_>,
    #[description = "频道"] channel: Option<GuildChannel>,
) -> crate::Result<()> {
    if let Some(channel) = channel {
        handle_remove(ctx, channel).await?;
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
/// 查看当前禁止留存消息的频道
pub(super) async fn list(ctx: PoiseContext<'_>) -> crate::Result<()> {
    let channel_vec = {
        let app_state = APP_STATE_MANAGER.get_app_state();
        app_state.access().monitored_channels.clone()
    };

    if channel_vec.is_empty() {
        ctx.send(create_ephemeral_reply("当前没有监控消息撤回的频道"))
            .await?;
    } else {
        let mut builder = MessageBuilder::new();
        builder.push_bold_line("当前不允许发消息的频道");
        for channel_id in channel_vec.iter() {
            builder.mention(&channel_id.to_channel(&ctx).await?);
            builder.push("\n");
        }
        let content = builder.build();
        let response = create_ephemeral_reply(content).ephemeral(true);
        ctx.send(response).await.map_err(|why| anyhow!(why))?;
    }

    Ok(())
}

async fn handle_add(ctx: PoiseContext<'_>, channel: GuildChannel) -> crate::Result<()> {
    let (already_exists, _) = {
        let app_state = APP_STATE_MANAGER.get_app_state();
        let mut data = app_state.exclusive_access();
        let exists = data.monitored_channels.contains(&channel.id);
        let name = channel.name.clone();
        if !exists {
            data.add_monitored_channel(channel.id);
            APP_STATE_MANAGER.save()?;
        }
        (exists, name)
    };

    if already_exists {
        let response = create_ephemeral_reply(format!("频道 <#{}> 已经在撤回列表中", channel.id));
        ctx.send(response).await?;
    } else {
        // 给受管控的频道发送公告
        let announcement = format!(
            "**<#{}> 已经被添加到撤回列表中，所有消息将被自动删除。**",
            channel.id
        );
        channel
            .send_message(&ctx, CreateMessage::new().content(announcement))
            .await
            .context("发送频道公告失败")?;
        let response = create_ephemeral_reply(format!("已将频道 <#{}> 添加到撤回列表", channel.id));
        ctx.send(response).await.map_err(|why| anyhow!("{}", why))?;
    }
    Ok(())
}
async fn handle_remove(ctx: PoiseContext<'_>, channel: GuildChannel) -> crate::Result<()> {
    let (exists, _) = {
        let app_state = APP_STATE_MANAGER.get_app_state();
        let mut data = app_state.exclusive_access();
        let exists = data.monitored_channels.contains(&channel.id);
        let name = channel.name.clone();
        if exists {
            data.remove_monitored_channel(channel.id);
            APP_STATE_MANAGER.save()?;
        }
        (exists, name)
    };

    if exists {
        let response =
            create_ephemeral_reply(format!("已将频道 <#{}> 从撤回列表中移除", channel.id));
        let announcement = format!(
            "**<#{}> 已经从撤回列表中移除，消息将不再被自动删除。**",
            channel.id
        );
        channel
            .send_message(&ctx, CreateMessage::new().content(announcement))
            .await?;
        ctx.send(response).await?;
    } else {
        let response = create_ephemeral_reply(format!("频道 <#{}> 不在撤回列表中", channel.id));
        ctx.send(response).await?;
    }
    Ok(())
}