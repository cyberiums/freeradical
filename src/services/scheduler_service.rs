// Scheduled Publishing Service
// Auto-publishes and unpublishes pages based on timestamps

use diesel::prelude::*;
use tokio_cron_scheduler::{Job, JobScheduler};
use actix_web::web;
use crate::models::status_enum::PageStatus;
use crate::schema::pages;
use crate::services::database_service;

use crate::services::billing_service::BillingService;

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
    let es = email_service.clone();
    let billing_job = Job::new_async("0 0 * * * *", move |_uuid, _l| {
        let es_clone = es.clone();
        Box::pin(async move {
             if let Err(e) = process_billing(es_clone).await {
                log::error!("Billing scheduling error: {}", e);
            }
        })
    })?;

    sched.add(publish_job).await?;
    sched.add(billing_job).await?;
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
    // We run this in a blocking thread because Diesel is synchronous
    let events = web::block(|| {
        let mut conn = database_service::establish_connection();
        let billing_events = BillingService::process_recurring_billing(&mut conn)?;
        
        // Resolve emails for events
        let mut notifications = Vec::new();
        use crate::models::tenant_models::{Tenant, TenantMember};
        use crate::models::user_models::User;
        // Basic query logic: Get Tenant -> Get Members -> Get User -> Get Email
        // For MVP, just try to get the first member's user email.
        
        for event in billing_events {
             // In a real app, optimize this to bulk query or joins.
             // Here we just do N+1 because volume is low.
             
             // Get members manually to avoid PooledConnection mismatch
             use crate::schema::tenant_members;
             let members = tenant_members::table
                .filter(tenant_members::tenant_id.eq(event.tenant_id))
                .load::<TenantMember>(&mut conn)
                .unwrap_or(vec![]);
             if let Some(member) = members.first() {
                 use crate::schema::users;
                 // Fetch user email
                 let user_res = users::table.find(member.user_id).first::<User>(&mut conn);
                 if let Ok(user) = user_res {
                     notifications.push((event, user.username)); // username IS email
                 }
             }
        }
        
        Ok::<Vec<(crate::services::billing_events::BillingEvent, String)>, String>(notifications)
    }).await.map_err(|e| format!("Blocking error: {}", e))??;

    if !events.is_empty() {
        log::info!("💰 Processed recurrng billing: {} events", events.len());
        
        for (event, email) in events {
            if event.event_type == "invoice_paid" {
                 let _ = email_service.send_template_email(
                    &email,
                    "Payment Successful",
                    "billing/invoice_paid",
                    &serde_json::json!({
                        "invoice_number": event.invoice_number,
                        "amount": format!("{:.2}", event.amount_cents as f64 / 100.0),
                        "date": chrono::Utc::now().format("%Y-%m-%d").to_string(),
                        "year": chrono::Utc::now().format("%Y").to_string()
                    })
                ).await;
            }
        }
    }
    
    Ok(())
}
