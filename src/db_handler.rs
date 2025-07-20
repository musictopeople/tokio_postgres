use crate::{api_models::PostRequest, api_models::Response, db::DbPool, error::ApiError, queries};
use axum::{
    extract::{Path, State},
    Json,
};
use axum_prometheus::metrics;
use tokio_postgres::Row;

pub async fn get_post(State(pool): State<DbPool>) -> Result<Json<Vec<Response>>, ApiError> {
    let client = pool.get().await.map_err(|_| ApiError::Internal)?;

    metrics::counter!("db_queries_total", "operation" => "select").increment(1);
    let start = std::time::Instant::now();

    let rows = client.query(queries::SELECT_POSTS, &[]).await?;

    metrics::histogram!("db_query_duration_seconds", "operation" => "select")
        .record(start.elapsed().as_secs_f64());

    let post = rows_to_posts(rows)?;
    Ok(Json(post))
}

pub async fn create_post(
    State(pool): State<DbPool>,
    Json(payload): Json<PostRequest>,
) -> Result<Json<Response>, ApiError> {
    let client = pool.get().await.map_err(|_| ApiError::Internal)?;

    // Record DB metrics
    metrics::counter!("db_queries_total", "operation" => "insert").increment(1);
    let start = std::time::Instant::now();

    let row = client
        .query_one(
            queries::INSERT_POST,
            &[&payload.title, &payload.body, &payload.published],
        )
        .await?;

    metrics::histogram!("db_query_duration_seconds", "operation" => "insert")
        .record(start.elapsed().as_secs_f64());

    let post = row_to_post(&row)?;
    Ok(Json(post))
}

pub async fn update_post(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(payload): Json<PostRequest>,
) -> Result<Json<Response>, ApiError> {
    let client = pool.get().await.map_err(|_| ApiError::Internal)?;

    // Record DB metrics
    metrics::counter!("db_queries_total", "operation" => "update").increment(1);
    let start = std::time::Instant::now();

    let row = client
        .query_one(
            queries::UPDATE_POST,
            &[&payload.title, &payload.body, &payload.published, &id],
        )
        .await?;

    metrics::histogram!("db_query_duration_seconds", "operation" => "update")
        .record(start.elapsed().as_secs_f64());

    let post = row_to_post(&row)?;
    Ok(Json(post))
}

pub async fn delete_post(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<Json<Response>, ApiError> {
    let client = pool.get().await.map_err(|_| ApiError::Internal)?;

    // Record DB metrics
    metrics::counter!("db_queries_total", "operation" => "delete").increment(1);
    let start = std::time::Instant::now();

    let row = client.query_one(queries::DELETE_POST, &[&id]).await?;

    metrics::histogram!("db_query_duration_seconds", "operation" => "delete")
        .record(start.elapsed().as_secs_f64());

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

fn rows_to_posts(rows: Vec<Row>) -> Result<Vec<Response>, ApiError> {
    rows.iter().map(|row| row_to_post(row)).collect()
}
