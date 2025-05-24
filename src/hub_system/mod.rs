//! 该模块主要使用serenity进行构建
mod model;

add_sub_mod!(guild_message);
#[cfg(feature = "ai-chat")]
add_sub_mod!(ai);