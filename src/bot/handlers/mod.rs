// 与[crate::cmd_system::manage_export]联动
create_handler_module!(manager);
// AI对话模块
create_handler_module!(ai);
// 服务启动时的挂钩
create_handler_module!(start);
// 清除消息的命令
create_handler_module!(clear);