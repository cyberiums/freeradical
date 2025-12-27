use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::analytics_events;

/// Analytics event tracking
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = analytics_events)]
pub struct AnalyticsEvent {
    pub id: i64,
    pub event_type: String,
    pub page_uuid: Option<String>,
    pub user_id: Option<i32>,
    pub session_id: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub referer: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: Option<NaiveDateTime>,
}

/// New analytics event for insertion
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = analytics_events)]
pub struct NewAnalyticsEvent {
    pub event_type: String,
    pub page_uuid: Option<String>,
    pub user_id: Option<i32>,
    pub session_id: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub referer: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

impl NewAnalyticsEvent {
    pub fn page_view(page_uuid: String, session_id: String) -> Self {
        Self {
            event_type: "page_view".to_string(),
            page_uuid: Some(page_uuid),
            user_id: None,
            session_id: Some(session_id),
            ip_address: None,
            user_agent: None,
            referer: None,
            metadata: None,
        }
    }
}
