// Page Revision Models - Simplified version
// Iteration 4, Task 2

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Debug, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::page_revisions)]
pub struct PageRevision {
    pub id: i64,
    pub page_uuid: String,
    pub revision_number: i32,
    pub page_title: String,
    pub page_url: String,
    pub page_content: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub canonical_url: Option<String>,
    pub full_snapshot: String,  // Store JSON as String, serialize on insert
    pub change_summary: Option<String>,
    pub changed_by_user_id: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::page_revisions)]
pub struct NewPageRevision {
    pub page_uuid: String,
    pub revision_number: i32,
    pub page_title: String,
    pub page_url: String,
    pub page_content: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub canonical_url: Option<String>,
    pub full_snapshot: String,  // Store JSON as String, serialize before insert
    pub change_summary: Option<String>,
    pub changed_by_user_id: Option<i32>,
}

#[derive(Serialize)]
pub struct RevisionSummary {
    pub id: i64,
    pub revision_number: i32,
    pub change_summary: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,  // Nullable in schema
    pub changed_by_user_id: Option<i32>,
}
