use super::Model;
use super::PooledDatabaseConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::schema::{tenants, tenant_members};

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = tenants)]
pub struct Tenant {
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub subdomain: String,
    pub custom_domain: Option<String>,
    pub plan: Option<String>,
    pub is_active: Option<bool>,
    pub settings: Option<serde_json::Value>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Insertable, AsChangeset, Clone, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = tenants)]
pub struct NewTenant {
    pub uuid: String,
    pub name: String,
    pub subdomain: String,
    pub custom_domain: Option<String>,
    pub plan: Option<String>,
    pub is_active: Option<bool>,
    pub settings: Option<serde_json::Value>,
}

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, Serialize, Deserialize, Associations, ToSchema)]
#[diesel(belongs_to(Tenant))]
#[diesel(table_name = tenant_members)]
pub struct TenantMember {
    pub id: i32,
    pub tenant_id: i32,
    pub user_id: i32,
    pub role: String,
    pub status: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Insertable, Clone, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = tenant_members)]
pub struct NewTenantMember {
    pub tenant_id: i32,
    pub user_id: i32,
    pub role: String,
    pub status: String,
}

impl Model<Tenant, NewTenant, i32> for Tenant {
    fn create(new: &NewTenant, db: &mut PooledDatabaseConnection) -> Result<usize, diesel::result::Error> {
        diesel::insert_into(tenants::table).values(new).execute(db)
    }

    fn read_one(id: i32, db: &mut PooledDatabaseConnection) -> Result<Tenant, diesel::result::Error> {
        tenants::table.find(id).first(db)
    }

    fn read_all(db: &mut PooledDatabaseConnection) -> Result<Vec<Tenant>, diesel::result::Error> {
        tenants::table.load(db)
    }

    fn update(
        id: i32,
        new: &NewTenant,
        db: &mut PooledDatabaseConnection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(tenants::table.find(id))
            .set((
                tenants::name.eq(&new.name),
                tenants::subdomain.eq(&new.subdomain),
                tenants::custom_domain.eq(&new.custom_domain),
                tenants::plan.eq(&new.plan),
                tenants::is_active.eq(&new.is_active),
                tenants::settings.eq(&new.settings),
                tenants::updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(db)
    }

    fn delete(id: i32, db: &mut PooledDatabaseConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(tenants::table.find(id)).execute(db)
    }
}

impl Tenant {
    pub fn find_by_subdomain(subdomain: &str, db: &mut PooledDatabaseConnection) -> Result<Tenant, diesel::result::Error> {
        tenants::table
            .filter(tenants::subdomain.eq(subdomain))
            .first(db)
    }

    pub fn add_member(
        tenant_id: i32,
        user_id: i32,
        role: &str,
        db: &mut PooledDatabaseConnection
    ) -> Result<usize, diesel::result::Error> {
        let new_member = NewTenantMember {
            tenant_id,
            user_id,
            role: role.to_string(),
            status: "active".to_string(),
        };
        diesel::insert_into(tenant_members::table)
            .values(&new_member)
            .execute(db)
    }

    pub fn get_members(tenant_id: i32, db: &mut PooledDatabaseConnection) -> Result<Vec<TenantMember>, diesel::result::Error> {
        tenant_members::table
            .filter(tenant_members::tenant_id.eq(tenant_id))
            .load(db)
    }

    pub fn get_tenant_id_for_user(user_id: i32, db: &mut PooledDatabaseConnection) -> Result<Option<i32>, diesel::result::Error> {
        tenant_members::table
            .filter(tenant_members::user_id.eq(user_id))
            .select(tenant_members::tenant_id)
            .first::<i32>(db)
            .optional()
    }
}
