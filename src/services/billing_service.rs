use actix_web::web;
use diesel::prelude::*;
use crate::models::{
    billing_models::{BillingPlan, Subscription, NewSubscription, Invoice, NewInvoice},
    db_connection::DatabasePool,
};
use crate::schema::{billing_plans, billing_subscriptions, billing_invoices};

#[derive(Clone)]
pub struct BillingService;

impl BillingService {
    pub fn new() -> Self {
        BillingService
    }

    pub fn get_plans(pool: &web::Data<DatabasePool>) -> Result<Vec<BillingPlan>, String> {
        let mut conn = pool.get().map_err(|e| format!("DB Connection Error: {}", e))?;
        
        billing_plans::table
            .filter(billing_plans::is_active.eq(true))
            .load::<BillingPlan>(&mut conn)
            .map_err(|e| format!("Error loading plans: {}", e))
    }

    pub fn subscribe_tenant(
        pool: &web::Data<DatabasePool>,
        tenant_id: i32,
        plan_code: &str,
    ) -> Result<Subscription, String> {
        let mut conn = pool.get().map_err(|e| format!("DB Connection Error: {}", e))?;

        // 1. Get Plan
        let plan = billing_plans::table
            .filter(billing_plans::code.eq(plan_code))
            .first::<BillingPlan>(&mut conn)
            .map_err(|_| format!("Plan '{}' not found", plan_code))?;

        // 2. Check existing active subscription
        let existing = billing_subscriptions::table
            .filter(billing_subscriptions::tenant_id.eq(tenant_id))
            .filter(billing_subscriptions::status.eq("active"))
            .first::<Subscription>(&mut conn)
            .optional() // requires diesel features = ["r2d2"] or similar, standard Result doesn't have optional. 
            .map_err(|e| format!("Error checking subscription: {}", e))?;

        if existing.is_some() {
            return Err("Tenant already has an active subscription. Cancel it first.".to_string());
        }

        // 3. Create Subscription
        let now = chrono::Local::now().naive_local();
        // Calculate period end based on interval
        let period_end = if plan.billing_interval == "year" {
            now + chrono::Duration::days(365)
        } else {
            now + chrono::Duration::days(30)
        };

        let new_sub = NewSubscription {
            tenant_id,
            plan_id: plan.id,
            status: "active".to_string(),
            current_period_start: now,
            current_period_end: period_end,
            provider_subscription_id: None, // Logic for Stripe ID would go here
        };

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            let created_sub = diesel::insert_into(billing_subscriptions::table)
                .values(&new_sub)
                .get_result::<Subscription>(conn)?;

            // 4. Generate Initial Invoice (Draft/Pending)
            let new_inv = NewInvoice {
                subscription_id: created_sub.id,
                amount_cents: plan.price_cents,
                status: "open".to_string(),
                due_date: now + chrono::Duration::hours(24),
                line_items: Some(serde_json::json!([{
                    "description": format!("Subscription to {} Plan", plan.name),
                    "amount": plan.price_cents
                }])),
                invoice_number: Some(format!("INV-{}-{}", tenant_id, now.and_utc().timestamp())),
            };

            diesel::insert_into(billing_invoices::table)
                .values(&new_inv)
                .execute(conn)?;

            Ok(created_sub)
        }).map_err(|e| format!("Transaction failed: {}", e))
    }

    pub fn cancel_subscription(
        pool: &web::Data<DatabasePool>,
        tenant_id: i32,
    ) -> Result<usize, String> {
        let mut conn = pool.get().map_err(|e| format!("DB Connection Error: {}", e))?;

        diesel::update(billing_subscriptions::table)
            .filter(billing_subscriptions::tenant_id.eq(tenant_id))
            .filter(billing_subscriptions::status.eq("active"))
            .set((
                billing_subscriptions::status.eq("canceled"),
                billing_subscriptions::canceled_at.eq(Some(chrono::Local::now().naive_local())),
            ))
            .execute(&mut conn)
            .map_err(|e| format!("Error canceling subscription: {}", e))
    }

    pub fn process_recurring_billing(conn: &mut PgConnection) -> Result<Vec<crate::services::billing_events::BillingEvent>, String> {
        let now = chrono::Utc::now().naive_utc();
        let mut events = Vec::new();

        // Find active subs where current_period_end < now
        let expired_subscriptions = billing_subscriptions::table
            .filter(billing_subscriptions::status.eq("active"))
            .filter(billing_subscriptions::current_period_end.lt(now))
            .load::<Subscription>(conn)
            .map_err(|e| format!("Error loading expired subs: {}", e))?;

        for sub in expired_subscriptions {
            // 1. Fetch Plan
            let plan = match billing_plans::table.find(sub.plan_id).first::<BillingPlan>(conn) {
                Ok(p) => p,
                Err(_) => {
                    log::error!("Plan {} not found for sub {}", sub.plan_id, sub.id);
                    continue; // Skip malformed
                }
            };

            // 2. Mock Charge / Invoice Generation
            // In real system: Call Stripe here.
            
            // 3. Extend Subscription
             let period_end = if plan.billing_interval == "year" {
                now + chrono::Duration::days(365)
            } else {
                now + chrono::Duration::days(30)
            };

            let update_res = diesel::update(billing_subscriptions::table.find(sub.id))
                .set((
                    billing_subscriptions::current_period_start.eq(now),
                    billing_subscriptions::current_period_end.eq(period_end),
                ))
                .execute(conn);

            if let Ok(_) = update_res {
                // 4. Create Invoice Record
                  let inv_num = format!("INV-{}-{}", sub.tenant_id, now.and_utc().timestamp());
                  let new_inv = NewInvoice {
                    subscription_id: sub.id,
                    amount_cents: plan.price_cents,
                    status: "paid".to_string(), // Auto-paid in this mock
                    due_date: now,
                    line_items: Some(serde_json::json!([{
                        "description": format!("Renewal: {} Plan", plan.name),
                        "amount": plan.price_cents
                    }])),
                    invoice_number: Some(inv_num.clone()),
                };
                
                let _ = diesel::insert_into(billing_invoices::table)
                    .values(&new_inv)
                    .execute(conn);

                events.push(crate::services::billing_events::BillingEvent {
                    tenant_id: sub.tenant_id,
                    event_type: "invoice_paid".to_string(),
                    amount_cents: plan.price_cents,
                    plan_name: plan.name,
                    invoice_number: inv_num,
                });
            }
        }
        
        Ok(events)
    }
    pub fn get_subscription_details(
         pool: &web::Data<DatabasePool>,
         tenant_id: i32,
    ) -> Result<serde_json::Value, String> {
        let mut conn = pool.get().map_err(|e| format!("DB Connection Error: {}", e))?;
        
        // 1. Get Active Subscription
        let active_sub_opt = billing_subscriptions::table
            .filter(billing_subscriptions::tenant_id.eq(tenant_id))
            .filter(billing_subscriptions::status.eq("active"))
            .first::<Subscription>(&mut conn)
            .optional()
            .map_err(|e| format!("Error loading subscription: {}", e))?;

        let mut current_plan_data = serde_json::json!({
            "name": "Free Tier",
            "price": "0.00",
            "interval": "month",
            "status": "Inactive",
            "next_billing_date": "N/A",
             "payment_method": { "last4": "----", "exp_month": "--", "exp_year": "--" }
        });

        if let Some(sub) = active_sub_opt {
             let plan = billing_plans::table.find(sub.plan_id).first::<BillingPlan>(&mut conn)
                .map_err(|e| format!("Plan not found: {}", e))?;
             
             current_plan_data = serde_json::json!({
                "name": plan.name,
                "price": format!("{:.2}", plan.price_cents as f64 / 100.0),
                "interval": plan.billing_interval,
                "status": "Active",
                 "next_billing_date": sub.current_period_end.format("%Y-%m-%d").to_string(),
                 "payment_method": { "last4": "4242", "exp_month": "12", "exp_year": "25" } // Mock
             });
        }

        // 2. Get Recent Invoices
        let invoices = billing_invoices::table
            .inner_join(billing_subscriptions::table)
            .filter(billing_subscriptions::tenant_id.eq(tenant_id))
            .order(billing_invoices::created_at.desc())
            .limit(5)
            .select(billing_invoices::all_columns)
            .load::<Invoice>(&mut conn)
            .map_err(|e| format!("Error loading invoices: {}", e))?;

        let recent_invoices: Vec<serde_json::Value> = invoices.into_iter().map(|inv| {
            serde_json::json!({
                "date": inv.created_at.map(|dt| dt.format("%Y-%m-%d").to_string()).unwrap_or_else(|| "N/A".to_string()),
                "amount": format!("{:.2}", inv.amount_cents as f64 / 100.0),
                 "status": inv.status,
            })
        }).collect();

        Ok(serde_json::json!({
            "currentPlan": current_plan_data,
            "recentInvoices": recent_invoices
        }))
    }
}
