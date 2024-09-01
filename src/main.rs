use dotenv::dotenv;
use tokio_postgres::{Error, NoTls};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // load environment variables from `.env` file
    dotenv().ok();
    // set temporary variable to connect db using url
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    // set url for postgres db
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client
        .query("select body from posts where id = $1", &[&1])
        .await?;

    let value: &str = rows[0].get(0);
    println!("\n{}!!!!!\n", value); // Create a new Actix Web application
    Ok(())
}
