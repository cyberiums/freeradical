use serde::{Deserialize, Serialize};
use crate::schema::tenant_webhooks;
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct TenantWebhook {
    pub id: Uuid,
    pub tenant_id: i32,
    pub url: String,
    pub events: serde_json::Value,
    pub secret: String,
    pub is_active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = tenant_webhooks)]
pub struct NewTenantWebhook {
    pub id: Uuid,
    pub tenant_id: i32,
    pub url: String,
    pub events: serde_json::Value,
    pub secret: String,
    pub is_active: bool,
}
