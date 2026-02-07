use std::sync::OnceLock;

use sqlx::PgPool;

pub mod pgsql;

// 全局 Postgres 数据库连接池实例
static GLOBAL_DATABASE_POOL: OnceLock<PgPool> = OnceLock::new();
/// 获取全局的静态 Postgres 数据库连接池引用
pub fn get_global_database_pool() -> &'static PgPool {
    GLOBAL_DATABASE_POOL.get().expect("database pool lost")
}
/// 初始化全局的静态数据库
pub async fn set_global_db(db: PgPool) -> anyhow::Result<()> {
    GLOBAL_DATABASE_POOL
        .set(db)
        .map_err(|_| anyhow::anyhow!("failed to set global database pool"))
}
