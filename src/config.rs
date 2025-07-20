use anyhow::Context;
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub server_addr: SocketAddr,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv().ok();
        let database_url =
            env::var("DATABASE_URL").context("DATABASE_URL environment variable not set")?;

        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .context("Invalid PORT value")?;

        let server_addr = format!("{}:{}", host, port)
            .parse::<SocketAddr>()
            .context("Invalid server address")?;
        Ok(Config {
            database_url,
            server_addr,
        })
    }
}
