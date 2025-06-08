//! 该模块中的命令使用poise与songbird构建
mod utils;
// 通用模块
create_command_module!(general);
// 音乐模块
create_command_module!(music);
// 管理模块
create_command_module!(manage);
