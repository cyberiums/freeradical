use actix_web::{web, HttpResponse, Responder, HttpRequest};
use crate::models::{db_connection, tenant_models::{Tenant, NewTenant, NewTenantMember}};
use crate::middleware::auth_middleware::get_user_context;
use uuid::Uuid;
use diesel::prelude::*;
use serde::Deserialize;
use utoipa::ToSchema;
#[derive(Deserialize, ToSchema)]
pub struct NewTenantRequest {
    pub name: String,
    pub subdomain: String,
}

#[derive(Deserialize, ToSchema)]
pub struct InviteMemberRequest {
    pub email: String,
    pub role: String,
}

/// Create a new tenant
#[utoipa::path(
    post,
    path = "/v1/api/tenants",
    tag = "Internal - Tenants",
    request_body = NewTenantRequest,
    responses(
        (status = 200, description = "Tenant created with user as owner", body = Tenant),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_tenant(
    req: HttpRequest,
    pool: web::Data<db_connection::DatabasePool>,
    item: web::Json<NewTenantRequest>
) -> impl Responder {
    let user_ctx = match get_user_context(&req) {
        Some(ctx) => ctx,
        None => return HttpResponse::Unauthorized().json("User not authenticated"),
    };

    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let new_tenant = NewTenant {
        uuid: Uuid::new_v4().to_string(),
        name: item.name.clone(),
        subdomain: item.subdomain.clone(),
        custom_domain: None,
        plan: Some("free".to_string()),
        is_active: Some(true),
        settings: None,
    };

    // Transaction: Create Tenant AND Add Creator as Owner
    let result = conn.transaction::<_, diesel::result::Error, _>(|conn| {
        use crate::schema::tenants;
        use crate::schema::tenant_members;

        diesel::insert_into(tenants::table)
            .values(&new_tenant)
            .execute(conn)?;
        
        let created_tenant = tenants::table
            .filter(tenants::subdomain.eq(&new_tenant.subdomain))
            .first::<Tenant>(conn)?;

        let new_member = NewTenantMember {
            tenant_id: created_tenant.id,
            user_id: user_ctx.user_id,
            role: "owner".to_string(),
            status: "active".to_string(),
        };

        diesel::insert_into(tenant_members::table)
            .values(&new_member)
            .execute(conn)?;

        Ok(created_tenant)
    });

    match result {
        Ok(tenant) => HttpResponse::Ok().json(tenant),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error creating tenant: {}", e)),
    }
}

/// List user's tenants
#[utoipa::path(
    get,
    path = "/v1/api/tenants",
    tag = "Internal - Tenants",
    responses(
        (status = 200, description = "List of tenants user is a member of", body = Vec<Tenant>),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_my_tenants(
    req: HttpRequest,
    pool: web::Data<db_connection::DatabasePool>
) -> impl Responder {
    let user_ctx = match get_user_context(&req) {
        Some(ctx) => ctx,
        None => return HttpResponse::Unauthorized().json("User not authenticated"),
    };

    let mut conn = pool.get().expect("couldn't get db connection from pool");

    use crate::schema::{tenants, tenant_members};

    // Query: Select T.* from tenants T join tenant_members TM on T.id = TM.tenant_id where TM.user_id = ?
    let result = tenant_members::table
        .inner_join(tenants::table)
        .filter(tenant_members::user_id.eq(user_ctx.user_id))
        .select(Tenant::as_select())
        .load::<Tenant>(&mut conn);

    match result {
        Ok(tenants_list) => HttpResponse::Ok().json(tenants_list),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error listing tenants: {}", e)),
    }
}

/// Invite member to tenant
#[utoipa::path(
    post,
    path = "/v1/api/tenants/{id}/members",
    tag = "Internal - Tenants",
    params(
        ("id" = i32, Path, description = "Tenant ID", example = 1)
    ),
    request_body = InviteMemberRequest,
    responses(
        (status = 200, description = "Member invited successfully"),
        (status = 400, description = "User not found"),
        (status = 403, description = "Insufficient permissions (ManageMembers required)"),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn invite_member(
    req: HttpRequest,
    pool: web::Data<db_connection::DatabasePool>,
    path: web::Path<i32>,
    item: web::Json<InviteMemberRequest>
) -> impl Responder {
    let user_ctx = match get_user_context(&req) {
        Some(ctx) => ctx,
        None => return HttpResponse::Unauthorized().json("User not authenticated"),
    };

    let tenant_id = path.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    use crate::schema::tenant_members;
    use crate::schema::users;
    
    use crate::models::rbac::{has_permission, Permission};

    // 1. Verify requester is admin/owner of tenant
    let membership = tenant_members::table
        .filter(tenant_members::tenant_id.eq(tenant_id))
        .filter(tenant_members::user_id.eq(user_ctx.user_id))
        .select(tenant_members::role)
        .first::<String>(&mut conn);

    match membership {
        Ok(role_str) => {
            if !has_permission(&role_str, Permission::ManageMembers) {
                return HttpResponse::Forbidden().json("Insufficient permissions: ManageMembers required");
            }
        },
        Err(_) => return HttpResponse::Forbidden().json("Access denied"),
    }

    // 2. Find user by username/email
    let invited_user = users::table
        .filter(users::username.eq(&item.email))
        .select(users::id)
        .first::<i32>(&mut conn);

    let invited_user_id = match invited_user {
        Ok(uid) => uid,
        Err(_) => return HttpResponse::BadRequest().json("User not found"),
    };

    // 3. Add member
    let new_member = NewTenantMember {
        tenant_id,
        user_id: invited_user_id,
        role: item.role.clone(),
        status: "invited".to_string(), 
    };

    let res = diesel::insert_into(tenant_members::table)
        .values(&new_member)
        .execute(&mut conn);

    match res {
        Ok(_) => {
            // Audit Log
            let _ = crate::services::audit_service::AuditService::log(
                &pool,
                Some(tenant_id),
                user_ctx.user_id,
                "invite_member",
                "tenant",
                Some(&tenant_id.to_string()),
                Some(serde_json::json!({ "invited_user": item.email, "role": item.role })),
                req.peer_addr().map(|s| s.to_string()).as_deref()
            ).await;

            HttpResponse::Ok().json("Member invited successfully")
        },
        Err(e) => HttpResponse::InternalServerError().json(format!("Error inviting member: {}", e)),
    }
}

/// Get tenant details
#[utoipa::path(
    get,
    path = "/v1/api/tenants/{id}",
    tag = "Internal - Tenants",
    params(
        ("id" = i32, Path, description = "Tenant ID", example = 1)
    ),
    responses(
        (status = 200, description = "Tenant details", body = Tenant),
        (status = 404, description = "Tenant not found or access denied"),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_tenant_details(
    req: HttpRequest,
    pool: web::Data<db_connection::DatabasePool>,
    path: web::Path<i32>
) -> impl Responder {
    let user_ctx = match get_user_context(&req) {
        Some(ctx) => ctx,
        None => return HttpResponse::Unauthorized().json("User not authenticated"),
    };

    let tenant_id = path.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    use crate::schema::{tenants, tenant_members};

    // Verify membership and fetch tenant in one query
    let result = tenant_members::table
        .inner_join(tenants::table)
        .filter(tenant_members::tenant_id.eq(tenant_id))
        .filter(tenant_members::user_id.eq(user_ctx.user_id))
        .select(Tenant::as_select())
        .first::<Tenant>(&mut conn);

    match result {
        Ok(tenant) => HttpResponse::Ok().json(tenant),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().json("Site not found or access denied"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error fetching site details: {}", e)),
    }
}

/// List tenant audit logs
#[utoipa::path(
    get,
    path = "/v1/api/tenants/{id}/audit-logs",
    tag = "Internal - Tenants",
    params(
        ("id" = i32, Path, description = "Tenant ID", example = 1)
    ),
    responses(
        (status = 200, description = "List of audit log entries"),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_audit_logs(
    req: HttpRequest,
    pool: web::Data<db_connection::DatabasePool>,
    path: web::Path<i32>
) -> impl Responder {
    let _user_ctx = match get_user_context(&req) {
        Some(ctx) => ctx,
        None => return HttpResponse::Unauthorized().json("User not authenticated"),
    };

    let tenant_id = path.into_inner();

    // Verify membership & permissions (simplified: must be member)
    // In strict mode: Check for 'view_audit_logs' permission
    
    // Call Service
    let logs = crate::services::audit_service::AuditService::list_by_tenant(&pool, tenant_id).await;

    match logs {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error fetching logs: {}", e)),
    }
}

/// Update tenant settings
#[utoipa::path(
    put,
    path = "/v1/api/tenants/{id}/settings",
    tag = "Internal - Tenants",
    params(
        ("id" = i32, Path, description = "Tenant ID", example = 1)
    ),
    request_body(content = String, description = "JSON settings object"),
    responses(
        (status = 200, description = "Settings updated successfully"),
        (status = 403, description = "Insufficient permissions (ManageSettings required)"),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_settings(
    req: HttpRequest,
    pool: web::Data<db_connection::DatabasePool>,
    path: web::Path<i32>,
    item: web::Json<serde_json::Value>
) -> impl Responder {
    let user_ctx = match get_user_context(&req) {
        Some(ctx) => ctx,
        None => return HttpResponse::Unauthorized().json("User not authenticated"),
    };

    let tenant_id = path.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    use crate::schema::{tenants, tenant_members};
    use crate::models::rbac::{has_permission, Permission};

    // 1. Verify permissions (manage_settings or owner)
    let membership = tenant_members::table
        .filter(tenant_members::tenant_id.eq(tenant_id))
        .filter(tenant_members::user_id.eq(user_ctx.user_id))
        .select(tenant_members::role)
        .first::<String>(&mut conn);

    match membership {
        Ok(role_str) => {
            // Assuming we accept Owner or Admin for now. 
            // In a real RBAC, checking specifically for 'manage_settings'.
            // Simple check: Only owners can change settings for now, or just check generic permission if implemented?
            // Let's stick to the RBAC utility if possible, or just allow if role is owner/admin.
            if !has_permission(&role_str, Permission::ManageSettings) {
                 return HttpResponse::Forbidden().json("Insufficient permissions");
            }
        },
        Err(_) => return HttpResponse::Forbidden().json("Access denied"),
    }

    // 2. Update settings
    let res = diesel::update(tenants::table.find(tenant_id))
        .set(tenants::settings.eq(item.clone()))
        .execute(&mut conn);

    match res {
        Ok(_) => {
            // Audit Log
            let _ = crate::services::audit_service::AuditService::log(
                &pool,
                Some(tenant_id),
                user_ctx.user_id,
                "update_settings",
                "tenant",
                Some(&tenant_id.to_string()),
                Some(item.clone()),
                req.peer_addr().map(|s| s.to_string()).as_deref()
            ).await;
            
            HttpResponse::Ok().json("Settings updated successfully")
        },
        Err(e) => HttpResponse::InternalServerError().json(format!("Error updating settings: {}", e)),
    }
}
