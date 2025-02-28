use crate::{
    api_models::PostRequest, api_models::Response, db::SharedClient, error::ApiError, queries,
};
use axum::{extract::State, Json};
use tokio_postgres::Row;

pub async fn get_post(State(client): State<SharedClient>) -> Result<Json<Response>, ApiError> {
    let rows = client.query(queries::SELECT_POST_BY_ID, &[&1]).await?;

    let post = rows
        .first()
        .ok_or(ApiError::NotFound)
        .and_then(row_to_post)?;

    Ok(Json(post))
}

pub async fn create_post(
    State(client): State<SharedClient>,
    Json(payload): Json<PostRequest>,
) -> Result<Json<Response>, ApiError> {
    let row = client
        .query_one(
            queries::INSERT_POST,
            &[&payload.title, &payload.body, &payload.published],
        )
        .await?;

    let post = row_to_post(&row)?;
    Ok(Json(post))
}

fn row_to_post(row: &Row) -> Result<Response, ApiError> {
    Ok(Response {
        id: row.try_get("id").map_err(|_| ApiError::Internal)?,
        title: row.try_get("title").map_err(|_| ApiError::Internal)?,
        body: row.try_get("body").map_err(|_| ApiError::Internal)?,
        published: row.try_get("published").map_err(|_| ApiError::Internal)?,
    })
}
