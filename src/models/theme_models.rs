use crate::schema::themes;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = themes)]
pub struct Theme {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub version: String,
    pub file_path: String,
    pub thumbnail_url: Option<String>,
    pub is_active: Option<bool>,
    pub is_default: Option<bool>,
    pub tenant_id: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = themes)]
pub struct NewTheme {
    pub name: String,
    pub description: Option<String>,
    pub version: String,
    pub file_path: String,
    pub thumbnail_url: Option<String>,
    pub is_active: Option<bool>,
    pub is_default: Option<bool>,
    pub tenant_id: Option<i32>,
}
