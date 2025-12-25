// GraphQL Query Root

use async_graphql::*;
use crate::graphql::types::*;
use crate::models::DatabasePool;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get a single page by UUID
    async fn page(&self, ctx: &Context<'_>, uuid: String) -> Result<Option<GqlPage>> {
        // Mock implementation - replace with actual database query
        Ok(Some(GqlPage {
            uuid: uuid.clone(),
            page_title: "Sample Page".to_string(),
            page_url: "/sample".to_string(),
            page_name: Some("sample-page".to_string()),
            content: Some("<p>Sample content</p>".to_string()),
            meta_title: Some("Sample Page Title".to_string()),
            meta_description: Some("Sample description".to_string()),
            status: "published".to_string(),
            created_at: Some("2025-12-24T00:00:00Z".to_string()),
            updated_at: Some("2025-12-24T00:00:00Z".to_string()),
        }))
    }
    
    /// List all pages with pagination
    async fn pages(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>
    ) -> Result<PageConnection> {
        let page = pagination.as_ref().and_then(|p| p.page).unwrap_or(1);
        let per_page = pagination.as_ref().and_then(|p| p.per_page).unwrap_or(20);
        
        // Mock implementation
        Ok(PageConnection {
            nodes: vec![
                GqlPage {
                    uuid: "page-1".to_string(),
                    page_title: "Home".to_string(),
                    page_url: "/".to_string(),
                    page_name: Some("home".to_string()),
                    content: Some("<h1>Welcome</h1>".to_string()),
                    meta_title: Some("Home Page".to_string()),
                    meta_description: None,
                    status: "published".to_string(),
                    created_at: Some("2025-12-24T00:00:00Z".to_string()),
                    updated_at: Some("2025-12-24T00:00:00Z".to_string()),
                }
            ],
            total_count: 1,
            page_info: PageInfo {
                has_next_page: false,
                has_previous_page: false,
            },
        })
    }
    
    /// Search across resources
    async fn search(
        &self,
        ctx: &Context<'_>,
        query: String,
        resources: Option<Vec<String>>
    ) -> Result<Vec<GqlSearchResult>> {
        // Mock implementation
        Ok(vec![
            GqlSearchResult {
                resource_type: "pages".to_string(),
                id: "page-1".to_string(),
                title: "Matching Page".to_string(),
                snippet: format!("Found: {}", query),
            }
        ])
    }
    
    /// Get modules for a page
    async fn modules(
        &self,
        ctx: &Context<'_>,
        page_uuid: String
    ) -> Result<Vec<GqlModule>> {
        // Mock implementation
        Ok(vec![
            GqlModule {
                uuid: "module-1".to_string(),
                page_uuid: page_uuid.clone(),
                title: "Hero Section".to_string(),
                content: "<h1>Welcome</h1>".to_string(),
                field_type: Some("wysiwyg".to_string()),
            }
        ])
    }
    
    /// Get media items
    async fn media_library(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>
    ) -> Result<Vec<GqlMedia>> {
        // Mock implementation
        Ok(vec![
            GqlMedia {
                uuid: "media-1".to_string(),
                filename: "image.jpg".to_string(),
                original_filename: "my-image.jpg".to_string(),
                mime_type: "image/jpeg".to_string(),
                file_size: 1024000,
                width: Some(1920),
                height: Some(1080),
                storage_path: "/uploads/image.jpg".to_string(),
            }
        ])
    }
}
