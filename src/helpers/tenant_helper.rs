use actix_web::HttpRequest;
use crate::models::db_connection::DatabasePool;
use diesel::prelude::*;
use crate::schema::tenants;
use crate::models::tenant_models::Tenant;

pub fn resolve_tenant_id(req: &HttpRequest, pool: &DatabasePool) -> Result<i32, String> {
    // 1. Check for X-Tenant-ID header (Dev/API overrides)
    if let Some(header_val) = req.headers().get("X-Tenant-ID") {
        if let Ok(val_str) = header_val.to_str() {
            if let Ok(id) = val_str.parse::<i32>() {
                return Ok(id);
            }
        }
    }

    // 2. Resolve from Host header
    if let Some(host_val) = req.headers().get("Host") {
        if let Ok(host_str) = host_val.to_str() {
            // Remove port if present
            let host_only = host_str.split(':').next().unwrap_or(host_str);
            
            // Database lookup
            let mut conn = pool.get().map_err(|_| "Failed to get db connection".to_string())?;
            
            // Check custom domain OR subdomain
            // Logic: if host ends with .oxidly.com (or app domain), it's a subdomain. Else custom domain.
            // For simplicity in this iteration, we search both columns.
            
            use crate::schema::tenants::dsl::*;
            
            let tenant_opt = tenants
                .filter(custom_domain.eq(host_only).or(subdomain.eq(host_only)))
                .first::<Tenant>(&mut conn)
                .optional()
                .map_err(|e| format!("Database error: {}", e))?;
                
            if let Some(t) = tenant_opt {
                return Ok(t.id);
            }
        }
    }

    Err("Tenant not found".to_string())
}

pub fn get_tenant_role(
    tid: i32,
    uid: i32,
    conn: &mut crate::models::PooledDatabaseConnection,
) -> Result<String, String> {
    use crate::schema::tenant_members;
    
    tenant_members::table
        .filter(tenant_members::tenant_id.eq(tid))
        .filter(tenant_members::user_id.eq(uid))
        .select(tenant_members::role)
        .first::<String>(conn)
        .map_err(|_| "Not a member of this tenant".to_string())
}
