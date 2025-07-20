mod api_models;
mod config;
mod db;
mod db_handler;
mod error;
mod metrics;
mod queries;
mod routes;
mod shutdown;

use anyhow::{Context, Result};
use config::Config;
use tracing::info;

use db::create_pool;
use metrics::setup_metrics;
use routes::create_router;
use shutdown::shutdown_signal;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = Config::from_env().context("Failed to load configuration")?;

    info!("Starting server with config: {:?}", config);

    let pool = create_pool(&config.database_url)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create database pool: {}", e))?;

    info!("Database pool created successfully");

    let (prometheus_layer, prometheus_handle) = setup_metrics();
    let app = create_router(pool, prometheus_handle).layer(prometheus_layer);

    let listener = tokio::net::TcpListener::bind(&config.server_addr)
        .await
        .with_context(|| format!("Failed to bind to address {}", config.server_addr))?;

    info!("Server listening on http://{}", config.server_addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("Server error")?;

    info!("Server shutdown complete");
    Ok(())
}
