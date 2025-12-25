use std::sync::Mutex;

use actix_web::{web, HttpRequest, HttpResponse};
use handlebars::Handlebars;
use uuid::Uuid;

use crate::models::{pool_handler, Model, MySQLPool};

use crate::models::module_models::{FieldsDTO};
use crate::models::page_models::{PageModuleDisplayDTO,MutPage, Page, PageDTO};

use crate::services::auth_service::Claims;
use crate::services::errors_service::CustomHttpError;

/// Validates SEO fields to ensure they meet requirements
fn validate_seo_fields(page: &MutPage) -> Result<(), CustomHttpError> {
    // Validate meta_title length (max 70 chars)
    if let Some(title) = &page.meta_title {
        if title.len() > 70 {
            return Err(CustomHttpError::BadRequest);
        }
    }
    
    // Validate meta_description length (max 160 chars)
    if let Some(desc) = &page.meta_description {
        if desc.len() > 160 {
            return Err(CustomHttpError::BadRequest);
        }
    }
    
    // Validate og_title length (max 70 chars)
    if let Some(title) = &page.og_title {
        if title.len() > 70 {
            return Err(CustomHttpError::BadRequest);
        }
    }
    
    // Validate og_description length (max 200 chars)
    if let Some(desc) = &page.og_description {
        if desc.len() > 200 {
            return Err(CustomHttpError::BadRequest);
        }
    }
    
    // Validate twitter_title length (max 70 chars)
    if let Some(title) = &page.twitter_title {
        if title.len() > 70 {
            return Err(CustomHttpError::BadRequest);
        }
    }
    
    // Validate twitter_description length (max 200 chars)
    if let Some(desc) = &page.twitter_description {
        if desc.len() > 200 {
            return Err(CustomHttpError::BadRequest);
        }
    }
    
    // Validate canonical_url format (must be valid URL or relative path)
    if let Some(url) = &page.canonical_url {
        if !url.starts_with("http://") && !url.starts_with("https://") && !url.starts_with("/") {
            return Err(CustomHttpError::BadRequest);
        }
    }
    
    Ok(())
}

fn parse_page(page: (Page, FieldsDTO)) -> Result<PageModuleDisplayDTO, CustomHttpError> {
    let origin_page = page.0;

    // cast the origin page that is always standard into a new object that has the modules as a vec of children.
    let mut res: PageModuleDisplayDTO = origin_page.into();

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
    pool: web::Data<MySQLPool>,
    hb: web::Data<Mutex<Handlebars<'_>>>,
) -> Result<HttpResponse, CustomHttpError> {
    let mut mysql_pool = pool_handler(pool)?;
    let path = req.path();
    let page_tuple = Page::read_one_join_on_url(path.to_string(), &mut mysql_pool);

    if let Err(_) = page_tuple {
        let s = hb.lock().unwrap().render("404", &String::from("")).unwrap();
        return Ok(HttpResponse::Ok().content_type("text/html").body(s));
    }

    let pagemodule = parse_page(page_tuple?)?;

    let s = hb
        .lock()
        .unwrap()
        .render(&pagemodule.page_name, &pagemodule)
        .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn create_page(
    new: web::Json<MutPage>,
    pool: web::Data<MySQLPool>,
    _: Claims
) -> Result<HttpResponse, CustomHttpError> {
    let mut mysql_pool = pool_handler(pool)?;

    // Validate SEO fields
    validate_seo_fields(&new)?;

    let mut uuid_new = new.clone();
    uuid_new.uuid = Some(Uuid::new_v4().to_string());

    Page::create(&uuid_new, &mut mysql_pool)?;

    Ok(HttpResponse::Ok().json(uuid_new))
}

pub async fn get_pages(pool: web::Data<MySQLPool>) -> Result<HttpResponse, CustomHttpError> {
    let mut mysql_pool = pool_handler(pool)?;
    let pages: Vec<PageDTO> = Page::read_all(&mut mysql_pool)?;

    Ok(HttpResponse::Ok().json(pages))

}

pub async fn get_page(
    id: web::Path<String>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mut mysql_pool = pool_handler(pool)?;

    let page: PageDTO = Page::read_one(id.clone(), &mut mysql_pool)?;
    Ok(HttpResponse::Ok().json(page))

}

pub async fn get_page_join_modules(
    id: web::Path<String>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mut mysql_pool = pool_handler(pool)?;

    let page_vec = Page::read_one_join_on(id.clone(), &mut mysql_pool)?;

    Ok(HttpResponse::Ok().json(page_vec))
}

pub async fn update_page(
    updated_page: web::Json<MutPage>,
    id: web::Path<String>,
    pool: web::Data<MySQLPool>,
    claims: Claims
) -> Result<HttpResponse, CustomHttpError> {
    let mut mysql_pool = pool_handler(pool)?;

    // Validate SEO fields
    validate_seo_fields(&updated_page)?;

    // Create revision BEFORE updating the page
    let user_id = Some(claims.sub.parse::<i32>().unwrap_or(0));
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
    Page::update(id.clone(), &updated_page, &mut mysql_pool)?;

    Ok(HttpResponse::Ok().json(updated_page.0))

}

pub async fn delete_page(
    id: web::Path<String>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mut mysql_pool = pool_handler(pool)?;

    let res = Page::delete(id.clone(), &mut mysql_pool)?;

    Ok(HttpResponse::Ok().json(res))
}
