// Search Service - Simplified for MySQL compatibility

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub resources: Option<Vec<String>>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize, Queryable)]
pub struct PageSearchResult {
    pub uuid: String,
    pub page_title: String,
    pub page_url: String,
    pub meta_description: Option<String>,
}

#[derive(Debug, Serialize, Queryable)]
pub struct ModuleSearchResult {
    pub uuid: String,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize, Queryable)]
pub struct MediaSearchResult {
    pub uuid: String,
    pub filename: String,
    pub original_filename: String,
    pub alt_text: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub resource_type: String,
    pub id: String,
    pub title: String,
    pub snippet: String,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub total: usize,
    pub page: i64,
    pub per_page: i64,
}

/// Simple keyword search (fallback when FTS not available)
pub fn search(
    query: &SearchQuery,
    conn: &mut MysqlConnection
) -> Result<SearchResponse, diesel::result::Error> {
    use crate::schema::{pages, modules, media};
    
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let search_term = format!("%{}%", query.q);
    
    let mut all_results: Vec<SearchResult> = Vec::new();
    
    // Search pages with LIKE (simple fallback)
    let page_results: Vec<PageSearchResult> = pages::table
        .filter(
            pages::page_title.like(&search_term)
            .or(pages::page_name.like(&search_term))
            .or(pages::meta_description.like(&search_term))
        )
        .limit(20)
        .select((pages::uuid, pages::page_title, pages::page_url, pages::meta_description))
        .load(conn)?;
    
    for p in page_results {
        all_results.push(SearchResult {
            resource_type: "pages".to_string(),
            id: p.uuid,
            title: p.page_title,
            snippet: p.meta_description.unwrap_or_default(),
        });
    }
    
    // Search modules
    let module_results: Vec<ModuleSearchResult> = modules::table
        .filter(
            modules::title.like(&search_term)
            .or(modules::content.like(&search_term))
        )
        .limit(20)
        .select((modules::uuid, modules::title, modules::content))
        .load(conn)?;
    
    for m in module_results {
        let snippet = if m.content.len() > 150 {
            format!("{}...", &m.content[..150])
        } else {
            m.content
        };
        
        all_results.push(SearchResult {
            resource_type: "modules".to_string(),
            id: m.uuid,
            title: m.title,
            snippet,
        });
    }
    
    // Search media
    let media_results: Vec<MediaSearchResult> = media::table
        .filter(
            media::filename.like(&search_term)
            .or(media::original_filename.like(&search_term))
            .or(media::alt_text.like(&search_term))
        )
        .limit(20)
        .select((media::uuid, media::filename, media::original_filename, media::alt_text))
        .load(conn)?;
    
    for m in media_results {
        all_results.push(SearchResult {
            resource_type: "media".to_string(),
            id: m.uuid,
            title: m.filename.clone(),
            snippet: m.alt_text.unwrap_or(m.original_filename),
        });
    }
    
    let total = all_results.len();
    let offset = ((page - 1) * per_page) as usize;
    let results: Vec<SearchResult> = all_results
        .into_iter()
        .skip(offset)
        .take(per_page as usize)
        .collect();
    
    Ok(SearchResponse {
        results,
        total,
        page,
        per_page,
    })
}
