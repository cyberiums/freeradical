// Scheduled Publishing Service
// Auto-publishes and unpublishes pages based on timestamps

use diesel::prelude::*;
use tokio_cron_scheduler::{Job, JobScheduler};
use actix_web::web;
use crate::models::status_enum::PageStatus;
use crate::schema::pages;
use crate::services::database_service;

// use crate::services::billing_service::BillingService; // Commented - billing disabled

use crate::services::email_service::EmailService;

/// Initialize and start the scheduled publishing scheduler
/// Runs every 1 minute to check for pages that need status changes
pub async fn start_scheduler(email_service: web::Data<EmailService>) -> Result<JobScheduler, Box<dyn std::error::Error>> {
    let sched = JobScheduler::new().await?;

    // Job runs every minute: "0 * * * * *" (sec min hour day month dow)
    let publish_job = Job::new_async("0 * * * * *", |_uuid, _l| {
        Box::pin(async {
            if let Err(e) = process_scheduled_pages().await {
                log::error!("Scheduled publishing error: {}", e);
            }
        })
    })?;

    // Job runs every hour: "0 0 * * * *"
    // Billing job disabled - billing_service not available
    /*
    let es = email_service.clone();
    let billing_job = Job::new_async("0 0 * * * *", move |_uuid, _l| {
        let es_clone = es.clone();
        Box::pin(async move {
             if let Err(e) = process_billing(es_clone).await {
                log::error!("Billing scheduling error: {}", e);
            }
        })
    })?;
    */

    sched.add(publish_job).await?;
    // sched.add(billing_job).await?; // Commented - billing disabled
    sched.start().await?;
    
    log::info!("📅 Scheduled publishing & billing service started");
    Ok(sched)
}

/// Process pages that need status transitions based on timestamps
async fn process_scheduled_pages() -> Result<(), diesel::result::Error> {
    let mut conn = database_service::establish_connection();
    let now = chrono::Utc::now().naive_utc();
    
    // Auto-publish: scheduled → published
    let published_count = diesel::update(
        pages::table
            .filter(pages::status.eq(Some(PageStatus::Scheduled)))
            .filter(pages::publish_at.le(now).and(pages::publish_at.is_not_null()))
    )
    .set(pages::status.eq(Some(PageStatus::Published)))
    .execute(&mut conn)?;
    
    if published_count > 0 {
        log::info!("✅ Auto-published {} page(s)", published_count);
    }
    
    // Auto-unpublish: published → archived
    let archived_count = diesel::update(
        pages::table
            .filter(pages::status.eq(Some(PageStatus::Published)))
            .filter(pages::unpublish_at.le(now).and(pages::unpublish_at.is_not_null()))
    )
    .set(pages::status.eq(Some(PageStatus::Archived)))
    .execute(&mut conn)?;
    
    if archived_count > 0 {
        log::info!("📦 Auto-archived {} page(s)", archived_count);
    }
    
    Ok(())
}

async fn process_billing(email_service: web::Data<EmailService>) -> Result<(), String> {
    // Billing service disabled - function body commented out
    /*
    let events = web::block(|| {
        let mut conn = database_service::establish_connection();
        let billing_events = BillingService::process_recurring_billing(&mut conn)?;
        
        // ... rest of function
    }).await.map_err(|e| format!("Blocking error: {}", e))??;
    */
    
    log::info!("💰 Billing processing skipped - billing_service disabled");
    Ok(())
}
