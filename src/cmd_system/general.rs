//! This file contains the implementation of the HubSystem struct and its associated methods.
//! `[#poise::command]`中的`#[channel_types]`对应路径为[serenity::model::channel::ChannelType] Enum

use chrono::FixedOffset;
use futures::{Stream, StreamExt};
use lazy_static::lazy_static;
use poise::CreateReply;
use serenity::all::{ActivityData, MessageBuilder};
use std::ops::Deref;

lazy_static! {
    /// UTC+8时区计算
    pub static ref UTC8: FixedOffset = FixedOffset::east_opt(8 * 3600).unwrap();
}

///用户数据
pub struct Data {}
///错误类型
pub type Error = anyhow::Error;
///上下文类型
pub type PoiseContext<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command, context_menu_command = "用户信息")]
pub async fn ping(
    ctx: PoiseContext<'_>,
    #[description = "选择一个用户"] user: poise::serenity_prelude::User,
) -> crate::Result<()> {
    let u = user;

    let response = MessageBuilder::new()
        .mention(&u)
        .push("创建于")
        .push_bold_safe(ToString::to_string(
            &u.created_at()
                .to_utc()
                .with_timezone(UTC8.deref())
                .format("%Y-%M-%d %H:%M:%S"),
        ))
        .build();

    ctx.say(response).await?;
    Ok(())
}

/// 注册命令的命令，需要使用@`[yourbot]` register来进行使用
#[poise::command(prefix_command, aliases("reg"))]
pub async fn register(ctx: PoiseContext<'_>) -> crate::Result<()> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

///机器人状态转换命令
#[poise::command(
    slash_command,
    aliases("status"),
    required_permissions = "ADMINISTRATOR"
)]
/// 设置机器人状态
pub async fn set_status(
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
pub async fn autocomplete_activity_type(
    _ctx: PoiseContext<'_>,
    partial: &str,
) -> impl Stream<Item = String> {
    futures::stream::iter(&["playing", "listening", "streaming", "watching", "custom"])
        .filter(move |name| futures::future::ready(name.starts_with(partial)))
        .map(|name| name.to_string())
}

