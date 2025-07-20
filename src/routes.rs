use crate::db_handler::{delete_post, update_post};
use crate::{
    db::DbPool,
    db_handler::{create_post, get_post},
};
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use axum_prometheus::metrics_exporter_prometheus::PrometheusHandle;

pub fn create_router(pool: DbPool, prometheus_handle: PrometheusHandle) -> Router {
    Router::new()
        .route("/post", get(get_post))
        .route("/post", post(create_post))
        .route("/post/{id}", put(update_post))
        .route("/post/{id}", delete(delete_post))
        .route(
            "/metrics",
            get(|| async move { prometheus_handle.render() }),
        )
        .with_state(pool)
}
