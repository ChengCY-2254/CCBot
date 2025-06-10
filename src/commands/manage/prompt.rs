use crate::PoiseContext;
use crate::config::data_config::APP_STATE_MANAGER;
use crate::utils::create_ephemeral_reply;
use futures::Stream;
use futures::StreamExt;
#[poise::command(
    slash_command,
    subcommands("switch_system_prompt", "put_prompt_file", "delete_prompt_file")
)]
pub(super) async fn prompt(_ctx: PoiseContext<'_>) -> crate::Result<()> {
    Ok(())
}

/// 切换系统提示
/// 自动补全程序需要给出路径下的md文件位置
#[poise::command(slash_command, rename = "switch", owners_only)]
async fn switch_system_prompt(
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
#[poise::command(slash_command, rename = "上传", owners_only)]
/// 上传一份提示，并存到config目录下
async fn put_prompt_file(
    ctx: PoiseContext<'_>,
    #[description = "提示名，不需要带后缀"] role_name: String,
    #[description = "提示内容"] content: String,
) -> crate::Result<()> {
    // 要检查config目录下是否有相同文件
    // 创建文件
    // 检查file_name是否符合格式
    if role_name.ends_with(".md") {
        return Err(anyhow::anyhow!("角色名不符合要求"));
    }
    // 预检查config目录下是否有相同文件
    let file_path = format!("config/{}.md", role_name);
    if std::fs::metadata(&file_path).is_ok() {
        return Err(anyhow::anyhow!("文件已存在"));
    }
    std::fs::write(file_path, content)?;
    ctx.reply("上传成功").await?;
    Ok(())
}

#[poise::command(slash_command, rename = "删除", owners_only)]
/// 删除角色
async fn delete_prompt_file(
    ctx: PoiseContext<'_>,
    #[description = "角色名"]
    #[autocomplete = "autocomplete_ai_prompt_list"]
    file_name: String,
) -> crate::Result<()> {
    if !file_name.ends_with(".md") {
        return Err(anyhow::anyhow!("文件名不符合要求，需以.md结尾"));
    }
    let file_path = format!("config/{}", file_name);
    if std::fs::remove_file(file_path).is_ok() {
        ctx.reply(format!("删除**{file_name}**成功")).await?;
    } else {
        ctx.reply(format!("删除**{file_name}**失败")).await?;
    }
    Ok(())
}

/// 处理系统提示切换逻辑
async fn handle_switch_prompt(ctx: PoiseContext<'_>, file_name: String) -> crate::Result<()> {
    let (prompt, content) = {
        let app_state = APP_STATE_MANAGER.get_app_state();
        let mut app_state = app_state.exclusive_access();
        app_state.aiconfig.use_others_prompt(&file_name)?;
        let (prompt_name, content) = app_state.aiconfig.get_system_prompt()?;
        (prompt_name, content)
    };
    APP_STATE_MANAGER.save()?;
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
    let files = std::fs::read_dir("config").unwrap();
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
