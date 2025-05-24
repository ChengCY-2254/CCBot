use anyhow::Context;
use cc_bot::{check_config_dir_exists, handle_file_if_not_dir};
use tracing::instrument;

#[instrument]
fn main() -> cc_bot::Result<()> {
    // 预检查并释放配置目录，释放后退出。
    pre_check()?;
    // Load environment variables from .env file
    dotenv::from_path("config/.env").with_context(|| "Failed to load .env")?;

    tracing_subscriber::fmt::init();
    let token = std::env::var("DISCORD_TOKEN").with_context(|| "DISCORD_TOKEN not set")?;
    let runtime = cc_bot::runtime();
    runtime.block_on(async {
        cc_bot::run(token)
            .await
            .with_context(|| "run discord hub bot failed")
    })
}

/// 预检查配置目录
/// 如果配置目录不存在，则创建配置目录和示例配置文件，然后退出程序。
/// 如果配置目录存在，则继续执行程序。
fn pre_check() -> cc_bot::Result<()> {
    const EXAMPLE_AI_CONFIG: &str = include_str!("../ai-config.json.example");
    const EXAMPLE_ENV: &str = include_str!("../.env.example");
    const DATA_CONFIG: &str = include_str!("../data.json.example");

    if let Err(_no_why) = check_config_dir_exists() {
        std::fs::create_dir_all("config")?;
        log::info!("Create config dir");

        handle_file_if_not_dir("config/ai-config.json", || {
            std::fs::write("config/ai-config.json", EXAMPLE_AI_CONFIG)
                .expect("Failed to write ai-config.json");
            log::info!("Create ai-config file");
        });

        handle_file_if_not_dir("config/.env", || {
            std::fs::write("config/.env", EXAMPLE_ENV).expect("Failed to write .env");
            log::info!("Create .env file");
        });

        handle_file_if_not_dir("config/data.json", || {
            std::fs::write("config/data.json", DATA_CONFIG).expect("Failed to write data.json");
            log::info!("Create config/data.json file");
        });

        log::info!("Config dir created, please modify the config file and restart the program");
        println!("配置文件已创建，请修改配置文件后重启程序。.env是隐藏文件，请使用ls -a查看");
        std::process::exit(0);
    }
    Ok(())
}
