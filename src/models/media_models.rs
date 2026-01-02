// Media Library Models - Aligned with actual schema
// Schema from migrations/2025-12-26-141808-0000_create_media_table

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Debug, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::media)]
pub struct Media {
    pub id: i32,
    pub uuid: String,
    pub filename: String,
    pub original_filename: String,
    pub file_path: String,
    pub mime_type: String,
    pub file_size: i64,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub alt_text: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<Option<String>>>,
    pub uploaded_by: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub tenant_id: Option<i32>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::media)]
pub struct NewMedia {
    pub uuid: String,
    pub filename: String,
    pub original_filename: String,
    pub file_path: String,
    pub mime_type: String,
    pub file_size: i64,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub alt_text: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<Option<String>>>,
    pub uploaded_by: Option<i32>,
}

// Media variants table doesn't exist in schema - commenting out
// #[derive(Queryable, Selectable, Identifiable, Associations, Debug, Serialize, Deserialize, Clone)]
// #[diesel(belongs_to(Media, foreign_key = media_id))]
// #[diesel(table_name = crate::schema::media_variants)]
// pub struct MediaVariant {
//     pub id: i32,
//     pub media_id: i32,
//     pub variant_name: String,
//     pub file_path: String,
//     pub width: Option<i32>,
//     pub height: Option<i32>,
//     pub file_size: Option<i64>,
//     pub created_at: Option<chrono::NaiveDateTime>,
// }
 
// #[derive(Insertable)]
// #[diesel(table_name = crate::schema::media_variants)]
// pub struct NewMediaVariant {
//     pub media_id: i32,
//     pub variant_name: String,
//     pub file_path: String,
//     pub width: Option<i32>,
//     pub height: Option<i32>,
//     pub file_size: Option<i64>,
// }
