use crate::utils::{create_ephemeral_reply, with_time_to_utc8};
use crate::PoiseContext;
use tracing::instrument;

#[poise::command(slash_command, prefix_command)]
#[instrument]
/// 获取并检查用户信息
pub(super) async fn ping(
    ctx: PoiseContext<'_>,
    #[description = "选择一个用户"] user: Option<poise::serenity_prelude::User>,
) -> crate::Result<()> {
    if user.is_none() {
        let response = create_ephemeral_reply("请选择一个用户");
        ctx.send(response).await?;
        return Ok(());
    }
    let user = user.unwrap();

    let user_create_time = {
        let time = user.created_at().to_utc();
        with_time_to_utc8(time)
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
    };
    let content = format!(
        "<@{}> 用户id为 `{}` 创建于 {}",
        user.id, user.id, user_create_time
    );
    let response = create_ephemeral_reply(content);

    ctx.send(response).await?;
    Ok(())
}
