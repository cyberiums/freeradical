use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// API DTO for CRM Customer - uses String for BigDecimal fields
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CrmCustomerDTO {
    pub id: i32,
    pub user_id: i32,
    
    // Lifecycle
    pub lifecycle_stage: String,
    pub customer_since: Option<String>,
    pub last_purchase_date: Option<String>,
    
    // RFM Scoring
    pub rfm_recency_score: Option<i32>,
    pub rfm_frequency_score: Option<i32>,
    pub rfm_monetary_score: Option<i32>,
    pub rfm_total_score: Option<i32>,
    
    // Metrics (BigDecimal converted to String)
    pub total_orders: Option<i32>,
    pub total_revenue: Option<String>,
    pub average_order_value: Option<String>,
    pub customer_lifetime_value: Option<String>,
    
    // Engagement
    pub last_interaction_date: Option<String>,
    pub interaction_count: Option<i32>,
    pub email_open_rate: Option<String>,
    pub email_click_rate: Option<String>,
    
    // Health & Risk
    pub health_score: Option<i32>,
    pub churn_risk: Option<String>,
    
    // Segmentation
    pub primary_segment_id: Option<i32>,
    pub tags: Option<Vec<Option<String>>>,
    
    // Metadata
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub tenant_id: Option<i32>,
}

/// API DTO for CRM Campaign - uses String for BigDecimal fields
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CrmCampaignDTO {
    pub id: i32,
    pub name: String,
    pub campaign_type: String,
    pub status: Option<String>,
    
    pub segment_id: Option<i32>,
    pub target_customer_count: Option<i32>,
    
    pub subject: Option<String>,
    pub content: Option<String>,
    pub template_id: Option<i32>,
    
    pub scheduled_at: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    
    // Performance metrics (BigDecimal converted to String)
    pub sent_count: Option<i32>,
    pub delivered_count: Option<i32>,
    pub opened_count: Option<i32>,
    pub clicked_count: Option<i32>,
    pub converted_count: Option<i32>,
    pub revenue_generated: Option<String>,
    
    pub created_by: Option<i32>,
    pub created_at: String,
    pub updated_at: String,
    pub tenant_id: Option<i32>,
}

// Conversion implementations
impl From<super::crm_models::CrmCustomer> for CrmCustomerDTO {
    fn from(customer: super::crm_models::CrmCustomer) -> Self {
        CrmCustomerDTO {
            id: customer.id,
            user_id: customer.user_id,
            lifecycle_stage: customer.lifecycle_stage,
            customer_since: customer.customer_since.map(|dt| dt.to_string()),
            last_purchase_date: customer.last_purchase_date.map(|dt| dt.to_string()),
            rfm_recency_score: customer.rfm_recency_score,
            rfm_frequency_score: customer.rfm_frequency_score,
            rfm_monetary_score: customer.rfm_monetary_score,
            rfm_total_score: customer.rfm_total_score,
            total_orders: customer.total_orders,
            total_revenue: customer.total_revenue.map(|bd| bd.to_string()),
            average_order_value: customer.average_order_value.map(|bd| bd.to_string()),
            customer_lifetime_value: customer.customer_lifetime_value.map(|bd| bd.to_string()),
            last_interaction_date: customer.last_interaction_date.map(|dt| dt.to_string()),
            interaction_count: customer.interaction_count,
            email_open_rate: customer.email_open_rate.map(|bd| bd.to_string()),
            email_click_rate: customer.email_click_rate.map(|bd| bd.to_string()),
            health_score: customer.health_score,
            churn_risk: customer.churn_risk,
            primary_segment_id: customer.primary_segment_id,
            tags: customer.tags,
            notes: customer.notes,
            created_at: customer.created_at.to_string(),
            updated_at: customer.updated_at.to_string(),
            tenant_id: customer.tenant_id,
        }
    }
}

impl From<super::crm_models::CrmCampaign> for CrmCampaignDTO {
    fn from(campaign: super::crm_models::CrmCampaign) -> Self {
        CrmCampaignDTO {
            id: campaign.id,
            name: campaign.name,
            campaign_type: campaign.campaign_type,
            status: campaign.status,
            segment_id: campaign.segment_id,
            target_customer_count: campaign.target_customer_count,
            subject: campaign.subject,
            content: campaign.content,
            template_id: campaign.template_id,
            scheduled_at: campaign.scheduled_at.map(|dt| dt.to_string()),
            started_at: campaign.started_at.map(|dt| dt.to_string()),
            completed_at: campaign.completed_at.map(|dt| dt.to_string()),
            sent_count: campaign.sent_count,
            delivered_count: campaign.delivered_count,
            opened_count: campaign.opened_count,
            clicked_count: campaign.clicked_count,
            converted_count: campaign.converted_count,
            revenue_generated: campaign.revenue_generated.map(|bd| bd.to_string()),
            created_by: campaign.created_by,
            created_at: campaign.created_at.to_string(),
            updated_at: campaign.updated_at.to_string(),
            tenant_id: campaign.tenant_id,
        }
    }
}
