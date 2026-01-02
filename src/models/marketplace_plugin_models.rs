use crate::schema::marketplace_plugins;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone, ToSchema)]
#[diesel(table_name = marketplace_plugins)]
pub struct MarketplacePlugin {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub version: String,
    pub file_path: String,
    pub icon_url: Option<String>,
    pub status: Option<String>,
    pub developer_id: Option<i32>,
    pub price_cents: Option<i32>,
    pub downloads_count: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Deserialize, ToSchema)]
#[diesel(table_name = marketplace_plugins)]
pub struct NewMarketplacePlugin {
    pub name: String,
    pub description: Option<String>,
    pub version: String,
    pub file_path: String,
    pub icon_url: Option<String>,
    pub status: Option<String>,
    pub developer_id: Option<i32>,
    pub price_cents: Option<i32>,
}

use crate::schema::tenant_plugins;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Clone, Serialize, Deserialize, ToSchema)]
#[diesel(belongs_to(MarketplacePlugin, foreign_key = plugin_id))]
#[diesel(table_name = tenant_plugins)]
pub struct TenantPlugin {
    pub id: i32,
    pub tenant_id: i32,
    pub plugin_id: i32,
    pub status: String,
    pub settings: Option<serde_json::Value>,
    pub installed_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = tenant_plugins)]
pub struct NewTenantPlugin {
    pub tenant_id: i32,
    pub plugin_id: i32,
    pub status: String,
    pub settings: Option<serde_json::Value>,
}
