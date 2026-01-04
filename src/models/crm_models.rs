use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use utoipa::ToSchema;

use crate::schema::{
    crm_customers, crm_interactions, crm_segments, crm_campaigns,
    crm_tasks, crm_notes, crm_segment_members
};

// ===== CRM Customer =====
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crm_customers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CrmCustomer {
    pub id: i32,
    pub user_id: i32,
    
    // Lifecycle
    pub lifecycle_stage: String, // lead, prospect, customer, champion, churned
    pub customer_since: Option<NaiveDateTime>,
    pub last_purchase_date: Option<NaiveDateTime>,
    
    // RFM Scoring
    pub rfm_recency_score: Option<i32>,
    pub rfm_frequency_score: Option<i32>,
    pub rfm_monetary_score: Option<i32>,
    pub rfm_total_score: Option<i32>,
    
    // Metrics
    pub total_orders: Option<i32>,
    pub total_revenue: Option<bigdecimal::BigDecimal>,
    pub average_order_value: Option<bigdecimal::BigDecimal>,
    pub customer_lifetime_value: Option<bigdecimal::BigDecimal>,
    
    // Engagement
    pub last_interaction_date: Option<NaiveDateTime>,
    pub interaction_count: Option<i32>,
    pub email_open_rate: Option<bigdecimal::BigDecimal>,
    pub email_click_rate: Option<bigdecimal::BigDecimal>,
    
    // Health & Risk
    pub health_score: Option<i32>,
    pub churn_risk: Option<String>, // low, medium, high
    
    // Segmentation
    pub primary_segment_id: Option<i32>,
    pub tags: Option<Vec<Option<String>>>,
    
    // Metadata
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub tenant_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, ToSchema)]
#[diesel(table_name = crm_customers)]
pub struct NewCrmCustomer {
    pub user_id: i32,
    pub lifecycle_stage: String,
    pub customer_since: Option<NaiveDateTime>,
    pub health_score: Option<i32>,
    pub churn_risk: Option<String>,
    pub tenant_id: Option<i32>,
    // Note: email, first_name, last_name, tags, source, metadata removed - not in database schema
}

// ===== CRM Interaction =====
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable, ToSchema)]
#[diesel(table_name = crm_interactions)]
pub struct CrmInteraction {
    pub id: i32,
    pub customer_id: i32,
    
    pub interaction_type: String, // email, call, meeting, order, support_ticket, page_view
    pub interaction_channel: Option<String>, // web, email, phone, in_person, social
    
    pub subject: Option<String>,
    pub description: Option<String>,
    pub outcome: Option<String>, // success, failed, pending, no_response
    
    pub order_id: Option<i64>,
    pub related_entity_type: Option<String>,
    pub related_entity_id: Option<i32>,
    
    pub created_by: Option<i32>,
    pub created_at: NaiveDateTime,
    pub tenant_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crm_interactions)]
pub struct NewCrmInteraction {
    pub customer_id: i32,
    pub interaction_type: String,
    pub interaction_channel: Option<String>,
    pub subject: Option<String>,
    pub description: Option<String>,
    pub outcome: Option<String>,
    pub created_by: Option<i32>,
    pub tenant_id: Option<i32>,
}

// ===== CRM Segment =====
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable, ToSchema)]
#[diesel(table_name = crm_segments)]
pub struct CrmSegment {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    
    pub criteria: serde_json::Value, // JSONB criteria
    pub is_dynamic: Option<bool>,
    
    pub customer_count: Option<i32>,
    pub last_calculated_at: Option<NaiveDateTime>,
    
    pub created_by: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub tenant_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crm_segments)]
pub struct NewCrmSegment {
    pub name: String,
    pub description: Option<String>,
    pub criteria: serde_json::Value,
    pub is_dynamic: Option<bool>,
    pub created_by: Option<i32>,
    pub tenant_id: Option<i32>,
}

// ===== CRM Campaign =====
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crm_campaigns)]
pub struct CrmCampaign {
    pub id: i32,
    pub name: String,
    pub campaign_type: String, // email, sms, push, social
    pub status: Option<String>, // draft, scheduled, active, paused, completed
    
    pub segment_id: Option<i32>,
    pub target_customer_count: Option<i32>,
    
    pub subject: Option<String>,
    pub content: Option<String>,
    pub template_id: Option<i32>,
    
    pub scheduled_at: Option<NaiveDateTime>,
    pub started_at: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>,
    
    // Performance metrics
    pub sent_count: Option<i32>,
    pub delivered_count: Option<i32>,
    pub opened_count: Option<i32>,
    pub clicked_count: Option<i32>,
    pub converted_count: Option<i32>,
    pub revenue_generated: Option<bigdecimal::BigDecimal>,
    
    pub created_by: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub tenant_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crm_campaigns)]
pub struct NewCrmCampaign {
    pub name: String,
    pub campaign_type: String,
    pub status: Option<String>,
    pub segment_id: Option<i32>,
    pub subject: Option<String>,
    pub content: Option<String>,
    pub scheduled_at: Option<NaiveDateTime>,
    pub created_by: Option<i32>,
    pub tenant_id: Option<i32>,
}

// ===== CRM Task =====
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable, ToSchema)]
#[diesel(table_name = crm_tasks)]
pub struct CrmTask {
    pub id: i32,
    pub customer_id: Option<i32>,
    
    pub title: String,
    pub description: Option<String>,
    pub task_type: Option<String>, // call, email, meeting, follow_up, demo
    pub priority: Option<String>, // low, medium, high, urgent
    pub status: Option<String>, // pending, in_progress, completed, cancelled
    
    pub due_date: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>,
    
    pub assigned_to: Option<i32>,
    pub created_by: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub tenant_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crm_tasks)]
pub struct NewCrmTask {
    pub customer_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub task_type: Option<String>,
    pub priority: Option<String>,
    pub due_date: Option<NaiveDateTime>,
    pub assigned_to: Option<i32>,
    pub created_by: Option<i32>,
    pub tenant_id: Option<i32>,
}

// ===== CRM Note =====
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable, ToSchema)]
#[diesel(table_name = crm_notes)]
pub struct CrmNote {
    pub id: i32,
    pub customer_id: i32,
    
    pub note_text: String,
    pub is_pinned: Option<bool>,
    
    pub created_by: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub tenant_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crm_notes)]
pub struct NewCrmNote {
    pub customer_id: i32,
    pub note_text: String,
    pub is_pinned: Option<bool>,
    pub created_by: Option<i32>,
    pub tenant_id: Option<i32>,
}

// ===== CRM Segment Member =====
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[diesel(table_name = crm_segment_members)]
#[diesel(belongs_to(CrmSegment, foreign_key = segment_id))]
#[diesel(belongs_to(CrmCustomer, foreign_key = customer_id))]
#[diesel(primary_key(segment_id, customer_id))]
pub struct CrmSegmentMember {
    pub segment_id: i32,
    pub customer_id: i32,
    pub added_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crm_segment_members)]
pub struct NewCrmSegmentMember {
    pub segment_id: i32,
    pub customer_id: i32,
}
