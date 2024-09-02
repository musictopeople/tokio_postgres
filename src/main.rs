use dotenv::dotenv;
use tokio_postgres::{Error, NoTls};

mod constants;
mod queries;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // load environment variables from `.env` file
    dotenv().ok();
    // set temporary variable to connect db using url
    let database_url = std::env::var("DATABASE_URL").expect(constants::DATABASE_CONNECTION_ERROR);
    // set url for postgres db
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client.query(queries::SELECT_POST_BY_ID, &[&1]).await?;

    let value: &str = rows[0].get(0);
    println!("\n{}!!!!!\n", value);
    Ok(())
}
