mod api_models;
mod constants;
mod db;
mod error;
mod handlers;
mod queries;
mod routes;

use db::create_db_client;
use routes::create_router;

#[tokio::main]
async fn main() -> Result<(), tokio_postgres::Error> {
    dotenv::dotenv().ok();

    let shared_client = create_db_client().await?;

    let server = create_router(shared_client);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("server running on http:localhost:8080");

    axum::serve(listener, server).await.unwrap();
    Ok(())
}
