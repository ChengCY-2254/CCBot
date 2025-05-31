//! 该模块主要使用serenity进行构建
// 测试模块
// create_handler_module!(guild_message);
// 与[crate::cmd_system::manage_export]联动
create_handler_module!(manager);
// AI对话模块
create_handler_module!(ai);
// 服务启动时的挂钩
create_handler_module!(start);
