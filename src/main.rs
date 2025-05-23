use anyhow::Context;
use discord_hub_bot::check_config_dir_exists;
use tracing::instrument;

#[instrument]
fn main() -> discord_hub_bot::Result<()> {
    // 预检查并释放配置目录，释放后退出。
    pre_check()?;
    // Load environment variables from .env file
    dotenv::from_path("config/.env").with_context(|| "Failed to load .env")?;

    tracing_subscriber::fmt::init();
    let token = std::env::var("DISCORD_TOKEN").with_context(|| "DISCORD_TOKEN not set")?;
    let runtime = discord_hub_bot::runtime();
    runtime.block_on(async {
        discord_hub_bot::run(token)
            .await
            .with_context(|| "run discord hub bot failed")
    })
}

/// 预检查配置目录
/// 如果配置目录不存在，则创建配置目录和示例配置文件，然后退出程序。
/// 如果配置目录存在，则继续执行程序。
fn pre_check() -> discord_hub_bot::Result<()> {
    const EXAMPLE_AI_CONFIG: &str = include_str!("../ai-config.json.example");
    const EXAMPLE_ENV: &str = include_str!("../.env.example");

    if let Err(_no_why) = check_config_dir_exists() {
        log::info!("Create config dir");
        std::fs::create_dir_all("config")?;
        log::info!("Create ai-config file");
        std::fs::write("config/ai-config.json", EXAMPLE_AI_CONFIG)?;
        log::info!("Create .env file");
        std::fs::write("config/.env", EXAMPLE_ENV)?;
        log::info!("Config dir created, please modify the config file and restart the program");
        println!("配置文件已创建，请修改配置文件后重启程序。.env是隐藏文件，请使用ls -a查看");
        std::process::exit(0);
    }
    Ok(())
}
