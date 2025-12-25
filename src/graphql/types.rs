// GraphQL Schema Types

use async_graphql::*;
use serde::{Serialize, Deserialize};

// Page Type
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
#[graphql(name = "Page")]
pub struct GqlPage {
    pub uuid: String,
    pub page_title: String,
    pub page_url: String,
    pub page_name: Option<String>,
    pub content: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub status: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

// Module Type
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
#[graphql(name = "Module")]
pub struct GqlModule {
    pub uuid: String,
    pub page_uuid: String,
    pub title: String,
    pub content: String,
    pub field_type: Option<String>,
}

// Media Type
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
#[graphql(name = "Media")]
pub struct GqlMedia {
    pub uuid: String,
    pub filename: String,
    pub original_filename: String,
    pub mime_type: String,
    pub file_size: i64,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub storage_path: String,
}

// Search Result Type
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct GqlSearchResult {
    pub resource_type: String,
    pub id: String,
    pub title: String,
    pub snippet: String,
}

// Input Types
#[derive(Debug, InputObject)]
pub struct CreatePageInput {
    pub page_title: String,
    pub page_url: String,
    pub page_name: Option<String>,
    pub content: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
}

#[derive(Debug, InputObject)]
pub struct UpdatePageInput {
    pub page_title: Option<String>,
    pub page_url: Option<String>,
    pub content: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, InputObject)]
pub struct PaginationInput {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

// Connection types for pagination
#[derive(Debug, SimpleObject)]
pub struct PageConnection {
    pub nodes: Vec<GqlPage>,
    pub total_count: i64,
    pub page_info: PageInfo,
}

#[derive(Debug, SimpleObject)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub has_previous_page: bool,
}
