//! 这个模块用于展示帮助信息

use crate::model::{ExportVec, PoiseContext};
use crate::utils::create_ephemeral_reply;

#[poise::command(slash_command)]
/// 机器人的帮助说明
pub async fn help(ctx: PoiseContext<'_>) -> crate::Result<()> {
    ctx.send(create_ephemeral_reply(format!("欢迎使用 CCBot {}",crate::VERSION))).await?;
    Ok(())
}

pub fn help_export() -> ExportVec {
    vec![help()]
}
