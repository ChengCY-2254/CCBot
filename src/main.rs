use anyhow::anyhow;
use cc_bot::utils::{check_config_dir_exists, create_file_and_process_if_missing};
use std::io::Write;
use tracing::instrument;

#[instrument]
fn main() -> cc_bot::Result<()> {
    tracing_subscriber::fmt::init();
    println_message();
    // 预检查并释放配置目录，释放后退出。
    pre_check()?;
    // Load environment variables from .env file
    dotenv::from_path("config/.env").map_err(|why| {
        let msg = format!("获取.env 文件失败，请检查报错信息: {}", why);
        log::error!("{}", &msg);
        anyhow!(msg)
    })?;

    let token = std::env::var("DISCORD_TOKEN")
        .map_err(|why| anyhow!("discord token未设置？请检查 config/.env 文件 \r\n{why}"))?;
    if token.is_empty() {
        log::error!("discord token未设置？请检查 config/.env 文件");
        println!("discord token未设置？请检查 config/.env 文件");
        std::process::exit(1);
    }
    log::info!("discord token: {}", token);
    let runtime = cc_bot::utils::runtime();
    runtime.block_on(async {
        cc_bot::run(token)
            .await
            .map_err(|why| anyhow!("run cc-bot failed because:\r\n{why}"))
    })
}

/// 预检查配置目录
/// 如果配置目录不存在，则创建配置目录和示例配置文件，然后退出程序。
/// 如果配置目录存在，则继续执行程序。
fn pre_check() -> cc_bot::Result<()> {
    const EXAMPLE_ENV: &str = include_str!("../.env.example");
    const DATA_CONFIG: &str = include_str!("../data.json.example");
    const AI_CONFIG: &str = include_str!("../奶盖波波糖.md");

    if let Err(_no_why) = check_config_dir_exists() {
        std::fs::create_dir_all("config")?;
        log::info!("Create config dir");

        create_file_and_process_if_missing("config/.env", |mut file| {
            let res = write!(file, "{}", EXAMPLE_ENV)
                .map_err(|why| anyhow!("Failed to write .env\r\nbeacuse {why}"));
            log::info!("Create config/.env file");
            res
        })?;

        create_file_and_process_if_missing("config/data.json", |mut file| {
            let res = write!(file, "{}", DATA_CONFIG)
                .map_err(|why| anyhow!("Failed to write data.json\r\nbeacuse {why}"));
            log::info!("Create config/data.json file");
            res
        })?;

        create_file_and_process_if_missing("config/奶盖波波糖.md", |mut file| {
            let res = write!(file, "{}", AI_CONFIG)
                .map_err(|why| anyhow!("Failed to write 奶盖波波糖.md\r\nbeacuse {why}"));
            log::info!("Create config/奶盖波波糖.md file");
            res
        })?;

        log::info!("Config dir created, please modify the config file and restart the program");
        println!("配置文件已创建，请修改配置文件后重启程序。\r\\n.env是隐藏文件，请使用ls -a查看");
        std::process::exit(0);
    }
    Ok(())
}

fn println_message() {
    println!(
        "欢迎使用CC-Bot 当前版本:{}，更多信息请查看",
        cc_bot::VERSION
    );
    println!("https://github.com/ChengCY-2254/CCBot");
    println!("如果你有任何问题，请在GitHub上提交issue");
}
