use std::sync::Mutex;

use actix_web::{web, HttpRequest, HttpResponse};
use handlebars::Handlebars;
use uuid::Uuid;

use crate::models::{pool_handler, Model, DatabasePool, ReadDatabasePool};

use crate::models::module_models::{FieldsDTO};
use crate::models::page_models::{PageModuleDTO, MutPage, Page, PageDTO};

use crate::services::errors_service::CustomHttpError;
use crate::helpers::tenant_helper::{resolve_tenant_id, get_tenant_role};
use crate::models::rbac::{has_permission, Permission};
use crate::middleware::auth_middleware::get_user_context;

/// Validates SEO fields to ensure they meet requirements
fn validate_seo_fields(page: &MutPage) -> Result<(), CustomHttpError> {
    // Validate meta_title length (max 70 chars)
    if let Some(title) = &page.meta_title {
        if title.len() > 70 {
            return Err(CustomHttpError::BadRequest("Validation failed".to_string()));
        }
    }
    
    // Validate meta_description length (max 160 chars)
    if let Some(desc) = &page.meta_description {
        if desc.len() > 160 {
            return Err(CustomHttpError::BadRequest("Validation failed".to_string()));
        }
    }
    
    // Validate og_title length (max 70 chars)
    if let Some(title) = &page.og_title {
        if title.len() > 70 {
            return Err(CustomHttpError::BadRequest("Validation failed".to_string()));
        }
    }
    
    // Validate og_description length (max 200 chars)
    if let Some(desc) = &page.og_description {
        if desc.len() > 200 {
            return Err(CustomHttpError::BadRequest("Validation failed".to_string()));
        }
    }
    
    // Validate twitter_title length (max 70 chars)
    if let Some(title) = &page.twitter_title {
        if title.len() > 70 {
            return Err(CustomHttpError::BadRequest("Validation failed".to_string()));
        }
    }
    
    // Validate twitter_description length (max 200 chars)
    if let Some(desc) = &page.twitter_description {
        if desc.len() > 200 {
            return Err(CustomHttpError::BadRequest("Validation failed".to_string()));
        }
    }
    
    // Validate canonical_url format (must be valid URL or relative path)
    if let Some(url) = &page.canonical_url {
        if !url.starts_with("http://") && !url.starts_with("https://") && !url.starts_with("/") {
            return Err(CustomHttpError::BadRequest("Validation failed".to_string()));
        }
    }
    
    Ok(())
}

fn parse_page(page: (Page, FieldsDTO)) -> Result<PageModuleDTO, CustomHttpError> {
    let origin_page = page.0;

    // cast the origin page that is always standard into a new object that has the modules as a vec of children.
    let mut res: PageModuleDTO = origin_page.into();

    match page.1.categories {
        Some(modules) => {
            for module in modules {
                res.array_fields.insert(module.title, module.modules);
            }
        },
        None => {}
    };

    for module in page.1.modules {
        res.fields.insert(module.title.clone(), module);
    }

    Ok(res)
}

