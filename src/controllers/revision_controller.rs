// Simplified Revision Controller
// Iteration 4, Task 2 - Basic implementation

use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use serde_json::json;

use crate::db_connection::establish_connection;
use crate::models::revision_models::{PageRevision, RevisionSummary};
use crate::schema::page_revisions;

/// List all revisions for a page
/// GET /api/pages/:page_uuid/revisions
pub async fn list_revisions(page_uuid: web::Path<String>) -> impl Responder {
    use crate::schema::page_revisions::dsl::*;
    
    let mut conn = establish_connection();
    
    match page_revisions
        .filter(page_uuid.eq(page_uuid.as_str()))
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
/// GET /api/pages/:page_uuid/revisions/:rev_number
pub async fn get_revision(path: web::Path<(String, i32)>) -> impl Responder {
    use crate::schema::page_revisions::dsl::*;
    
    let (uuid, rev_num) = path.into_inner();
    let mut conn = establish_connection();
    
    match page_revisions
        .filter(page_uuid.eq(uuid))
        .filter(revision_number.eq(rev_num))
        .first::<PageRevision>(&mut conn)
    {
        Ok(revision) => HttpResponse::Ok().json(revision),
        Err(_) => HttpResponse::NotFound().json("Revision not found"),
    }
}

/// Rollback to a specific revision (simplified)
/// POST /api/pages/:page_uuid/rollback/:rev_number
/// This is a placeholder - full implementation would update the page
pub async fn rollback_revision(path: web::Path<(String, i32)>) -> impl Responder {
    let (uuid, rev_num) = path.into_inner();
    
    // TODO: Implement actual rollback logic
    // Would need to:
    // 1. Load the revision
    // 2. Update the page with revision data
    // 3. Create a new revision for the rollback
    
    HttpResponse::Ok().json(json!({
        "message": "Rollback placeholder",
        "page_uuid": uuid,
        "revision_number": rev_num,
        "note": "Full implementation pending"
    }))
}
