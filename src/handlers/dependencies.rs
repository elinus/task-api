use std::collections::{HashSet, VecDeque};
use crate::{
    database::dependency_repo::DependencyRepository,
    error::Result,
    models::{AddDependencyRequest, DependencyInfo, TaskDependency},
    state::AppState,
    utils::jwt::Claims,
};
use axum::http::StatusCode;
use axum::{
    extract::{Path, State},
    {Extension, Json},
};
use uuid::Uuid;

// Add a dependency
pub async fn add_dependency(
    Extension(_claims): Extension<Claims>,
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
    Json(payload): Json<AddDependencyRequest>,
) -> Result<Json<TaskDependency>> {
    let repo = DependencyRepository::new(state.pool);

    // TODO! Verify the user owns the task or is admin

    let dependency = repo.add_dependency(task_id, payload.depends_on).await?;

    Ok(Json(dependency))
}

// Remove a dependency
pub async fn remove_dependency(
    Extension(_claims): Extension<Claims>,
    State(state): State<AppState>,
    Path((task_id, depends_on)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode> {
    let repo = DependencyRepository::new(state.pool);
    repo.remove_dependency(task_id, depends_on).await?;

    Ok(StatusCode::NO_CONTENT)
}

// Get all dependencies for a task
pub async fn get_dependencies(
    Extension(_claims): Extension<Claims>,
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
) -> Result<Json<Vec<DependencyInfo>>> {
    let repo = DependencyRepository::new(state.pool);
    let dependencies = repo.get_dependencies(task_id).await?;

    Ok(Json(dependencies))
}

// Get blocked tasks (tasks that depend on this one)
pub async fn get_blocked_tasks(
    Extension(_claims): Extension<Claims>,
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
) -> Result<Json<Vec<Uuid>>> {
    let repo = DependencyRepository::new(state.pool);
    let blocked = repo.get_blocked_tasks(task_id).await?;

    Ok(Json(blocked))
}

// Get all transitive dependencies (everything this task depends on, directly or indirectly)
pub async fn get_all_dependencies(
    Extension(_claims): Extension<Claims>,
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
) -> Result<Json<Vec<Uuid>>> {
    // BFS to find all transitive dependencies
    let mut all_deps = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(task_id);

    while let Some(current) = queue.pop_front() {
        // Get direct dependencies
        let deps = sqlx::query_scalar::<_, Uuid>(
            "SELECT depends_on FROM task_dependencies WHERE task_id = $1"
        )
            .bind(current)
            .fetch_all(&state.pool)
            .await?;

        for dep in deps {
            if all_deps.insert(dep) {
                // New dependency found, explore its dependencies
                queue.push_back(dep);
            }
        }
    }

    // Remove the task itself if it's in there
    all_deps.remove(&task_id);

    Ok(Json(all_deps.into_iter().collect()))
}