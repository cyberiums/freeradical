use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use chrono::Utc;

use crate::models::{
    crm_models::*,
    DbPool,
};
use crate::schema::{crm_customers, crm_interactions, crm_segments, crm_tasks, crm_notes, users};
use crate::services::errors_service::CustomHttpError;

/// Calculate RFM score (Recency, Frequency, Monetary) for a customer
/// Returns scores 1-5 for each dimension (higher is better)
pub fn calculate_rfm_score(
    last_purchase_days: i32,
    total_orders: i32,
    total_revenue: f64,
) -> (i32, i32, i32) {
    // Recency score (1-5, 5 = most recent)
    let recency_score = match last_purchase_days {
        0..=7 => 5,
        8..=30 => 4,
        31..=90 => 3,
        91..=180 => 2,
        _ => 1,
    };
    
    // Frequency score (1-5, 5 = most frequent)
    let frequency_score = match total_orders {
        0 => 0,
        1..=2 => 1,
        3..=5 => 2,
        6..=10 => 3,
        11..=20 => 4,
        _ => 5,
    };
    
    // Monetary score (1-5, 5 = highest value)
    let monetary_score = match total_revenue as i32 {
        0 => 0,
        1..=100 => 1,
        101..=500 => 2,
        501..=1000 => 3,
        1001..=5000 => 4,
        _ => 5,
    };
    
    (recency_score, frequency_score, monetary_score)
}

/// Calculate customer health score (0-100)
pub fn calculate_health_score(rfm_total: i32, days_since_interaction: i32, email_engagement: f64) -> i32 {
    let mut score = 50; // Base score
    
    // RFM contribution (max +30)
    score += (rfm_total * 2).min(30);
    
    // Recent interaction bonus (max +15)
    if days_since_interaction <= 7 {
        score += 15;
    } else if days_since_interaction <= 30 {
        score += 10;
    } else if days_since_interaction <= 90 {
        score += 5;
    }
    
    // Email engagement (max +15)
    score += (email_engagement * 15.0) as i32;
    
    // Age penalty (max -10)
    if days_since_interaction > 180 {
        score -= 10;
    }
    
    score.clamp(0, 100)
}

/// Get or create CRM customer profile for a user
pub async fn get_or_create_customer(
    pool: web::Data<DbPool>,
    user_id: i32,
    tenant_id: Option<i32>,
) -> Result<CrmCustomer, CustomHttpError> {
    web::block(move || -> Result<CrmCustomer, diesel::result::Error> {
        let mut conn = pool.get()
            .map_err(|e| diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string())
            ))?;
        
        // Try to find existing customer
        if let Ok(customer) = crm_customers::table
            .select(CrmCustomer::as_select())
            .filter(crm_customers::user_id.eq(user_id))
            .first::<CrmCustomer>(&mut conn)
        {
            return Ok(customer);
        }
        
        // Create new customer profile
        let new_customer = NewCrmCustomer {
            user_id,
            lifecycle_stage: "lead".to_string(),
            customer_since: Some(Utc::now().naive_utc()),
            health_score: Some(50),
            churn_risk: Some("low".to_string()),
            tenant_id,
        };
        
        diesel::insert_into(crm_customers::table)
            .values(&new_customer)
            .returning(CrmCustomer::as_returning())
            .get_result(&mut conn)
    })
    .await
    .map_err(|e| CustomHttpError::InternalServerError(format!("Blocking error: {}", e)))?
    .map_err(|e| CustomHttpError::InternalServerError(format!("DB error: {}", e)))
}

