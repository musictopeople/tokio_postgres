use crate::constants;
use std::sync::Arc;
use tokio_postgres::{Client, Error, NoTls};

pub type SharedClient = Arc<Client>;

pub async fn create_db_client() -> Result<SharedClient, Error> {
    let database_url = std::env::var("DATABASE_URL").expect(constants::DATABASE_CONNECTION_ERROR);

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(Arc::new(client))
}
