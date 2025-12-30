use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::pending_verifications)]
pub struct PendingVerification {
    pub id: i32,
    pub uuid: String,
    pub verification_type: String,
    pub email: String,
    pub verification_token: String,
    pub payload: serde_json::Value,
    pub tenant_id: Option<i32>,
    pub verified: bool,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub verified_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::pending_verifications)]
pub struct NewPendingVerification {
    pub uuid: String,
    pub verification_type: String,
    pub email: String,
    pub verification_token: String,
    pub payload: serde_json::Value,
    pub tenant_id: Option<i32>,
    pub expires_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::verification_settings)]
pub struct VerificationSettings {
    pub id: i32,
    pub tenant_id: Option<i32>,
    pub verification_type: String,
    pub ttl_hours: i32,
    pub enabled: bool,
    pub email_template: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
