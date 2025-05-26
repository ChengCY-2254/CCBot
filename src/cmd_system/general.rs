//! # 通用模块
//! This file contains the implementation of the HubSystem struct and its associated methods.
//! `[#poise::command]`中的`#[channel_types]`对应路径为[serenity::model::channel::ChannelType] Enum

use crate::{ExportVec, PoiseContext, create_ephemeral_reply};
use anyhow::anyhow;
use chrono::FixedOffset;
use futures::{Stream, StreamExt};
use lazy_static::lazy_static;
use poise::CreateReply;
use serenity::all::{ActivityData, GetMessages};
use std::ops::Deref;

lazy_static! {
    /// UTC+8时区计算
    static ref UTC8: FixedOffset = FixedOffset::east_opt(8 * 3600).unwrap();
}

#[poise::command(slash_command, prefix_command, context_menu_command = "用户信息")]
/// 获取并检查用户信息
async fn ping(
    ctx: PoiseContext<'_>,
    #[description = "选择一个用户"] user: poise::serenity_prelude::User,
) -> crate::Result<()> {
    let user_create_time = user
        .created_at()
        .to_utc()
        .with_timezone(UTC8.deref())
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    let content = format!(
        "<@{}> 用户id为 `{}` 创建于 {}",
        user.id, user.id, user_create_time
    );
    let response = create_ephemeral_reply(content);

    ctx.send(response).await?;
    Ok(())
}

/// 注册命令的命令，需要使用@`[yourbot]` register来进行使用
#[poise::command(prefix_command, aliases("reg"), owners_only)]
async fn register(ctx: PoiseContext<'_>) -> crate::Result<()> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

///机器人状态转换命令
#[poise::command(
    slash_command,
    aliases("status"),
    rename = "status",
    required_permissions = "ADMINISTRATOR",
    default_member_permissions = "ADMINISTRATOR",
    owners_only
)]
async fn set_status(
    ctx: PoiseContext<'_>,
    #[autocomplete = "autocomplete_activity_type"]
    #[description = "状态类型"]
    activity_type: String,
    #[description = "内容"] activity_name: String,
    #[description = "活动网址"] url: Option<String>,
) -> crate::Result<()> {
    {
        let activity_name = activity_name.clone();
        match activity_type.as_str() {
            "playing" => {
                ctx.serenity_context()
                    .set_activity(Some(ActivityData::playing(activity_name)));
            }
            "streaming" => {
                ctx.serenity_context()
                    .set_activity(Some(ActivityData::streaming(
                        activity_name,
                        url.unwrap_or_else(|| "https://ys.mihoyo.com".to_owned()),
                    )?));
            }
            "listening" => {
                ctx.serenity_context()
                    .set_activity(Some(ActivityData::listening(activity_name)));
            }
            "watching" => {
                ctx.serenity_context()
                    .set_activity(Some(ActivityData::watching(activity_name)));
            }
            _ => {
                ctx.serenity_context()
                    .set_activity(Some(ActivityData::custom(activity_name)));
            }
        };
    }
    //发送仅自己可见的消息
    let reply = CreateReply::default().ephemeral(true).content("状态已更新");
    ctx.send(reply).await?;
    Ok(())
}

/// 状态补全程序
async fn autocomplete_activity_type(
    _ctx: PoiseContext<'_>,
    partial: &str,
) -> impl Stream<Item = String> {
    futures::stream::iter(&["playing", "listening", "streaming", "watching", "custom"])
        .filter(move |name| futures::future::ready(name.starts_with(partial)))
        .map(|name| name.to_string())
}

/// 清除消息的命令
#[poise::command(
    slash_command,
    prefix_command,
    aliases("clear"),
    rename = "clear",
    required_permissions = "MANAGE_MESSAGES"
)]
async fn clear(
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
        return Ok(())
    } else {
        ctx.defer()
            .await
            .map_err(|why| anyhow!("延迟响应时发生错误 {why}"))?;
        let mut delete_count = 0u16;
        for message in &messages {
            // 如果是用户消息并且是私聊就跳过。
            // 因为机器人无法删除私聊中的用户消息
            #[allow(deprecated)]
            if !message.author.bot && message.is_private() {
                continue;
            } else {
                message.delete(ctx.serenity_context()).await?;
                delete_count+=1;
            }
        }
        let response = create_ephemeral_reply(format!("已删除{}条消息", delete_count));
        ctx.send(response).await?;
    }
    Ok(())
}

pub fn general_export() -> ExportVec {
    vec![ping(), register(), set_status(), clear()]
}
