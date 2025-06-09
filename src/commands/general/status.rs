use crate::config::data_config::APP_STATE_MANAGER;
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
    let activity_data = match_activity_type(&activity_type,activity_name,url);
    {
        ctx.serenity_context().set_activity(Some(activity_data.clone()));
        let app_state = APP_STATE_MANAGER.get_app_state();
        app_state.exclusive_access().bot_activity = crate::config::ActivityData::from(activity_data);
        APP_STATE_MANAGER.save()?;
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
    futures::stream::iter(["playing", "listening", "streaming", "watching", "custom"])
        .filter(move |name| futures::future::ready(name.starts_with(partial)))
        .map(|name| name.to_string())
}
/// 匹配活动类型
fn match_activity_type(
    activity_str: &str,
    activity_name: impl Into<String>,
    url: Option<String>,
) -> ActivityData {
    match activity_str {
        "playing" => ActivityData::playing(activity_name),
        "streaming" => ActivityData::streaming(
            activity_name,
            url.unwrap_or_else(|| "https://ys.mihoyo.com".to_owned()),
        )
        .unwrap(),
        "listening" => ActivityData::listening(activity_name),
        "watching" => ActivityData::watching(activity_name),
        _ => ActivityData::custom(activity_name),
    }
}
