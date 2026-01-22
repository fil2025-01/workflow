use axum::{
    extract::{State},
    response::{IntoResponse, Json as AxumJson},
    http::StatusCode,
};
use sqlx::PgPool;
use crate::models::dtos::TaskGroup;

// Handler to get task groups
pub async fn get_groups(State(pool): State<PgPool>) -> impl IntoResponse {
    match get_groups_inner(pool).await {
        Ok(groups) => AxumJson(groups).into_response(),
        Err(e) => {
            eprintln!("Database error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_groups_inner(pool: PgPool) -> Result<Vec<TaskGroup>, sqlx::Error> {
    sqlx::query_as!(
        TaskGroup,
        "SELECT id, name, description, ordering FROM task_groups ORDER BY ordering ASC"
    )
    .fetch_all(&pool)
    .await
}
