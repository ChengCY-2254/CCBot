use anyhow::Context;
use tracing::instrument;

#[instrument]
fn main() -> discord_hub_bot::Result<()> {
    let runtime = discord_hub_bot::runtime();
    runtime.block_on(async {
        discord_hub_bot::run()
            .await
            .with_context(|| "run discord hub bot failed")
    })
}
