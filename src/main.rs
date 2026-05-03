use tracing::info;
use RDBMS_from_scratch::interface::api::{start_server, ServerConfig};
use RDBMS_from_scratch::VERSION;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ロガーを初期化
    tracing_subscriber::fmt::init();

    info!("Starting DataBase v{}", VERSION);

    // サーバ設定 (デフォルト： localhost:8000)
    let config = ServerConfig::default();

    // サーバを起動
    start_server(config).await?;

    Ok(())
}
