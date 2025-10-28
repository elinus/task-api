use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::JsonValue;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AuditLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Uuid,
    pub old_value: Option<JsonValue>,
    pub new_value: Option<JsonValue>,
    pub ip_address: Option<std::net::IpAddr>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AuditLogWithUser {
    pub id: Uuid,
    pub user_email: String,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Uuid,
    pub old_value: Option<JsonValue>,
    pub new_value: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct AuditQuery {
    pub limit: Option<i64>,
}
