use crate::{
    database::audit_repo::AuditRepository,
    error::Result,
    models::{AuditLogWithUser, AuditQuery},
    state::AppState,
    utils::jwt::Claims,
};
use axum::{
    extract::{Path, Query, State},
    {Extension, Json},
};
use uuid::Uuid;

// Get audit history for a specific task
pub async fn get_task_history(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
) -> Result<Json<Vec<AuditLogWithUser>>> {
    let repo = AuditRepository::new(state.pool);
    let logs = repo.get_resource_history("task", task_id).await?;

    Ok(Json(logs))
}

// Get user's activity
pub async fn get_user_activity(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Query(params): Query<AuditQuery>,
) -> Result<Json<Vec<AuditLogWithUser>>> {
    let user_id = claims.user_id()?;
    let repo = AuditRepository::new(state.pool);

    let limit = params.limit.unwrap_or(50);
    let logs = repo.get_user_activity(user_id, limit).await?;

    Ok(Json(logs))
}

// Admin: Get recent activity across all users
pub async fn get_recent_activity(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Query(params): Query<AuditQuery>,
) -> Result<Json<Vec<AuditLogWithUser>>> {
    // Check admin role
    if claims.role != "admin" {
        return Err(crate::error::AppError::Unauthorized("Admin access required".to_string()));
    }
    let repo = AuditRepository::new(state.pool);
    let limit = params.limit.unwrap_or(100);
    let logs = repo.get_recent_activity(limit).await?;

    Ok(Json(logs))
}
