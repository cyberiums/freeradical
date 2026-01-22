use diesel::prelude::*;
use freeradical::models::user_models::User;
use freeradical::models::tenant_models::{Tenant, TenantMember};
use freeradical::schema::{users, tenants, tenant_members};
use freeradical::models::db_connection::establish_connection_pool;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let pool = establish_connection_pool();
    let mut conn = pool.get().expect("Failed to get DB connection");

    println!("--- USERS ---");
    let all_users = users::table.load::<User>(&mut conn).unwrap_or(vec![]);
    for u in &all_users {
        println!("User: {} (ID: {})", u.email, u.id);
    }

    println!("\n--- TENANTS ---");
    let all_tenants = tenants::table.load::<Tenant>(&mut conn).unwrap_or(vec![]);
    for t in &all_tenants {
        println!("Tenant: {} (ID: {})", t.name, t.id);
    }

    println!("\n--- MEMBERSHIPS (Links) ---");
    let all_members = tenant_members::table.load::<TenantMember>(&mut conn).unwrap_or(vec![]);
    for m in &all_members {
        println!("Link: User {} <-> Tenant {}", m.user_id, m.tenant_id);
    }
}