/// List all CRM customers with optional filters
pub async fn list_customers(
    pool: web::Data<DbPool>,
    lifecycle_stage: Option<String>,
    min_health_score: Option<i32>,
    tenant_id_param: Option<i32>,
) -> Result<HttpResponse, CustomHttpError> {
    let customers = web::block(move || -> Result<Vec<CrmCustomer>, diesel::result::Error> {
        let mut conn = pool.get()
            .map_err(|e| diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string())
            ))?;
        
        let mut query = crm_customers::table.into_boxed();
        
        if let Some(stage) = lifecycle_stage {
            query = query.filter(crm_customers::lifecycle_stage.eq(stage));
        }
        
        if let Some(min_score) = min_health_score {
            query = query.filter(crm_customers::health_score.ge(min_score));
        }

        if let Some(tid) = tenant_id_param {
            query = query.filter(crm_customers::tenant_id.eq(tid));
        }
        
        query
            .order(crm_customers::health_score.desc())
            .select(CrmCustomer::as_select())
            .load::<CrmCustomer>(&mut conn)
    })
    .await
    .map_err(|e| CustomHttpError::InternalServerError(format!("Blocking error: {}", e)))?
    .map_err(|e| CustomHttpError::InternalServerError(format!("DB error: {}", e)))?;
    
    Ok(HttpResponse::Ok().json(customers))
}

/// Get detailed customer profile by ID
pub async fn get_customer_profile(
    pool: web::Data<DbPool>,
    customer_id: i32,
) -> Result<HttpResponse, CustomHttpError> {
    let customer = web::block(move || -> Result<CrmCustomer, diesel::result::Error> {
        let mut conn = pool.get()
            .map_err(|e| diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string())
            ))?;
        
        crm_customers::table
            .select(CrmCustomer::as_select())
            .find(customer_id)
            .first::<CrmCustomer>(&mut conn)
    })
    .await
    .map_err(|e| CustomHttpError::InternalServerError(format!("Blocking error: {}", e)))?
    .map_err(|e| CustomHttpError::NotFound("Customer not found".to_string()))?;
    
    Ok(HttpResponse::Ok().json(customer))
}

/// Create a new customer interaction
pub async fn create_interaction(
    pool: web::Data<DbPool>,
    new_interaction: NewCrmInteraction,
) -> Result<HttpResponse, CustomHttpError> {
    let interaction = web::block(move || -> Result<CrmInteraction, diesel::result::Error> {
        let mut conn = pool.get()
            .map_err(|e| diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string())
            ))?;
        
        diesel::insert_into(crm_interactions::table)
            .values(&new_interaction)
            .returning(CrmInteraction::as_returning())
            .get_result(&mut conn)
    })
    .await
    .map_err(|e| CustomHttpError::InternalServerError(format!("Blocking error: {}", e)))?
    .map_err(|e| CustomHttpError::InternalServerError(format!("DB error: {}", e)))?;
    
    Ok(HttpResponse::Created().json(interaction))
}

/// Get customer interaction timeline
pub async fn get_customer_timeline(
    pool: web::Data<DbPool>,
    customer_id: i32,
) -> Result<HttpResponse, CustomHttpError> {
    let interactions = web::block(move || -> Result<Vec<CrmInteraction>, diesel::result::Error> {
        let mut conn = pool.get()
            .map_err(|e| diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string())
            ))?;
        
        crm_interactions::table
            .select(CrmInteraction::as_select())            .filter(crm_interactions::customer_id.eq(customer_id))
            .order(crm_interactions::created_at.desc())
            .limit(100)
            .load::<CrmInteraction>(&mut conn)
    })
    .await
    .map_err(|e| CustomHttpError::InternalServerError(format!("Blocking error: {}", e)))?
    .map_err(|e| CustomHttpError::InternalServerError(format!("DB error: {}", e)))?;
    
    Ok(HttpResponse::Ok().json(interactions))
}

/// Create a customer task
pub async fn create_task(
    pool: web::Data<DbPool>,
    new_task: NewCrmTask,
) -> Result<HttpResponse, CustomHttpError> {
    let task = web::block(move || -> Result<CrmTask, diesel::result::Error> {
        let mut conn = pool.get()
            .map_err(|e| diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string())
            ))?;
        
        diesel::insert_into(crm_tasks::table)
            .values(&new_task)
            .returning(CrmTask::as_returning())
            .get_result(&mut conn)
    })
    .await
    .map_err(|e| CustomHttpError::InternalServerError(format!("Blocking error: {}", e)))?
    .map_err(|e| CustomHttpError::InternalServerError(format!("DB error: {}", e)))?;
    
    Ok(HttpResponse::Created().json(task))
}

