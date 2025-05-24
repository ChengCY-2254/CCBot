//! 该模块主要使用serenity进行构建
mod model;

add_handler_mod!(guild_message);
add_handler_mod!(manager);
#[cfg(feature = "ai-chat")]
add_handler_mod!(ai);