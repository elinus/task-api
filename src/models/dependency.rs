use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TaskDependency {
    pub task_id: Uuid,
    pub depends_on: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AddDependencyRequest {
    pub depends_on: Uuid,
}
#[derive(Debug, Serialize, FromRow)]
pub struct DependencyInfo {
    pub task_id: Uuid,
    pub task_title: String,
    pub depends_on: Uuid,
    pub depends_on_title: String,
    pub dependency_status: String,
}

#[derive(Debug, Serialize)]
pub struct DependencyNode {
    pub task_id: Uuid,
    pub title: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct DependencyTree {
    pub task_id: Uuid,
    pub title: String,
    pub dependencies: Vec<DependencyTree>,
    pub blocked_by: Vec<Uuid>,
}
