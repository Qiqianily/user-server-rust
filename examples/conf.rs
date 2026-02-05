use user_server::{
    conf::app::AppConfig, db::pgsql::init_database_pool_with_config,
    log::logger::init_logger_without_file,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = AppConfig::load()?;
    eprintln!("config:{:?}", config);
    init_logger_without_file(config.grpc_config().log_level()).await?;
    let _guard = init_database_pool_with_config(config.database()).await?;
    tracing::info!("finished");
    Ok(())
}
