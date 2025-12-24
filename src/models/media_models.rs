// Media Library Models - Simplified version
// Iteration 4, Task 1

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Debug, Serialize, Deserialize, Clone)]
#[diesel(table_name = media)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Media {
    pub id: i32,
    pub uuid: String,
    pub filename: String,
    pub original_filename: String,
    pub mime_type: String,
    pub file_size: i64,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub folder: Option<String>,
    pub storage_path: String,
    pub cdn_url: Option<String>,
    pub upload_user_id: Option<i32>,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::media)]
pub struct NewMedia {
    pub uuid: String,
    pub filename: String,
    pub original_filename: String,
    pub mime_type: String,
    pub file_size: i64,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub folder: Option<String>,
    pub storage_path: String,
    pub cdn_url: Option<String>,
    pub upload_user_id: Option<i32>,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Serialize, Deserialize, Clone)]
#[diesel(belongs_to(Media, foreign_key = media_id))]
#[diesel(table_name = media_variants)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct MediaVariant {
    pub id: i32,
    pub media_id: i32,
    pub variant_name: String,
    pub file_path: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub file_size: Option<i64>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::media_variants)]
pub struct NewMediaVariant {
    pub media_id: i32,
    pub variant_name: String,
    pub file_path: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub file_size: Option<i64>,
}
