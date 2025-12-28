use crate::schema::marketplace_plugins;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
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

#[derive(Insertable, Deserialize)]
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
