use user_server::{app, conf, log};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 读取配置信息
    let config = conf::get_app_config();
    // 2. 初始化日志，为了防止多线程日志写入不完整，要保留 guard，main 函数结束时释放
    let log_level = config.http_config().log_level();
    let _guard = log::logger::init_logger_with_file(log_level).await?;
    // 3. 服务地址
    let mut grpc_addr = format!(
        "http://{}:{}",
        config.grpc_config().name(),
        config.grpc_config().port()
    );
    if config.is_dev() {
        grpc_addr = format!("http://[::1]:{}", config.grpc_config().port());
    }
    tracing::info!("grpc_addr:{}", grpc_addr);
    // 4. 启动服务
    app::server::Server::new(config)
        .start_server(&grpc_addr)
        .await?;

    Ok(())
}
