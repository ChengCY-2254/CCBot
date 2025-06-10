//! # 管理相关
//! 这里创建频道管理命令  
//! 首先需要的是将xx频道加入撤回列表  
//! 那么我需要的是add withdraw #channelID和remove withdraw #channelID  
//! 需要查看subcommand的写法[link](https://github.com/serenity-rs/poise/blob/current/examples/feature_showcase/subcommand_required.rs)    
//! 吃了个大亏，应该把add放到withdraw的子命令中，而不是放在顶层，也就是 withdraw add #channelID
//! todo 创建一个叫up的命令，将配置文件传到config目录下，用法为 up filename:xyz.md content:……

mod withdraw;
mod send_message;
mod prompt;
use crate::commands::manage::prompt::prompt;
use crate::commands::manage::send_message::send_message;
use crate::commands::manage::withdraw::withdraw;
use crate::macros::ExportCommand;

pub fn manage_export() -> ExportCommand {
    vec![withdraw(), prompt(), send_message()]
}
