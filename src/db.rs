use deadpool_postgres::{Config, Pool, Runtime};
use std::env;

pub type DbPool = Pool;

pub async fn create_pool(database_url: &str) -> Result<DbPool, Box<dyn std::error::Error>> {
    let max_connections: usize = env::var("DATABASE_MAX_CONNECTIONS")
        .unwrap_or_else(|_| "16".to_string())
        .parse()
        .unwrap_or(16);

    let timeout_seconds: u64 = env::var("DATABASE_CONNECT_TIMEOUT_SECONDS")
        .unwrap_or_else(|_| "30".to_string())
        .parse()
        .unwrap_or(30);

    let mut cfg = Config::new();
    cfg.url = Some(database_url.to_string());
    cfg.pool = Some(deadpool_postgres::PoolConfig {
        max_size: max_connections,
        timeouts: deadpool_postgres::Timeouts::wait_millis(timeout_seconds * 1000),
        ..Default::default()
    });

    cfg.create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
        .map_err(Into::into)
}
