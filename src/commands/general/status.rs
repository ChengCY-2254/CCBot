use crate::PoiseContext;
use futures::Stream;
use futures::StreamExt;
use poise::CreateReply;
use serenity::all::ActivityData;
use tracing::instrument;

///机器人状态转换命令
#[poise::command(
    slash_command,
    aliases("status"),
    rename = "status",
    required_permissions = "ADMINISTRATOR",
    default_member_permissions = "ADMINISTRATOR",
    owners_only
)]
#[instrument]
pub(super) async fn set_status(
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
) -> impl Stream<Item=String> {
    futures::stream::iter(["playing", "listening", "streaming", "watching", "custom"])
        .filter(move |name| futures::future::ready(name.starts_with(partial)))
        .map(|name| name.to_string())
}