use sqlx::PgPool;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::{
    error::{AppError, Result},
    models::{DependencyInfo, TaskDependency},
};

pub struct DependencyRepository {
    pool: PgPool,
}

impl DependencyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Add a dependency
    pub async fn add_dependency(&self, task_id: Uuid, depends_on: Uuid) -> Result<TaskDependency> {
        // Check if both tasks exist
        let task_exists =
            sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM tasks WHERE id = $1)")
                .bind(task_id)
                .fetch_one(&self.pool)
                .await?;

        let dependency_exists =
            sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM tasks WHERE id = $1)")
                .bind(depends_on)
                .fetch_one(&self.pool)
                .await?;

        if !task_exists || !dependency_exists {
            return Err(AppError::NotFound);
        }

        // Check for circular dependency BEFORE inserting
        if self.would_create_cycle(task_id, depends_on).await? {
            return Err(AppError::ValidationError(validator::ValidationErrors::new()));
        }

        // Insert dependency
        let dependency = sqlx::query_as::<_, TaskDependency>(
            r#"
            INSERT INTO task_dependencies (task_id, depends_on)
            VALUES ($1, $2)
            ON CONFLICT (task_id, depends_on) DO NOTHING
            RETURNING *
            "#,
        )
        .bind(task_id)
        .bind(depends_on)
        .fetch_one(&self.pool)
        .await?;

        Ok(dependency)
    }

    // Remove a dependency
    pub async fn remove_dependency(&self, task_id: Uuid, depends_on: Uuid) -> Result<()> {
        let result = sqlx::query!(
            "DELETE FROM task_dependencies WHERE task_id = $1 AND depends_on = $2",
            task_id,
            depends_on
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound);
        }

        Ok(())
    }

    // Get all dependencies for a task
    pub async fn get_dependencies(&self, task_id: Uuid) -> Result<Vec<DependencyInfo>> {
        let dependencies = sqlx::query_as::<_, DependencyInfo>(
            r#"
            SELECT
                td.task_id,
                t1.title as task_title,
                td.depends_on,
                t2.title as depends_on_title,
                t2.status as dependency_status
            FROM task_dependencies td
            JOIN tasks t1 ON td.task_id = t1.id
            JOIN tasks t2 ON td.depends_on = t2.id
            WHERE td.task_id = $1
            "#,
        )
        .bind(task_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(dependencies)
    }

    // Get all tasks that depend on this task (reverse dependencies)
    pub async fn get_blocked_tasks(&self, task_id: Uuid) -> Result<Vec<Uuid>> {
        let blocked = sqlx::query_scalar::<_, Uuid>(
            "SELECT task_id FROM task_dependencies WHERE depends_on = $1",
        )
        .bind(task_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(blocked)
    }

    // Check if a task can be completed (all dependencies must be completed)
    pub async fn can_complete_task(&self, task_id: Uuid) -> Result<bool> {
        let incomplete_deps = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM task_dependencies td
            JOIN tasks t ON td.depends_on = t.id
            WHERE td.task_id = $1 AND t.status != 'completed'
            "#,
        )
        .bind(task_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(incomplete_deps == 0)
    }

    // CORE ALGORITHM: Detect circular dependencies using DFS
    async fn would_create_cycle(&self, from: Uuid, to: Uuid) -> Result<bool> {
        // If adding edge from→to creates a cycle, return true

        // Get all existing dependencies
        let all_deps =
            sqlx::query_as::<_, (Uuid, Uuid)>("SELECT task_id, depends_on FROM task_dependencies")
                .fetch_all(&self.pool)
                .await?;

        // Build adjacency list (graph representation)
        let mut graph: HashMap<Uuid, Vec<Uuid>> = HashMap::new();

        for (task_id, depends_on) in all_deps {
            graph.entry(task_id).or_insert_with(Vec::new).push(depends_on);
        }

        // Add the proposed new edge
        graph.entry(from).or_insert_with(Vec::new).push(to);

        // Run DFS to detect cycle
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        self.has_cycle_dfs(&graph, from, &mut visited, &mut rec_stack)
    }

    // DFS with a recursion stack to detect cycles
    fn has_cycle_dfs(
        &self,
        graph: &HashMap<Uuid, Vec<Uuid>>,
        node: Uuid,
        visited: &mut HashSet<Uuid>,
        rec_stack: &mut HashSet<Uuid>,
    ) -> Result<bool> {
        // Mark the current node as being visited
        visited.insert(node);
        rec_stack.insert(node);

        // Visit all neighbors
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                // If a neighbor not visited, recurse
                if !visited.contains(&neighbor) {
                    if self.has_cycle_dfs(graph, neighbor, visited, rec_stack)? {
                        return Ok(true);
                    }
                }
                // If neighbor is in recursion stack, we found a cycle!
                else if rec_stack.contains(&neighbor) {
                    return Ok(true);
                }
            }
        }

        // Remove from the recursion stack before returning
        rec_stack.remove(&node);
        Ok(false)
    }
}
