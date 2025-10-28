use sqlx::{PgPool, types::JsonValue};
use uuid::Uuid;

use crate::{
    error::Result,
    models::{AuditLog, AuditLogWithUser},
};

pub struct AuditRepository {
    pool: PgPool,
}

impl AuditRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Log an action
    pub async fn log_action(
        &self,
        user_id: Uuid,
        action: &str,
        resource_type: &str,
        resource_id: Uuid,
        old_values: Option<JsonValue>,
        new_values: Option<JsonValue>,
        ip_address: Option<std::net::IpAddr>,
        user_agent: Option<String>,
    ) -> Result<AuditLog> {
        let log = sqlx::query_as::<_, AuditLog>(
            r#"
            INSERT INTO audit_logs
            (user_id, action, resource_type, resource_id, old_values, new_values, ip_address, user_agent)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#
        )
            .bind(user_id)
            .bind(action)
            .bind(resource_type)
            .bind(resource_id)
            .bind(old_values)
            .bind(new_values)
            .bind(ip_address)
            .bind(user_agent)
            .fetch_one(&self.pool)
            .await?;

        Ok(log)
    }

    // Get audit logs for a specific resource
    pub async fn get_resource_history(
        &self,
        resource_type: &str,
        resource_id: Uuid,
    ) -> Result<Vec<AuditLogWithUser>> {
        let logs = sqlx::query_as::<_, AuditLogWithUser>(
            r#"
            SELECT
                al.id,
                u.email as user_email,
                al.action,
                al.resource_type,
                al.resource_id,
                al.old_values,
                al.new_values,
                al.created_at
            FROM audit_logs al
            JOIN users u ON al.user_id = u.id
            WHERE al.resource_type = $1 AND al.resource_id = $2
            ORDER BY al.created_at DESC
            "#,
        )
        .bind(resource_type)
        .bind(resource_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(logs)
    }

    // Get user's activity
    pub async fn get_user_activity(
        &self,
        user_id: Uuid,
        limit: i64,
    ) -> Result<Vec<AuditLogWithUser>> {
        let logs = sqlx::query_as::<_, AuditLogWithUser>(
            r#"
            SELECT
                al.id,
                u.email as user_email,
                al.action,
                al.resource_type,
                al.resource_id,
                al.old_values,
                al.new_values,
                al.created_at
            FROM audit_logs al
            JOIN users u ON al.user_id = u.id
            WHERE al.user_id = $1
            ORDER BY al.created_at DESC
            LIMIT $2
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(logs)
    }

    // Get recent activity across all users (admin only)
    pub async fn get_recent_activity(&self, limit: i64) -> Result<Vec<AuditLogWithUser>> {
        let logs = sqlx::query_as::<_, AuditLogWithUser>(
            r#"
            SELECT
                al.id,
                u.email as user_email,
                al.action,
                al.resource_type,
                al.resource_id,
                al.old_values,
                al.new_values,
                al.created_at
            FROM audit_logs al
            JOIN users u ON al.user_id = u.id
            ORDER BY al.created_at DESC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(logs)
    }
}
