/// 这个宏用于将模块添加到当前作用域中，并导出其内容。   
/// 如果声明为 `add_sub_mod!(general)`     
/// 那么导出格式为     
/// ```ignore
/// pub fn general_export() -> Vec<poise::Command<Data, Error>> {
///     vec![add()]
/// }
/// ```
macro_rules! add_cmd_mod {
    ($mod_name:ident) => {
        mod $mod_name;
        paste::paste!{
            pub use $mod_name::[<$mod_name _export>];
        }
    };
}
/// 这个宏用于创建一个模块并公开导入其中的handler
/// ```ignore
/// add_handler_mod(manager)
/// ```
/// 将会生成
/// ```ignore
/// mod manager;
/// pub use manager::ManagerHandler;
/// ```
macro_rules! add_handler_mod {
    ($mod_name:ident) => {
        mod $mod_name;
        paste::paste!{
            pub use $mod_name::[<$mod_name:camel Handler>];
        }
    };
}