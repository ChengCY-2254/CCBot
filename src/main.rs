use anyhow::anyhow;
use tracing::instrument;
use cc_bot::utils::{check_config_dir_exists, handle_file_if_not_dir};

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
    let runtime = cc_bot::utils::runtime();
    runtime.block_on(async {
        cc_bot::run(token)
            .await
            .map_err(|why| anyhow!("run discord hub bot failed \r\n{why}"))
    })
}

/// 预检查配置目录
/// 如果配置目录不存在，则创建配置目录和示例配置文件，然后退出程序。
/// 如果配置目录存在，则继续执行程序。
fn pre_check() -> cc_bot::Result<()> {
    const EXAMPLE_ENV: &str = include_str!("../.env.example");
    const DATA_CONFIG: &str = include_str!("../data.json.example");

    if let Err(_no_why) = check_config_dir_exists() {
        std::fs::create_dir_all("config")?;
        log::info!("Create config dir");

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

fn println_message() {
    println!(
        "欢迎使用CC-Bot 当前版本:{}，更多信息请查看",
        cc_bot::VERSION
    );
    println!("https://github.com/ChengCY-2254/CCBot");
    println!("如果你有任何问题，请在GitHub上提交issue");
}
