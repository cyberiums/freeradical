// Scheduled Publishing Service
// Auto-publishes and unpublishes pages based on timestamps

use diesel::prelude::*;
use tokio_cron_scheduler::{Job, JobScheduler};
// use crate::models::status_enum::PageStatus; // Temporarily disabled - enum not in schema
use crate::schema::pages;
use crate::services::database_service;

/// Initialize and start the scheduled publishing scheduler
/// Runs every 1 minute to check for pages that need status changes
pub async fn start_scheduler() -> Result<JobScheduler, Box<dyn std::error::Error>> {
    let sched = JobScheduler::new().await?;

    // Job runs every minute: "0 * * * * *" (sec min hour day month dow)
    let publish_job = Job::new_async("0 * * * * *", |_uuid, _l| {
        Box::pin(async {
            if let Err(e) = process_scheduled_pages().await {
                log::error!("Scheduled publishing error: {}", e);
            }
        })
    })?;

    sched.add(publish_job).await?;
    sched.start().await?;
    
    log::info!("📅 Scheduled publishing service started - checking every minute");
    Ok(sched)
}

/// Process pages that need status transitions based on timestamps
async fn process_scheduled_pages() -> Result<(), diesel::result::Error> {
    let mut conn = database_service::establish_connection();
    let now = chrono::Utc::now().naive_utc();
    
    // Auto-publish: scheduled → published
    // TODO: Re-enable when PageStatus enum is added back
    let published_count = diesel::update(
        pages::table
            .filter(pages::status.eq(Some(crate::models::status_enum::PageStatus::Scheduled)))
            .filter(pages::publish_at.le(now).and(pages::publish_at.is_not_null()))
    )
    .set(pages::status.eq(Some(crate::models::status_enum::PageStatus::Published)))
    .execute(&mut conn)?;
    
    if published_count > 0 {
        log::info !("✅ Auto-published {} page(s)", published_count);
    }
    
    // Auto-unpublish: published → archived
    let archived_count = diesel::update(
        pages::table
            .filter(pages::status.eq(Some(crate::models::status_enum::PageStatus::Published)))
            .filter(pages::unpublish_at.le(now).and(pages::unpublish_at.is_not_null()))
    )
    .set(pages::status.eq(Some(crate::models::status_enum::PageStatus::Archived)))
    .execute(&mut conn)?;
    
    if archived_count > 0 {
        log::info!("📦 Auto-archived {} page(s)", archived_count);
    }
    
    Ok(())
}
