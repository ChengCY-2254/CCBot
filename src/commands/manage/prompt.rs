use crate::PoiseContext;
use crate::config::data_config::APP_STATE_MANAGER;
use crate::utils::create_ephemeral_reply;
use futures::Stream;
use futures::StreamExt;
use lazy_static::lazy_static;
use std::ops::Deref;
use std::path::PathBuf;

lazy_static! {
    static ref CONFIG_DIR: PathBuf = {
        let mut buf = PathBuf::new();
        buf.push("config");
        buf
    };
}

/// 切换系统提示
/// 自动补全程序需要给出路径下的md文件位置
#[poise::command(
    slash_command,
    rename = "switch_prompt",
    required_permissions = "ADMINISTRATOR",
    default_member_permissions = "ADMINISTRATOR",
    owners_only
)]
pub(super) async fn switch_system_prompt(
    ctx: PoiseContext<'_>,
    #[autocomplete = "autocomplete_ai_prompt_list"]
    #[description = "角色"]
    file_name: Option<String>,
) -> crate::Result<()> {
    if let Some(file_name) = file_name {
        handle_switch_prompt(ctx, file_name).await
    } else {
        handle_show_prompt(ctx).await
    }
}
/// 处理系统提示切换逻辑
async fn handle_switch_prompt(ctx: PoiseContext<'_>, file_name: String) -> crate::Result<()> {
    let (prompt, content) = {
        let app_state = APP_STATE_MANAGER.get_app_state();
        let mut app_state = app_state.exclusive_access();
        app_state.aiconfig.use_others_prompt(&file_name)?;
        APP_STATE_MANAGER.save()?;
        let (prompt_name, content) = app_state.aiconfig.get_system_prompt()?;
        (prompt_name, content)
    };
    let reply = create_ephemeral_reply(format!("已使用 {} 的提示文件\r\n {}", prompt, content));
    ctx.send(reply).await?;
    Ok(())
}

/// 自动补全程序，把config目录下的md文件过滤出来返回给客户端
async fn handle_show_prompt(ctx: PoiseContext<'_>) -> crate::Result<()> {
    let app_state = APP_STATE_MANAGER.get_app_state();
    let (prompt, content) = app_state.access().aiconfig.get_system_prompt()?;
    let reply = create_ephemeral_reply(format!("已使用 {} 的提示文件\r\n > {}", prompt, content));
    ctx.send(reply).await?;
    Ok(())
}

/// 自动补全程序，把config目录下的md文件过滤出来返回给客户端
async fn autocomplete_ai_prompt_list(
    _ctx: PoiseContext<'_>,
    partial: &str,
) -> impl Stream<Item = String> {
    let files = std::fs::read_dir(CONFIG_DIR.deref()).unwrap();
    let names = files
        .into_iter()
        .map(|dir| {
            let file_name = dir.unwrap().file_name();
            let file_name = file_name.to_str().unwrap();
            file_name.to_string()
        })
        .filter(|name| name.ends_with(".md"))
        .collect::<Vec<_>>();
    futures::stream::iter(names)
        .filter(move |name| futures::future::ready(name.starts_with(partial)))
}
