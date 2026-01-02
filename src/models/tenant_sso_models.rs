use super::Model;
use super::PooledDatabaseConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::tenant_sso_configs;

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = tenant_sso_configs)]
pub struct TenantSsoConfig {
    pub id: i32,
    pub tenant_id: i32,
    pub idp_entity_id: String,
    pub idp_sso_url: String,
    pub x509_certificate: String,
    pub is_enabled: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    // Note: provider_type, client_id, client_secret, discovery_url removed - not in schema
}

#[derive(Debug, Insertable, AsChangeset, Clone, Serialize, Deserialize)]
#[diesel(table_name = tenant_sso_configs)]
pub struct MutTenantSsoConfig {
    pub tenant_id: i32,
    pub idp_entity_id: Option<String>,
    pub idp_sso_url: Option<String>,
    pub x509_certificate: Option<String>,
    pub is_enabled: Option<bool>,
    // Note: provider_type, client_id, client_secret, discovery_url removed - not in schema
}

impl TenantSsoConfig {
    pub fn find_by_tenant(t_id: i32, db: &mut PooledDatabaseConnection) -> Result<Option<TenantSsoConfig>, diesel::result::Error> {
        tenant_sso_configs::table
            .filter(tenant_sso_configs::tenant_id.eq(t_id))
            .first(db)
            .optional()
    }

    pub fn upsert(new: &MutTenantSsoConfig, db: &mut PooledDatabaseConnection) -> Result<usize, diesel::result::Error> {
        diesel::insert_into(tenant_sso_configs::table)
            .values(new)
            .on_conflict(tenant_sso_configs::tenant_id)
            .do_update()
            .set(new)
            .execute(db)
    }
}
