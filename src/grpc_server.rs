use tonic::transport::Server;
use user_server::{
    conf::app::AppConfig,
    db::{get_global_database_pool, pgsql::init_database_pool_with_config, set_global_db},
    log::logger::init_logger_with_file,
    pb::user::user_service_server::UserServiceServer,
    service_impl::user::UserServiceImpl,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let addr = "[::1]:50001".parse()?;
    // 1. 读取配置信息
    let config = AppConfig::load()?;
    let log_level = config.grpc_config().log_level();
    // 2. 初始化日志
    let _guard = init_logger_with_file(log_level).await?;
    // 3. 初始化数据库连接池
    let db = init_database_pool_with_config(config.database()).await?;
    set_global_db(db).await?;
    // 4. 创建服务
    let srv = UserServiceImpl::new(get_global_database_pool());
    // 5. 服务地址
    let mut addr = format!("0.0.0.0:{}", config.grpc_config().port()).parse()?;
    if config.is_dev() {
        addr = format!("[::1]:{}", config.grpc_config().port()).parse()?;
    }
    tracing::info!("Starting UserService on {}", addr);
    // 6. 启动服务
    Server::builder()
        .add_service(UserServiceServer::new(srv))
        .serve(addr)
        .await?;
    Ok(())
}
