use crate::{
    error::Result,
    models::{CreateTaskRequest, Task},
};
use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::PgPool;
use uuid::Uuid;

// List all tasks
pub async fn list_tasks(State(pool): State<PgPool>) -> Result<Json<Vec<Task>>> {
    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await?;
    Ok(Json(tasks))
}

// Get single task
pub async fn get_task(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> Result<Json<Task>> {
    let task = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or(crate::error::AppError::NotFound)?;
    Ok(Json(task))
}

// Create task
pub async fn create_task(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<Json<Task>> {
    // For now, hardcode created_by to our test user
    // We'll add auth later
    let test_user_id =
        sqlx::query_scalar::<_, Uuid>("SELECT id FROM users WHERE email = 'test@example.com'")
            .fetch_one(&pool)
            .await?;

    let task = sqlx::query_as::<_, Task>(
        r#"
            INSERT INTO tasks (title, description, priority, assigned_to, created_by, due_date)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
    )
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(payload.priority.unwrap_or_else(|| "medium".to_string()))
    .bind(payload.assigned_to)
    .bind(test_user_id)
    .bind(payload.due_date)
    .fetch_one(&pool)
    .await?;

    Ok(Json(task))
}