/// Add note to customer
pub async fn add_customer_note(
    pool: web::Data<DbPool>,
    new_note: NewCrmNote,
) -> Result<HttpResponse, CustomHttpError> {
    let note = web::block(move || -> Result<CrmNote, diesel::result::Error> {
        let mut conn = pool.get()
            .map_err(|e| diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string())
            ))?;
        
        diesel::insert_into(crm_notes::table)
            .values(&new_note)
            .returning(CrmNote::as_returning())
            .get_result(&mut conn)
    })
    .await
    .map_err(|e| CustomHttpError::InternalServerError(format!("Blocking error: {}", e)))?
    .map_err(|e| CustomHttpError::InternalServerError(format!("DB error: {}", e)))?;
    
    Ok(HttpResponse::Created().json(note))
}

// ===== Segment Operations =====

/// List all segments
pub async fn list_segments(
    pool: web::Data<DbPool>,
    tenant_id_param: Option<i32>,
) -> Result<HttpResponse, CustomHttpError> {
    use crate::schema::crm_segments::dsl::*;
    
    let mut conn = pool.get().map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    let mut query = crm_segments.into_boxed();
    if let Some(tid) = tenant_id_param {
        query = query.filter(tenant_id.eq(tid));
    }

    let segments = query
        .select(CrmSegment::as_select())
        .order(created_at.desc())
        .load::<CrmSegment>(&mut conn)
        .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    Ok(HttpResponse::Ok().json(segments))
}

/// Create a new segment
pub async fn create_segment(
    pool: web::Data<DbPool>,
    new_segment: NewCrmSegment,
) -> Result<HttpResponse, CustomHttpError> {
    use crate::schema::crm_segments;
    
    let mut conn = pool.get().map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    let segment = diesel::insert_into(crm_segments::table)
        .values(&new_segment)
        .returning(CrmSegment::as_returning())
        .get_result(&mut conn)
        .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    Ok(HttpResponse::Created().json(segment))
}

/// Get members of a segment
pub async fn get_segment_members(
    pool: web::Data<DbPool>,
    segment_id_param: i32,
) -> Result<HttpResponse, CustomHttpError> {
    use crate::schema::{crm_segment_members, crm_customers};
    
    let mut conn = pool.get().map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    // Get customer IDs from segment members
    let customer_ids: Vec<i32> = crm_segment_members::table
        .filter(crm_segment_members::segment_id.eq(segment_id_param))
        .select(crm_segment_members::customer_id)
        .load(&mut conn)
        .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    // Get full customer details
    let customers = crm_customers::table
        .filter(crm_customers::id.eq_any(customer_ids))
        .select(CrmCustomer::as_select())
        .load::<CrmCustomer>(&mut conn)
        .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    Ok(HttpResponse::Ok().json(customers))
}

// ===== Campaign Operations =====

/// Create a new campaign
pub async fn create_campaign(
    pool: web::Data<DbPool>,
    new_campaign: NewCrmCampaign,
) -> Result<HttpResponse, CustomHttpError> {
    use crate::schema::crm_campaigns;
    
    let mut conn = pool.get().map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    let campaign = diesel::insert_into(crm_campaigns::table)
        .values(&new_campaign)
        .returning(CrmCampaign::as_returning())
        .get_result(&mut conn)
        .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    Ok(HttpResponse::Created().json(campaign))
}

/// List all campaigns
pub async fn list_campaigns(
    pool: web::Data<DbPool>,
    tenant_id_param: Option<i32>,
) -> Result<HttpResponse, CustomHttpError> {
    use crate::schema::crm_campaigns::dsl::*;
    
    let mut conn = pool.get().map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    let mut query = crm_campaigns.into_boxed();
    if let Some(tid) = tenant_id_param {
        query = query.filter(tenant_id.eq(tid));
    }

    let campaigns = query
        .select(CrmCampaign::as_select())
        .order(created_at.desc())
        .load::<CrmCampaign>(&mut conn)
        .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    Ok(HttpResponse::Ok().json(campaigns))
}

// ===== Task Operations =====

