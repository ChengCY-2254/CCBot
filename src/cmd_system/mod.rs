//! 该模块中的命令使用poise与songbird构建
mod utils;
// 通用模块
add_cmd_mod!(general);
// 音乐模块
add_cmd_mod!(music);
// 管理模块
add_cmd_mod!(manage);
// 帮助命令
add_cmd_mod!(help);
