//! 这个模块用于展示帮助信息

use crate::utils::create_ephemeral_reply;
use crate::{CommandVec, PoiseContext};

const HELP: &str = include_str!("help.md");
#[poise::command(slash_command)]
/// 机器人的帮助说明
pub async fn help(ctx: PoiseContext<'_>) -> crate::Result<()> {
    let content = format!("当前程序版本{}\r\n{}", crate::VERSION, HELP);
    ctx.send(create_ephemeral_reply(content)).await?;
    Ok(())
}

pub fn help_export() -> CommandVec {
    vec![help()]
}
