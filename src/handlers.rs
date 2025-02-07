use crate::{db::SharedClient, queries};
use axum::{extract::State, Json};

pub async fn get_post(State(client): State<SharedClient>) -> Json<String> {
    let rows = client
        .query(queries::SELECT_POST_BY_ID, &[&1])
        .await
        .unwrap_or_default();

    let value = if !rows.is_empty() {
        rows[0].get(0)
    } else {
        "no post found"
    };

    Json(value.to_string())
}
