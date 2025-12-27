use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::languages;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Language {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub native_name: Option<String>,
    pub is_default: Option<bool>,
    pub is_rtl: Option<bool>,
    pub enabled: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = languages)]
pub struct NewLanguage {
    pub code: String,
    pub name: String,
    pub native_name: Option<String>,
    pub is_default: Option<bool>,
    pub is_rtl: Option<bool>,
    pub enabled: Option<bool>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = languages)]
pub struct UpdateLanguage {
    pub name: Option<String>,
    pub native_name: Option<String>,
    pub is_default: Option<bool>,
    pub is_rtl: Option<bool>,
    pub enabled: Option<bool>,
}
