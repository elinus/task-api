use crate::{
    error::Result,
    models::{CreateTaskRequest, Task, TaskQuery, UpdateTaskRequest},
    state::AppState,
    utils::jwt::Claims,
};
use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;
use validator::Validate;

// List tasks with filters
pub async fn list_tasks(
    State(state): State<AppState>,
    Query(params): Query<TaskQuery>,
) -> Result<Json<Vec<Task>>> {
    // Build a dynamic query based on filters
    let mut query = String::from("SELECT * FROM tasks WHERE 1=1");
    if params.status.is_some() {
        query.push_str(" AND status = $1");
    }
    if params.priority.is_some() {
        query.push_str(" AND status = $2");
    }
    if params.assigned_to.is_some() {
        query.push_str(" AND status = $3");
    }
    if params.created_by.is_some() {
        query.push_str(" AND status = $4");
    }
    query.push_str(" ORDER BY created_at DESC");
    let tasks = match (
        params.status,
        params.priority,
        params.assigned_to,
        params.created_by,
    ) {
        (None, None, None, None) => {
            // No filters
            sqlx::query_as::<_, Task>(
                "SELECT * FROM tasks ORDER BY created_at DESC",
            )
            .fetch_all(&state.pool)
            .await?
        }
        (Some(status), None, None, None) => {
            // Filter by status only
            sqlx::query_as::<_, Task>(
                "SELECT * FROM tasks WHERE status = $1 ORDER BY created_at DESC",
            )
            .bind(status)
            .fetch_all(&state.pool)
            .await?
        }
        (None, Some(priority), None, None) => {
            // Filter by priority only
            sqlx::query_as::<_, Task>(
                "SELECT * FROM tasks WHERE priority = $1 ORDER BY created_at DESC",
            )
            .bind(priority)
            .fetch_all(&state.pool)
            .await?
        }
        (Some(status), Some(priority), None, None) => {
            // Filter by status AND priority
            sqlx::query_as::<_, Task>(
                "SELECT * FROM tasks WHERE status = $1 AND priority = $2 ORDER BY created_at DESC",
            )
            .bind(status)
            .bind(priority)
            .fetch_all(&state.pool)
            .await?
        }
        (None, None, Some(assigned_to), None) => {
            // Filter by assigned_to
            sqlx::query_as::<_, Task>(
                "SELECT * FROM tasks WHERE assigned_to = $1 ORDER BY created_at DESC",
            )
            .bind(assigned_to)
            .fetch_all(&state.pool)
            .await?
        }
        // Add more combinations as needed...
        _ => {
            // For now, return all if combination not handled
            sqlx::query_as::<_, Task>(
                "SELECT * FROM tasks ORDER BY created_at DESC",
            )
            .fetch_all(&state.pool)
            .await?
        }
    };

    Ok(Json(tasks))
}

// Get single task
pub async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Task>> {
    let task = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or(crate::error::AppError::NotFound)?;
    Ok(Json(task))
}

// Create task
pub async fn create_task(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<Json<Task>> {
    // Validate input
    payload.validate()?;

    let created_by = claims.user_id()?;

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
    .bind(created_by)
    .bind(payload.due_date)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(task))
}

// Update task
pub async fn update_task(
    Extension(_claims): Extension<Claims>,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTaskRequest>,
) -> Result<Json<Task>> {
    // First, check if a task exists
    let existing =
        sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.pool)
            .await?
            .ok_or(crate::error::AppError::NotFound)?;

    // Build dynamic update query
    // let mut query_parts = Vec::new();
    // let mut param_count = 1;

    // We'll use a different approach - update all fields
    let title = payload.title.unwrap_or(existing.title);
    let description = payload.description.or(existing.description);
    let status = payload.status.unwrap_or(existing.status);
    let priority = payload.priority.unwrap_or(existing.priority);
    let assigned_to = payload.assigned_to.or(existing.assigned_to);
    let due_date = payload.due_date.or(existing.due_date);

    let task = sqlx::query_as::<_, Task>(
        r#"
        UPDATE tasks
        SET title = $1, description = $2, status = $3, priority = $4,
        assigned_to = $5, due_date = $6, updated_at = NOW()
        WHERE id = $7
        RETURNING *"#,
    )
    .bind(title)
    .bind(description)
    .bind(status)
    .bind(priority)
    .bind(assigned_to)
    .bind(due_date)
    .bind(id)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(task))
}

// Delete task
pub async fn delete_task(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    let user_id = claims.user_id()?;
    let result = sqlx::query!(
        r#"DELETE FROM tasks WHERE id = $1 AND created_by = $2"#,
        id,
        user_id
    )
    .execute(&state.pool)
    .await?;
    if result.rows_affected() == 0 {
        return Err(crate::error::AppError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}

// Admin-only: Delete ANY task
pub async fn admin_delete_any_task(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    // Middleware already checked a role, but let's be explicit
    if claims.role != "admin" {
        return Err(crate::error::AppError::Unauthorized(
            "Admin access required".to_string(),
        ));
    }

    let result = sqlx::query!(r#"
        DELETE FROM tasks
        WHERE id = $1
        "#,
        id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(crate::error::AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