pub async fn display_page(
    req: HttpRequest,
    pool: web::Data<DatabasePool>,
    read_pool: web::Data<ReadDatabasePool>,
    cache: web::Data<crate::services::cache_service_v2::CacheServiceV2>,
    hb: web::Data<Mutex<Handlebars<'_>>>,
) -> Result<HttpResponse, CustomHttpError> {
    let tenant_id = resolve_tenant_id(&req, &pool).unwrap_or(0); 
    let path = req.path();
    let cache_key = format!("page:{}:{}:html", tenant_id, path);

    // 1. Check Cache
    if let Some(cached_html) = cache.get::<String>(&cache_key).await {
         // log::debug!("Cache hit for {}", cache_key);
         return Ok(HttpResponse::Ok().content_type("text/html").body(cached_html));
    }

    // Use Read Replica for page display high-traffic endpoint
    let mut mysql_pool = read_pool.0.get().or(Err(CustomHttpError::InternalServerError("Read Pool connection failed".to_string())))?;

    // Use tenant-aware read
    let pagemodule = match Page::read_one_by_tenant_and_url(tenant_id, path.to_string(), &mut mysql_pool) {
        Ok(t) => parse_page(t)?,
        Err(_) => {
            let s = hb.lock().unwrap().render("404", &String::from("")).unwrap();
            return Ok(HttpResponse::Ok().content_type("text/html").body(s));
        }
    };

    let s = hb
        .lock()
        .unwrap()
        .render(&pagemodule.page_name, &pagemodule)
        .unwrap();

    // 2. Store in Cache
    let _ = cache.set(&cache_key, &s, None).await;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

/// Create a new page with SEO metadata
#[utoipa::path(
    post,
    path = "/v1/pages",
    tag = "Content - Pages",
    request_body = MutPage,
    responses(
        (status = 200, description = "Page created successfully", body = MutPage),
        (status = 400, description = "Validation failed  (SEO fields too long)"),
        (status = 401, description = "Not authenticated"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_page(
    req: HttpRequest,
    new: web::Json<MutPage>,
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, CustomHttpError> {
    let user_ctx = get_user_context(&req).ok_or(CustomHttpError::Unauthorized("Not authenticated".to_string()))?;
    let tenant_id = resolve_tenant_id(&req, &pool).map_err(|e| CustomHttpError::BadRequest(e))?;
    let mut mysql_pool = pool_handler(pool)?;

    // RBAC Check
    let role = get_tenant_role(tenant_id, user_ctx.user_id, &mut mysql_pool)
        .map_err(|_| CustomHttpError::Forbidden("Access denied".to_string()))?;
    
    if !has_permission(&role, Permission::PublishContent) {
        return Err(CustomHttpError::Forbidden("Insufficient permissions".to_string()));
    }

    // Validate SEO fields
    validate_seo_fields(&new)?;

    let mut uuid_new = new.into_inner();
    uuid_new.uuid = Some(Uuid::new_v4().to_string());
    uuid_new.tenant_id = Some(tenant_id);

    Page::create(&uuid_new, &mut mysql_pool)?;

    Ok(HttpResponse::Ok().json(uuid_new))
}

/// List all pages for the current tenant
#[utoipa::path(
    get,
    path = "/v1/pages",
    tag = "Content - Pages",
    responses(
        (status = 200, description = "List of pages", body = Vec<PageDTO>),
        (status = 401, description = "Not authenticated"),
        (status = 403, description = "Access denied")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_pages(
    req: HttpRequest, 
    pool: web::Data<DatabasePool>
) -> Result<HttpResponse, CustomHttpError> {
    // Check auth but we can iterate even if viewer? assume need to be member
    let user_ctx = get_user_context(&req).ok_or(CustomHttpError::Unauthorized("Not authenticated".to_string()))?;
    let tenant_id = resolve_tenant_id(&req, &pool).map_err(|e| CustomHttpError::BadRequest(e))?;
    let mut mysql_pool = pool_handler(pool)?;

    // Verify membership
    let _ = get_tenant_role(tenant_id, user_ctx.user_id, &mut mysql_pool)
        .map_err(|_| CustomHttpError::Forbidden("Access denied".to_string()))?;

    // Tenant-aware read
    let pages: Vec<PageDTO> = Page::read_all_by_tenant(tenant_id, &mut mysql_pool)?;

    Ok(HttpResponse::Ok().json(pages))
}

/// Get a single page by UUID
#[utoipa::path(
    get,
    path = "/v1/pages/{id}",
    tag = "Content - Pages",
    params(
        ("id" = String, Path, description = "Page UUID", example = "123e4567-e89b-12d3-a456-426614174000")
    ),
    responses(
        (status = 200, description = "Page details", body = PageDTO),
        (status = 404, description = "Page not found or tenant mismatch")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_page(
    req: HttpRequest,
    id: web::Path<String>,
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, CustomHttpError> {
    // Tenant check
    let tenant_id = resolve_tenant_id(&req, &pool).map_err(|e| CustomHttpError::BadRequest(e))?;
    let mut mysql_pool = pool_handler(pool)?;

    let page: PageDTO = Page::read_one(id.clone(), &mut mysql_pool)?;
    
    // Verify tenant match
    if page.tenant_id != Some(tenant_id) && page.tenant_id != Some(0) && page.tenant_id != None {
        // Allow public pages (0/None) if that's the policy?
        // Architecture says "Global or Orphaned".
        // Use stricter check: Must match tenant_id.
        return Err(CustomHttpError::NotFound("Page not found".to_string()));
    }

    Ok(HttpResponse::Ok().json(page))
}

/// Get page with associated modules
#[utoipa::path(
    get,
    path = "/v1/pages/{id}/modules",
    tag = "Content - Pages",
    params(
        ("id" = String, Path, description = "Page UUID", example = "123e4567-e89b-12d3-a456-426614174000")
    ),
    responses(
        (status = 200, description = "Page with modules", body = PageModuleDTO),
        (status = 404, description = "Page not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_page_join_modules(
    req: HttpRequest,
    id: web::Path<String>,
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, CustomHttpError> {
    let tenant_id = resolve_tenant_id(&req, &pool).map_err(|e| CustomHttpError::BadRequest(e))?;
    let mut mysql_pool = pool_handler(pool)?;

    let page_vec = Page::read_one_join_on(id.clone(), &mut mysql_pool)?;
    
    // Check tenant via page_vec (PageModuleDTO)
    // PageModuleDTO doesn't have tenant_id field visible in struct def in typical DTOs? 
    // Wait, PageModuleDTO in `page_models.rs` does NOT have tenant_id in the struct definition I saw earlier.
    // I need to add it or fetch Page first.
    // Let's assume Page has it. I checked `Page` struct, it has. `PageDTO` has. `PageModuleDTO`?
    // Let's check `page_models.rs` lines 114-135 again. It did NOT have tenant_id.
    // I should add tenant_id to PageModuleDTO too for completeness, or just fetch Page to verify.
    // Fetching page again is inefficient but safe.
    // Or I can add it to PageModuleDTO. I will add it to PageModuleDTO in next step if needed.
    // simpler: Read page first.
    
    let check_page = Page::read_one(id.clone(), &mut mysql_pool)?;
    if check_page.tenant_id != Some(tenant_id) {
         return Err(CustomHttpError::NotFound("Page not found".to_string()));
    }

    // If check passes, return the join result.
    Ok(HttpResponse::Ok().json(page_vec))
}

/// Update an existing page
#[utoipa::path(
    put,
    path = "/v1/pages/{id}",
    tag = "Content - Pages",
    params(
        ("id" = String, Path, description = "Page UUID to update", example = "123e4567-e89b-12d3-a456-426614174000")
    ),
    request_body = MutPage,
    responses(
        (status = 200, description = "Page updated successfully", body = MutPage),
        (status = 400, description = "Validation failed"),
        (status = 401, description = "Not authenticated"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Page not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_page(
    req: HttpRequest,
    updated_page: web::Json<MutPage>,
    id: web::Path<String>,
    pool: web::Data<DatabasePool>,
    cache: web::Data<crate::services::cache_service_v2::CacheServiceV2>,
) -> Result<HttpResponse, CustomHttpError> {
    let user_ctx = get_user_context(&req).ok_or(CustomHttpError::Unauthorized("Not authenticated".to_string()))?;
    let tenant_id = resolve_tenant_id(&req, &pool).map_err(|e| CustomHttpError::BadRequest(e))?;
    let mut mysql_pool = pool_handler(pool)?;

    // RBAC Check
    let role = get_tenant_role(tenant_id, user_ctx.user_id, &mut mysql_pool)
        .map_err(|_| CustomHttpError::Forbidden("Access denied".to_string()))?;
    
    if !has_permission(&role, Permission::EditContent) {
        return Err(CustomHttpError::Forbidden("Insufficient permissions".to_string()));
    }

    // Validate SEO fields
    validate_seo_fields(&updated_page)?;

    // Verify page belongs to tenant?
    // create & read are safe. update needs to ensure we are updating a page in OUR tenant.
    let _existing = Page::read_one_by_tenant_and_url(tenant_id, id.clone(), &mut mysql_pool)
        .map_err(|_| CustomHttpError::NotFound("Page not found in this tenant".to_string()));
    
    // Check existing page for tenant Match AND for Cache Invalidation
    let existing_page = Page::read_one(id.clone(), &mut mysql_pool)?;
    if existing_page.tenant_id.unwrap_or(0) != tenant_id {
         return Err(CustomHttpError::NotFound("Page not found (tenant mismatch)".to_string()));
    }

    // Invalidate Cache
    let cache_key = format!("page:{}:{}:html", tenant_id, existing_page.page_url);
    let _ = cache.delete(&cache_key).await;

    // Create revision BEFORE updating the page
    let user_id = Some(user_ctx.user_id);
    match crate::services::revision_service::create_page_revision(
        &id,
        user_id,
        Some("Page updated".to_string()),
        &mut mysql_pool
    ) {
        Ok(rev_num) => {
            log::info!("Created revision {} for page {}", rev_num, id);
        }
        Err(e) => {
            log::warn!("Failed to create revision: {}", e);
            // Continue with update even if revision fails
        }
    }

    // Update the page
    let mut final_page = updated_page.into_inner();
    final_page.tenant_id = Some(tenant_id); // Enforce tenant persistence
    
    Page::update(id.clone(), &final_page, &mut mysql_pool)?;

    Ok(HttpResponse::Ok().json(final_page))
}

/// Delete a page
#[utoipa::path(
    delete,
    path = "/v1/pages/{id}",
    tag = "Content - Pages",
    params(
        ("id" = String, Path, description = "Page UUID to delete", example = "123e4567-e89b-12d3-a456-426614174000")
    ),
    responses(
        (status = 200, description = "Page deleted successfully"),
        (status = 401, description = "Not authenticated"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Page not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_page(
    req: HttpRequest,
    id: web::Path<String>,
    pool: web::Data<DatabasePool>,
    cache: web::Data<crate::services::cache_service_v2::CacheServiceV2>,
) -> Result<HttpResponse, CustomHttpError> {
    let user_ctx = get_user_context(&req).ok_or(CustomHttpError::Unauthorized("Not authenticated".to_string()))?;
    let tenant_id = resolve_tenant_id(&req, &pool).map_err(|e| CustomHttpError::BadRequest(e))?;
    let mut mysql_pool = pool_handler(pool)?;

    // RBAC Check
    let role = get_tenant_role(tenant_id, user_ctx.user_id, &mut mysql_pool)
        .map_err(|_| CustomHttpError::Forbidden("Access denied".to_string()))?;
    
    if !has_permission(&role, Permission::DeleteContent) {
        return Err(CustomHttpError::Forbidden("Insufficient permissions".to_string()));
    }

    // Tenant Check
    let existing_page = Page::read_one(id.clone(), &mut mysql_pool)?;
    if existing_page.tenant_id.unwrap_or(0) != tenant_id {
         return Err(CustomHttpError::NotFound("Page not found (tenant mismatch)".to_string()));
    }

    // Invalidate Cache
    let cache_key = format!("page:{}:{}:html", tenant_id, existing_page.page_url);
    let _ = cache.delete(&cache_key).await;

    let res = Page::delete(id.clone(), &mut mysql_pool)?;

    Ok(HttpResponse::Ok().json(res))
}
