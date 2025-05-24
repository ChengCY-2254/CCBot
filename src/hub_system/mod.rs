//! 该模块主要使用serenity进行构建
// 测试模块
add_handler_mod!(guild_message);
// 与[crate::cmd_system::manage_export]联动
add_handler_mod!(manager);
// AI对话模块
add_handler_mod!(ai);
