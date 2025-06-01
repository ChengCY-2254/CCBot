use crate::PoiseContext;

/// 注册命令的命令，需要使用@`[yourbot]` register来进行使用
#[poise::command(prefix_command, aliases("reg"), owners_only)]
pub(super) async fn register(ctx: PoiseContext<'_>) -> crate::Result<()> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}