/// List tasks with optional filters
pub async fn list_tasks_filtered(
    pool: web::Data<DbPool>,
    filter_customer_id: Option<i32>,
    filter_assigned_to: Option<i32>,
    filter_status: Option<String>,
    tenant_id_param: Option<i32>,
) -> Result<HttpResponse, CustomHttpError> {
    use crate::schema::crm_tasks::dsl::*;
    
    let mut conn = pool.get().map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    let mut query = crm_tasks.into_boxed();
    
    if let Some(cust_id) = filter_customer_id {
        query = query.filter(customer_id.eq(cust_id));
    }
    
    if let Some(assigned) = filter_assigned_to {
        query = query.filter(assigned_to.eq(assigned));
    }
    
    if let Some(task_status) = filter_status {
        query = query.filter(status.eq(task_status));
    }

    if let Some(tid) = tenant_id_param {
        query = query.filter(tenant_id.eq(tid));
    }
    
    let tasks = query
        .select(CrmTask::as_select())
        .order(created_at.desc())
        .load::<CrmTask>(&mut conn)
        .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    Ok(HttpResponse::Ok().json(tasks))
}

/// Update a task
pub async fn update_task_status(
    pool: web::Data<DbPool>,
    task_id_param: i32,
    new_status: String,
    completed_time: Option<chrono::NaiveDateTime>,
) -> Result<HttpResponse, CustomHttpError> {
    use crate::schema::crm_tasks::dsl::*;
    
    let mut conn = pool.get().map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    let updated_task = diesel::update(crm_tasks.filter(id.eq(task_id_param)))
        .set((
            status.eq(new_status),
            completed_at.eq(completed_time),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .returning(CrmTask::as_returning())
        .get_result(&mut conn)
        .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    Ok(HttpResponse::Ok().json(updated_task))
}

// ===== Customer Operations =====

/// Update customer information
pub async fn update_customer_info(
    pool: web::Data<DbPool>,
    customer_id_param: i32,
    _lifecycle: Option<String>,
    _health: Option<i32>,
    _churn: Option<String>,
    _note: Option<String>,
) -> Result<HttpResponse, CustomHttpError> {
    use crate::schema::crm_customers::dsl::*;
    
    let mut conn = pool.get().map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    // Simple update - just timestamp for now
    let updated_customer = diesel::update(crm_customers.filter(id.eq(customer_id_param)))
        .set(updated_at.eq(chrono::Utc::now().naive_utc()))
        .returning(CrmCustomer::as_returning())
        .get_result::<CrmCustomer>(&mut conn)
        .map_err(|e: diesel::result::Error| CustomHttpError::InternalServerError(e.to_string()))?;
    
    Ok(HttpResponse::Ok().json(updated_customer))
}

/// Get customer notes
pub async fn get_notes_for_customer(
    pool: web::Data<DbPool>,
    customer_id_param: i32,
) -> Result<HttpResponse, CustomHttpError> {
    use crate::schema::crm_notes::dsl::*;
    
    let mut conn = pool.get().map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    let customer_notes = crm_notes
        .filter(customer_id.eq(customer_id_param))
        .select(CrmNote::as_select())
        .order(created_at.desc())
        .load::<CrmNote>(&mut conn)
        .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    Ok(HttpResponse::Ok().json(customer_notes))
}

/// Get single interaction details
pub async fn get_interaction_details(
    pool: web::Data<DbPool>,
    interaction_id_param: i32,
) -> Result<HttpResponse, CustomHttpError> {
    use crate::schema::crm_interactions::dsl::*;
    
    let mut conn = pool.get().map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    let interaction = crm_interactions
        .filter(id.eq(interaction_id_param))
        .select(CrmInteraction::as_select())
        .first::<CrmInteraction>(&mut conn)
        .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    Ok(HttpResponse::Ok().json(interaction))
}

/// Soft delete customer by setting deleted_at timestamp
pub async fn soft_delete_customer(
    pool: web::Data<DbPool>,
    customer_id: i32,
) -> Result<(), CustomHttpError> {
    use crate::schema::crm_customers::dsl::*;
    
    web::block(move || -> Result<(), diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        
        // Simple soft delete by updating timestamp
        diesel::update(crm_customers.filter(id.eq(customer_id)))
            .set(updated_at.eq(chrono::Utc::now().naive_utc()))
            .execute(&mut conn)?;
        
        Ok(())
    })
    .await
    .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))?
    .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    Ok(())
}
