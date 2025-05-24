//! 该模块主要使用serenity进行构建
#[cfg(feature = "ai-chat")]
mod ai;
mod guild_message;
mod manager;
mod model;

#[cfg(feature = "ai-chat")]
pub use ai::*;
pub use guild_message::*;
pub use manager::*;
