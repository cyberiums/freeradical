use serde::{Deserialize, Serialize};
use crate::schema::audit_logs;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct AuditLog {
    pub id: i32,
    pub tenant_id: Option<i32>,
    pub user_id: i32,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub details: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = audit_logs)]
pub struct NewAuditLog {
    pub tenant_id: Option<i32>,
    pub user_id: i32,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub details: Option<serde_json::Value>,
    pub ip_address: Option<String>,
}
