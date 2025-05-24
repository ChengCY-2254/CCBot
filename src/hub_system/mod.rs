//! 该模块主要使用serenity进行构建
mod model;

mod guild_message;
pub use guild_message::*;
#[cfg(feature = "ai-chat")]
mod ai;
#[cfg(feature = "ai-chat")]
pub use ai::*;
