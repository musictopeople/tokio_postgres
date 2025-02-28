use crate::{
    db::SharedClient,
    handlers::{create_post, get_post},
};
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_router(shared_client: SharedClient) -> Router {
    Router::new()
        .route("/post", get(get_post))
        .route("/post", post(create_post))
        .with_state(shared_client)
}
