use anyhow::Context;
use tracing::instrument;

#[instrument]
fn main() -> discord_hub_bot::Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv()
        .with_context(|| "Failed to load .env")?;
    // Initialize the logger
    tracing_subscriber::fmt::init();
    let token = std::env::var("DISCORD_TOKEN").with_context(|| "DISCORD_TOKEN not set")?;
    let runtime = discord_hub_bot::runtime();
    runtime.block_on(async {
        discord_hub_bot::run(token)
            .await
            .with_context(|| "run discord hub bot failed")
    })
}
