use actix_web::{web, HttpResponse, Responder, HttpRequest};
use crate::models::{db_connection, tenant_models::{Tenant, NewTenant, NewTenantMember}};
use crate::middleware::auth_middleware::get_user_context;
use uuid::Uuid;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewTenantRequest {
    pub name: String,
    pub subdomain: String,
}

#[derive(Deserialize)]
pub struct InviteMemberRequest {
    pub email: String,
    pub role: String,
}

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
        Ok(_) => HttpResponse::Ok().json("Member invited successfully"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error inviting member: {}", e)),
    }
}
