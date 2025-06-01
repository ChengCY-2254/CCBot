use crate::commands::music::utils::CURRENT_JOIN_CHANNEL;
use crate::PoiseContext;
use anyhow::Context;
use poise::{async_trait, CreateReply};
use serenity::all::GuildChannel;
use songbird::{Event, EventContext, EventHandler, TrackEvent};

///加入一个语音频道
#[poise::command(slash_command, owners_only)]
pub(super) async fn join(
    ctx: PoiseContext<'_>,
    #[channel_types("Voice")] channel: GuildChannel,
) -> crate::Result<()> {
    let (guild_id, channel_id) = {
        let guild = channel.guild_id;
        let channel_id = channel.id;
        (guild, channel_id)
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .with_context(|| "语音客户端初始化中")?
        .clone();
    let handler_lock = manager.join(guild_id, channel_id).await?;
    {
        let mut current_join_channel = CURRENT_JOIN_CHANNEL.exclusive_access();
        if current_join_channel.is_none() {
            let _ = current_join_channel.replace(channel.clone());
        }
    }
    let mut handler = handler_lock.lock().await;
    handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);
    let reply = CreateReply::default()
        .ephemeral(true)
        .content(format!("已加入语音频道: {}", channel.name));
    ctx.send(reply).await?;
    Ok(())
}

struct TrackErrorNotifier;

#[async_trait]
impl EventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                log::error!(
                    "Track {:?} encountered an error: {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        }

        None
    }
}
