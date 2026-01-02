use serde::{Deserialize, Serialize};
use crate::schema::tenant_webhooks;
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct TenantWebhook {
    pub id: Uuid,
    pub tenant_id: i32,
    pub url: String,
    pub secret: String,
    pub events: serde_json::Value,
    pub is_active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug, ToSchema)]
#[diesel(table_name = tenant_webhooks)]
pub struct NewTenantWebhook {
    pub id: Uuid,
    pub tenant_id: i32,
    pub url: String,
    pub secret: String,
    pub events: serde_json::Value,
    pub is_active: bool,
}
