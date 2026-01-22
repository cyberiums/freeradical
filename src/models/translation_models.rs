use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::page_translations;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct PageTranslation {
    pub id: i32,
    pub page_id: String,
    pub language_id: i32,
    pub page_title: Option<String>,
    pub page_content: Option<String>,
    pub page_url: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub twitter_title: Option<String>,
    pub twitter_description: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = page_translations)]
pub struct NewPageTranslation {
    pub page_id: String,
    pub language_id: i32,
    pub page_title: Option<String>,
    pub page_content: Option<String>,
    pub page_url: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
}
