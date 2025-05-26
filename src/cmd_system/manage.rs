//! # 管理相关
//! 这里创建频道管理命令  
//! 首先需要的是将xx频道加入撤回列表  
//! 那么我需要的是add withdraw #channelID和remove withdraw #channelID  
//! 需要查看subcommand的写法[link](https://github.com/serenity-rs/poise/blob/current/examples/feature_showcase/subcommand_required.rs)    
//! 吃了个大亏，应该把add放到withdraw的子命令中，而不是放在顶层，也就是 withdraw add #channelID

use crate::keys::BotDataKey;
use crate::{ExportVec, PoiseContext, create_ephemeral_reply};
use anyhow::{Context, anyhow};
use futures::Stream;
use futures::StreamExt;
use lazy_static::lazy_static;
use serenity::all::{CreateMessage, GuildChannel, MessageBuilder};
use std::ops::Deref;
use std::path::PathBuf;

lazy_static! {
    pub static ref CONFIG_DIR: PathBuf = {
        let mut buf = PathBuf::new();
        buf.push("config");
        buf
    };
}

#[poise::command(
    slash_command,
    subcommands("add", "remove", "list"),
    subcommand_required,
    required_permissions = "ADMINISTRATOR",
    prefix_command
)]
/// 管理撤回频道，机器人自动删除该频道中的消息
pub async fn withdraw(_ctx: PoiseContext<'_>) -> crate::Result<()> {
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
/// 添加一个频道到撤回列表
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
/// 从撤回列表中移除一个频道
pub async fn remove(
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
pub async fn list(ctx: PoiseContext<'_>) -> crate::Result<()> {
    let channel_vec = {
        let type_map = ctx.serenity_context().data.read().await;
        let bot_data = type_map
            .get::<BotDataKey>()
            .context("机器人数据文件访问失败")?;
        bot_data.access().monitored_channels.clone()
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
/// 切换系统提示
/// 自动补全程序需要给出路径下的md文件位置
#[poise::command(
    slash_command,
    rename = "switch_prompt",
    required_permissions = "ADMINISTRATOR",
    default_member_permissions = "ADMINISTRATOR",
    guild_only,
)]
pub async fn switch_system_prompt(
    ctx: PoiseContext<'_>,
    #[autocomplete = "autocomplete_ai_prompt_list"]
    #[description = "角色"]
    file_name: Option<String>,
) -> crate::Result<()> {
    if let Some(file_name) = file_name {
        handle_switch_prompt(ctx, file_name).await
    } else {
        handle_show_prompt(ctx).await
    }
}
/// 处理系统提示切换逻辑
async fn handle_switch_prompt(ctx: PoiseContext<'_>, file_name: String) -> crate::Result<()> {
    let (prompt, content) = {
        let type_map = ctx.serenity_context().data.write().await;
        let bot_data = type_map
            .get::<BotDataKey>()
            .context("app数据目录访问失败")?;
        let mut bot_data = bot_data.exclusive_access();
        bot_data.aiconfig.use_others_prompt(file_name)?;
        let (prompt_name, content) = bot_data.aiconfig.get_system_prompt()?;
        (prompt_name, content)
    };
    let reply = create_ephemeral_reply(format!("已使用 {} 的提示文件\r\n {}", prompt, content));
    ctx.send(reply).await?;
    Ok(())
}

/// 自动补全程序，把config目录下的md文件过滤出来返回给客户端
async fn handle_show_prompt(ctx: PoiseContext<'_>) -> crate::Result<()> {
    let type_map = ctx.serenity_context().data.read().await;
    let bot_data = type_map
        .get::<BotDataKey>()
        .context("app数据目录访问失败")?;
    let (prompt, content) = bot_data.access().aiconfig.get_system_prompt()?;
    let reply = create_ephemeral_reply(format!("已使用 {} 的提示文件\r\n {}", prompt, content));
    ctx.send(reply).await?;
    Ok(())
}

async fn autocomplete_ai_prompt_list(
    _ctx: PoiseContext<'_>,
    partial: &str,
) -> impl Stream<Item = String> {
    let files = std::fs::read_dir(CONFIG_DIR.deref()).unwrap();
    let names = files
        .into_iter()
        .map(|dir| {
            let file_name = dir.unwrap().file_name();
            let file_name = file_name.to_str().unwrap();
            file_name.to_string()
        })
        .filter(|name| name.ends_with(".md"))
        .collect::<Vec<_>>();
    futures::stream::iter(names)
        .filter(move |name| futures::future::ready(name.starts_with(partial)))
}
pub fn manage_export() -> ExportVec {
    vec![withdraw(), switch_system_prompt()]
}
