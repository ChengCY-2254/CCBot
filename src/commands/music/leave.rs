use crate::commands::music::utils::{update_channel_state, CURRENT_JOIN_CHANNEL};
use crate::utils::create_ephemeral_reply;
use crate::PoiseContext;
use anyhow::Context;
use poise::CreateReply;

/// 离开一个语音频道
#[poise::command(slash_command, owners_only)]
pub async fn leave(ctx: PoiseContext<'_>) -> crate::Result<()> {
    let cur_channel = super::utils::get_current_voice_channel();
    
    if let Ok(channel) = cur_channel {
        let guild_id = channel.guild_id;
        let manager = songbird::get(ctx.serenity_context())
            .await
            .with_context(|| "语音客户端初始化中")?
            .clone();

        let has_handler = manager.get(guild_id).is_some();

        if has_handler {
            if let Err(why) = manager.remove(guild_id).await {
                log::error!("Error leaving channel: {:?}", why);
                ctx.say("离开语音频道失败").await?;
                return Err(anyhow::anyhow!("离开语音频道失败"));
            };
            // 暂停播放就重置状态
            update_channel_state(ctx, "").await?;
            {
                // 离开了频道，所以可以丢弃值
                let mut current_join_channel = CURRENT_JOIN_CHANNEL.exclusive_access();
                if current_join_channel.is_some() {
                    let _ = current_join_channel.take();
                }
                super::utils::clear_current_track_handle();
            }
        }
        ctx.send(
            CreateReply::default()
                .ephemeral(true)
                .content(format!("已离开频道 {}", channel.name)),
        )
            .await?;
    } else {
        ctx.send(create_ephemeral_reply("当前未加入任何一个语音频道")).await?;
    }

    Ok(())
}