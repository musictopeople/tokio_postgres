use crate::{db::SharedClient, handlers};
use axum::{routing::get, Router};

pub fn create_router(shared_client: SharedClient) -> Router {
    Router::new()
        .route("/post", get(handlers::get_post))
        .with_state(shared_client)
}
