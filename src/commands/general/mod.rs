//! # 通用模块
//! This file contains the implementation of the HubSystem struct and its associated methods.
//! `[#poise::command]`中的`#[channel_types]`对应路径为[serenity::model::channel::ChannelType] Enum

mod clear;
mod help;
mod ping;
mod register;
mod status;
mod version;
use crate::commands::general::clear::clear;
use crate::commands::general::help::help;
use crate::commands::general::ping::ping;
use crate::commands::general::register::register;
use crate::commands::general::status::set_status;
use crate::commands::general::version::version;
use crate::macros::ExportCommand;

/// 导出命令
pub fn general_export() -> ExportCommand {
    vec![ping(), register(), set_status(), clear(), help(), version()]
}
