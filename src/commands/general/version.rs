use crate::PoiseContext;
#[poise::command(
    slash_command,
    prefix_command,
)]
/// 当前机器人版本
pub(super) async fn version(ctx:PoiseContext<'_>)->crate::Result<()>{
    let reply = format!("当前版本为 **{}** link: {}",crate::VERSION,env!("CARGO_PKG_REPOSITORY"));
    ctx.say(reply).await?;
    Ok(())
}