//! # 音乐模块
//! 语音方向的内容

mod join;
mod leave;
mod play;
mod stop;
mod utils;

use crate::macros::ExportCommand;
use crate::PoiseContext;
use join::join;
use leave::leave;
use play::play;
use stop::stop;

/// 音乐相关命令
#[poise::command(slash_command, subcommands("play", "join", "leave", "stop"))]
pub async fn music(_ctx: PoiseContext<'_>) -> crate::Result<()> {
    Ok(())
}

pub fn music_export() -> ExportCommand {
    vec![music()]
}
