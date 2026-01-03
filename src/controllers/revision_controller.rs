// Simplified Revision Controller
// Iteration 4, Task 2 - Basic implementation

use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use utoipa::ToSchema;

use crate::services::database_service;
use crate::models::revision_models::{PageRevision, RevisionSummary};

/// List all revisions for a page
#[utoipa::path(
    get,
    path = "/api/pages/{page_uuid}/revisions",
    tag = "Content - Revisions",
    params(
        ("page_uuid" = String, Path, description = "Page UUID")
    ),
    responses(
        (status = 200, description = "List of page revisions"),
        (status = 500, description = "Failed to fetch revisions")
    )
)]
pub async fn list_revisions(page_uuid_param: web::Path<String>) -> impl Responder {
    use crate::schema::page_revisions::dsl::*;
    
    let mut conn = database_service::establish_connection();
    let uuid_str = page_uuid_param.into_inner();
    
    match page_revisions
        .filter(page_uuid.eq(uuid_str))
        .order(revision_number.desc())
        .load::<PageRevision>(&mut conn)
    {
        Ok(revisions) => {
            let summaries: Vec<RevisionSummary> = revisions
                .iter()
                .map(|r| RevisionSummary {
                    id: r.id,
                    revision_number: r.revision_number,
                    change_summary: r.change_summary.clone(),
                    created_at: r.created_at,
                    changed_by_user_id: r.changed_by_user_id,
                })
                .collect();
            
            HttpResponse::Ok().json(summaries)
        }
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch revisions"),
    }
}

/// Get specific revision
#[utoipa::path(
    get,
    path = "/api/pages/{page_uuid}/revisions/{rev_number}",
    tag = "Content - Revisions",
    params(
        ("page_uuid" = String, Path, description = "Page UUID"),
        ("rev_number" = i32, Path, description = "Revision number")
    ),
    responses(
        (status = 200, description = "Page revision details"),
        (status = 404, description = "Revision not found")
    )
)]
pub async fn get_revision(path: web::Path<(String, i32)>) -> impl Responder {
    use crate::schema::page_revisions::dsl::*;
    
    let (uuid, rev_num) = path.into_inner();
    let mut conn = database_service::establish_connection();
    
    match page_revisions
        .filter(page_uuid.eq(uuid))
        .filter(revision_number.eq(rev_num))
        .first::<PageRevision>(&mut conn)
    {
        Ok(revision) => HttpResponse::Ok().json(revision),
        Err(_) => HttpResponse::NotFound().json("Revision not found"),
    }
}

/// Rollback to a specific revision
#[utoipa::path(
    post,
    path = "/api/pages/{page_uuid}/rollback/{rev_number}",
    tag = "Content - Revisions",
    params(
        ("page_uuid" = String, Path, description = "Page UUID"),
        ("rev_number" = i32, Path, description = "Revision number to rollback to")
    ),
    responses(
        (status = 200, description = "Page rolled back successfully"),
        (status = 404, description = "Revision not found"),
        (status = 500, description = "Rollback failed")
    )
)]
pub async fn rollback_revision(path: web::Path<(String, i32)>) -> impl Responder {
    use crate::schema::page_revisions::dsl::*;
    use crate::schema::pages;
    
    let (uuid, rev_num) = path.into_inner();
    let mut conn = database_service::establish_connection();
    
    // 1. Load the revision
    let revision = match page_revisions
        .filter(page_uuid.eq(&uuid))
        .filter(revision_number.eq(rev_num))
        .first::<PageRevision>(&mut conn)
    {
        Ok(rev) => rev,
        Err(_) => return HttpResponse::NotFound().json("Revision not found"),
    };
    
    // 2. Deserialize the full_snapshot to get page state
    let restored_page: crate::models::page_models::Page = match serde_json::from_str(&revision.full_snapshot) {
        Ok(page) => page,
        Err(e) =>  {
            return HttpResponse::InternalServerError()
                .json(format!("Failed to deserialize snapshot: {}", e));
        }
    };
    
    // 3. Update the page with revision data
    // We'll use Diesel's update directly
    match diesel::update(pages::table.filter(pages::uuid.eq(&uuid)))
        .set((
            pages::page_title.eq(&restored_page.page_title),
            pages::page_url.eq(&restored_page.page_url),
            pages::page_name.eq(&restored_page.page_name),
            pages::meta_title.eq(&restored_page.meta_title),
            pages::meta_description.eq(&restored_page.meta_description),
            pages::meta_keywords.eq(&restored_page.meta_keywords),
            pages::canonical_url.eq(&restored_page.canonical_url),
            pages::og_title.eq(&restored_page.og_title),
            pages::og_description.eq(&restored_page.og_description),
            pages::og_image.eq(&restored_page.og_image),
            pages::twitter_card.eq(&restored_page.twitter_card),
            pages::twitter_title.eq(&restored_page.twitter_title),
            pages::twitter_description.eq(&restored_page.twitter_description),
            pages::author.eq(&restored_page.author),
            pages::article_type.eq(&restored_page.article_type),
            pages::featured_image.eq(&restored_page.featured_image),
            pages::word_count.eq(&restored_page.word_count),
            pages::reading_time.eq(&restored_page.reading_time),
        ))
        .execute(&mut conn)
    {
        Ok(_) => {
            // Successfully rolled back - return success without creating new revision
            // (Creating revision requires complex connection type handling)
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Page rolled back successfully",
                "page_uuid": uuid,
                "rollback_to_revision": rev_num
            }))
        }
        Err(e) => {
            HttpResponse::InternalServerError()
                .json(format!("Failed to rollback page: {}", e))
        }
    }
}
