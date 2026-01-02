use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, ToSchema)]
#[diesel(table_name = crate::schema::pending_verifications)]
pub struct PendingVerification {
    pub id: i32,
    pub uuid: Uuid,
    pub verification_type: String,
    pub email: String,
    pub verification_token: String,
    pub payload: serde_json::Value,
    pub tenant_id: Option<i32>,
    pub verified: Option<bool>,
    pub expires_at: NaiveDateTime,
    pub created_at: Option<NaiveDateTime>,
    pub verified_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::pending_verifications)]
pub struct NewPendingVerification {
    pub uuid: Uuid,
    pub verification_type: String,
    pub email: String,
    pub verification_token: String,
    pub payload: serde_json::Value,
    pub tenant_id: Option<i32>,
    pub expires_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, ToSchema)]
#[diesel(table_name = crate::schema::verification_settings)]
pub struct VerificationSettings {
    pub id: i32,
    pub tenant_id: Option<i32>,
    pub verification_type: String,
    pub ttl_hours: Option<i32>,
    pub enabled: Option<bool>,
    pub email_template: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, AsChangeset, Debug)]
#[diesel(table_name = crate::schema::verification_settings)]
pub struct NewVerificationSettings {
    pub tenant_id: Option<i32>,
    pub verification_type: String,
    pub ttl_hours: i32,
    pub enabled: bool,
    pub email_template: Option<String>,
}
