//! # 音乐模块
//! 语音方向的内容

mod join;
mod leave;
mod music_continue;
mod pause;
mod play;
mod stop;
mod utils;
mod volume;

use crate::PoiseContext;
use crate::macros::ExportCommand;
use join::join;
use leave::leave;
use music_continue::music_continue;
use pause::pause;
use play::play;
use stop::stop;
use volume::set_volume;

/// 音乐相关命令
#[poise::command(
    slash_command,
    subcommands(
        "play",
        "join",
        "leave",
        "stop",
        "pause",
        "set_volume",
        "music_continue"
    )
)]
pub async fn music(_ctx: PoiseContext<'_>) -> crate::Result<()> {
    Ok(())
}

pub fn music_export() -> ExportCommand {
    vec![music()]
}
