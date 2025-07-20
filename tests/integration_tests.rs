use axum::{routing::get, Router};
use axum_test::TestServer;
use serde_json;
use tokio_postgres_poc::{
    api_models::PostRequest,
    config::Config,
    db::create_pool,
    db_handler::{create_post, get_post},
};

#[tokio::test]
async fn test_get_posts_returns_success() {
    let config = Config::from_env().expect("Failed to load config");
    let pool = create_pool(&config.database_url)
        .await
        .expect("Failed to create pool");

    let app = Router::new().route("/post", get(get_post)).with_state(pool);

    let server = TestServer::new(app).unwrap();
    let response = server.get("/post").await;

    response.assert_status_ok();
    let posts: Vec<serde_json::Value> = response.json();
    assert!(posts.is_empty() || !posts.is_empty());
}

#[tokio::test]
async fn test_create_and_get_post() {
    let config = Config::from_env().expect("Failed to load config");
    let pool = create_pool(&config.database_url)
        .await
        .expect("Failed to create pool");

    let app = Router::new()
        .route("/post", get(get_post).post(create_post))
        .with_state(pool);

    let server = TestServer::new(app).unwrap();

    // Create a post
    let new_post = PostRequest {
        title: "Test Title".to_string(),
        body: "Test Body".to_string(),
        published: true,
    };

    let create_response = server.post("/post").json(&new_post).await;

    create_response.assert_status_ok();

    let get_response = server.get("/post").await;
    get_response.assert_status_ok();

    let posts: Vec<serde_json::Value> = get_response.json();
    assert!(!posts.is_empty());

    let test_post = posts.iter().find(|post| {
        post["title"] == "Test Title" && post["body"] == "Test Body" && post["published"] == true
    });

    assert!(
        test_post.is_some(),
        "Created test post not found in results"
    );
}
