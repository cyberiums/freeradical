// Helper function to create a page revision - PostgreSQL only
use diesel::prelude::*;
use crate::models::page_models::Page;
use crate::models::revision_models::NewPageRevision;
use crate::schema::{pages, page_revisions};

/// Creates a new revision record for a page
/// 
/// # Arguments
/// * `page_uuid` - UUID of the page being revised
/// * `user_id` - ID of the user making the change (from Claims)
/// * `change_summary` - Optional description of what changed
/// * `conn` - Database connection
///
/// # Returns
/// Result with the new revision number or database error
pub fn create_page_revision(
    page_uuid: &str,
    user_id: Option<i32>,
    change_summary: Option<String>,
    conn: &mut crate::models::PooledDatabaseConnection,
) -> Result<i32, diesel::result::Error> {
    use crate::schema::pages::dsl::*;
    use crate::schema::page_revisions::dsl::page_revisions as pr;
    
    // Get current page state
    let current_page: Page = pages.filter(uuid.eq(page_uuid)).first::<Page>(conn)?;
    
    // Get current revision number (or start at 0)
    let new_revision_number = current_page.current_revision.unwrap_or(0) + 1;
    
    // Serialize full page to JSON
    let full_snapshot = serde_json::to_string(&current_page)
        .unwrap_or_else(|_| "{}".to_string());
    
    // Create revision record
    let new_revision = NewPageRevision {
        page_uuid: page_uuid.to_string(),
        revision_number: new_revision_number,
        page_title: current_page.page_title.clone(),
        page_url: current_page.page_url.clone(),
        page_content: None,
        meta_title: current_page.meta_title.clone(),
        meta_description: current_page.meta_description.clone(),
        meta_keywords: current_page.meta_keywords,
        canonical_url: current_page.canonical_url,
        full_snapshot,
        change_summary,
        changed_by_user_id: user_id,
    };
    
    // Insert revision
    diesel::insert_into(pr).values(&new_revision).execute(conn)?;
    
    // Update page's current_revision counter
    diesel::update(pages.filter(uuid.eq(page_uuid)))
        .set((
            current_revision.eq(new_revision_number),
            last_modified_by.eq(user_id),
        ))
        .execute(conn)?;
    
    Ok(new_revision_number)
}
