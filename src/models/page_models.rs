use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

use super::module_models::Module;
use super::status_enum::PageStatus;  // Re-enabled
use super::Model;
use super::PooledDatabaseConnection;
use crate::models::module_models::CategoryDTO;
use crate::models::module_models::FieldsDTO;
use crate::models::module_models::ModuleCategory;
use crate::schema::module_category;
use crate::schema::modules;
use crate::schema::pages;

#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable, Selectable, PartialEq, Clone, ToSchema)]
#[diesel(table_name = pages)]
#[diesel(primary_key(uuid))]
pub struct Page {
    pub uuid: String,
    pub page_name: String,
    pub page_url: String,
    pub page_title: String,
    pub time_created: NaiveDateTime,
    // Group 1: SEO Metadata - CLEAN ✅
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub canonical_url: Option<String>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image: Option<String>,
    pub twitter_card: Option<String>,
    pub twitter_title: Option<String>,
    pub twitter_description: Option<String>,
    // Group 2: Article Info - CLEAN ✅
    pub author: Option<String>,
    pub article_type: Option<String>,
    pub featured_image: Option<String>,
    pub word_count: Option<i32>,
    pub reading_time: Option<i32>,
    // Iteration 4 fields - MUST match schema.rs exact order!
    pub current_revision: Option<i32>,
    pub last_modified_by: Option<i32>,
    pub status: Option<PageStatus>,
    pub publish_at: Option<NaiveDateTime>,
    pub unpublish_at: Option<NaiveDateTime>,
    pub tenant_id: Option<i32>,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = pages)]
pub struct MutPage {
    pub uuid: Option<String>,
    pub page_name: String,
    pub page_url: String,
    pub page_title: String,
    // SEO Fields
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub canonical_url: Option<String>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image: Option<String>,
    pub twitter_card: Option<String>,
    pub twitter_title: Option<String>,
    pub twitter_description: Option<String>,
    // Article Info
    pub author: Option<String>,
    pub article_type: Option<String>,
    pub featured_image: Option<String>,
    pub word_count: Option<i32>,
    pub reading_time: Option<i32>,
    // Publishing
    pub current_revision: Option<i32>,
    pub last_modified_by: Option<i32>,
    pub status: Option<PageStatus>,
    pub publish_at: Option<NaiveDateTime>,
    pub unpublish_at: Option<NaiveDateTime>,
    pub tenant_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct PageDTO {
    pub uuid: String,
    pub page_name: String,
    pub page_url: String,
    pub page_title: String,
    pub time_created: NaiveDateTime,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub canonical_url: Option<String>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image: Option<String>,
    pub twitter_card: Option<String>,
    pub twitter_title: Option<String>,
    pub twitter_description: Option<String>,
    pub author: Option<String>,
    pub article_type: Option<String>,
    pub featured_image: Option<String>,
    pub word_count: Option<i32>,
    pub reading_time: Option<i32>,
    pub current_revision: Option<i32>,
    pub last_modified_by: Option<i32>,
    pub status: Option<PageStatus>,
    pub publish_at: Option<NaiveDateTime>,
    pub unpublish_at: Option<NaiveDateTime>,
    pub tenant_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct PageModuleDTO {
    pub uuid: String,
    pub page_name: String,
    pub page_url: String,
    pub page_title: String,
    pub time_created: NaiveDateTime,
    pub fields: HashMap<String, Module>,
    pub array_fields: HashMap<String, Vec<Module>>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub canonical_url: Option<String>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image: Option<String>,
    pub twitter_card: Option<String>,
    pub twitter_title: Option<String>,
    pub twitter_description: Option<String>,
    pub current_revision: Option<i32>,
    pub last_modified_by: Option<i32>,
    pub status: Option<PageStatus>,
}

impl From<Page> for PageDTO {
    fn from(page: Page) -> PageDTO {
        PageDTO {
            uuid: page.uuid,
            page_name: page.page_name,
            page_url: page.page_url,
            page_title: page.page_title,
            time_created: page.time_created,
            meta_title: page.meta_title,
            meta_description: page.meta_description,
            meta_keywords: page.meta_keywords,
            canonical_url: page.canonical_url,
            og_title: page.og_title,
            og_description: page.og_description,
            og_image: page.og_image,
            twitter_card: page.twitter_card,
            twitter_title: page.twitter_title,
            twitter_description: page.twitter_description,
            author: page.author,
            article_type: page.article_type,
            featured_image: page.featured_image,
            word_count: page.word_count,
            reading_time: page.reading_time,
            current_revision: page.current_revision,
            last_modified_by: page.last_modified_by,
            status: page.status,
            publish_at: page.publish_at,
            unpublish_at: page.unpublish_at,
            tenant_id: page.tenant_id,
        }
    }
}

impl From<Page> for PageModuleDTO {
    fn from(page: Page) -> PageModuleDTO {
        PageModuleDTO {
            uuid: page.uuid,
            page_name: page.page_name,
            page_url: page.page_url,
            page_title: page.page_title,
            time_created: page.time_created,
            fields: HashMap::new(),
            array_fields: HashMap::new(),
            meta_title: page.meta_title,
            meta_description: page.meta_description,
            meta_keywords: page.meta_keywords,
            canonical_url: page.canonical_url,
            og_title: page.og_title,
            og_description: page.og_description,
            og_image: page.og_image,
            twitter_card: page.twitter_card,
            twitter_title: page.twitter_title,
            twitter_description: page.twitter_description,
            current_revision: page.current_revision,
            last_modified_by: page.last_modified_by,
            status: page.status,
        }
    }
}

impl Model<Page, MutPage, String, PageDTO> for Page {
    fn create(new_page: &MutPage, db: &mut PooledDatabaseConnection) -> Result<usize, diesel::result::Error> {
        diesel::insert_into(pages::table)
            .values(new_page)
            .on_conflict_do_nothing()
            .execute(db)
    }

    fn read_one(_id: String, db: &mut PooledDatabaseConnection) -> Result<PageDTO, diesel::result::Error> {
        use pages::dsl::uuid;
        pages::table.filter(uuid.eq(_id)).first::<Self>(db).map(|p| p.into())
    }

    fn read_all(db: &mut PooledDatabaseConnection) -> Result<Vec<PageDTO>, diesel::result::Error> {
        pages::table
            .select(Page::as_select())
            .load::<Self>(db)
            .map(|pages| pages.into_iter().map(|x| x.into()).collect())
    }


    fn update(
        _id: String,
        new_page: &MutPage,
        db: &mut PooledDatabaseConnection,
    ) -> Result<usize, diesel::result::Error> {
        use pages::dsl::uuid;
        diesel::update(pages::table.filter(uuid.eq(_id)))
            .set(new_page)
            .execute(db)
    }

    fn delete(_id: String, db: &mut PooledDatabaseConnection) -> Result<usize, diesel::result::Error> {
        use pages::dsl::uuid;
        diesel::delete(pages::table.filter(uuid.eq(_id))).execute(db)
    }
}

impl Page {
    pub fn read_one_join_on(
        _id: String,
        db: &mut PooledDatabaseConnection,
    ) -> Result<PageModuleDTO, diesel::result::Error> {
        use pages::dsl::uuid;
        use modules::dsl::category_uuid;

        let filtered_page = pages::table
            .filter(uuid.eq(_id))
            .select(Page::as_select())
            .first::<Page>(db)?;

        let modules_no_category = Module::belonging_to(&filtered_page)
            .filter(category_uuid.is_null())
            .load::<Module>(db)?;

        let categories = ModuleCategory::belonging_to(&filtered_page)
            .load::<ModuleCategory>(db)?;

        let module_array: Vec<(Vec<Module>, ModuleCategory)> = Module::belonging_to(&categories)
            .load::<Module>(db)?
            .grouped_by(&categories)
            .iter()
            .map(|a| a.clone())
            .zip(categories)
            .collect::<Vec<_>>();

        let category_dtos: Vec<CategoryDTO> = module_array
            .iter()
            .map(|a| CategoryDTO {
                title: a.1.title.clone(),
                modules: a.0.clone(),
                uuid: a.1.uuid.clone(),
            })
            .collect::<Vec<_>>();

        let _module_dto = FieldsDTO {
            modules: modules_no_category.into_iter().map(|m| m.into()).collect(),
            categories: Some(category_dtos),
        };

        let page_dto: PageModuleDTO = filtered_page.into();
        // Return FieldsDTO with module structure instead of assigning to fields
        Ok(page_dto)
    }

    /// This is used for displaying a page, rather than getting a page's modules/array modules.
    pub fn read_one_join_on_url(
        id: String,
        db: &mut PooledDatabaseConnection,
    ) -> Result<(Self, FieldsDTO), diesel::result::Error> {
        use crate::schema::pages::dsl::page_url;

        let filtered_page = pages::table
            .filter(page_url.eq(id))
            .select(Page::as_select())
            .first::<Page>(db)?;

        let modules = Module::belonging_to(&filtered_page).load::<Module>(db)?;

        let categories: Vec<ModuleCategory> = Module::belonging_to(&filtered_page)
            .inner_join(module_category::table)
            .select(module_category::all_columns)
            .load::<ModuleCategory>(db)?;

        let module_array: Vec<(Vec<Module>, ModuleCategory)> = Module::belonging_to(&categories)
            .load::<Module>(db)?
            .grouped_by(&categories)
            .into_iter()
            .zip(categories)
            .collect::<Vec<_>>();

        let category_dtos: Vec<CategoryDTO> = module_array
            .iter()
            .map(|a| CategoryDTO {
                uuid: a.1.uuid.clone(),
                title: a.1.title.clone(),
                modules: a.0.clone().into_iter().map(|m| m.into()).collect(),
            })
            .collect::<Vec<_>>();

        let module_dto = FieldsDTO {
            modules: modules.into_iter().map(|m| m.into()).collect(),
            categories: Some(category_dtos),
        };

        Ok((filtered_page, module_dto))
    }

    pub fn read_one_by_tenant_and_url(
        tid: i32,
        url_path: String,
        db: &mut PooledDatabaseConnection,
    ) -> Result<(Self, FieldsDTO), diesel::result::Error> {
        use crate::schema::pages::dsl::{page_url, tenant_id};

        let filtered_page = pages::table
            .filter(page_url.eq(url_path))
            .filter(tenant_id.eq(tid))
            .select(Page::as_select())
            .first::<Page>(db)?;

        let modules = Module::belonging_to(&filtered_page).load::<Module>(db)?;

        let categories: Vec<ModuleCategory> = Module::belonging_to(&filtered_page)
            .inner_join(module_category::table)
            .select(module_category::all_columns)
            .load::<ModuleCategory>(db)?;

        let module_array: Vec<(Vec<Module>, ModuleCategory)> = Module::belonging_to(&categories)
            .load::<Module>(db)?
            .grouped_by(&categories)
            .into_iter()
            .zip(categories)
            .collect::<Vec<_>>();

        let category_dtos: Vec<CategoryDTO> = module_array
            .iter()
            .map(|a| CategoryDTO {
                uuid: a.1.uuid.clone(),
                title: a.1.title.clone(),
                modules: a.0.clone().into_iter().map(|m| m.into()).collect(),
            })
            .collect::<Vec<_>>();

        let module_dto = FieldsDTO {
            modules: modules.into_iter().map(|m| m.into()).collect(),
            categories: Some(category_dtos),
        };

        Ok((filtered_page, module_dto))
    }

    pub fn read_all_by_tenant(tenant_id: i32, db: &mut PooledDatabaseConnection) -> Result<Vec<PageDTO>, diesel::result::Error> {
        pages::table
            .filter(pages::tenant_id.eq(tenant_id))
            .select(Page::as_select())
            .load::<Self>(db)
            .map(|pages| pages.into_iter().map(|x| x.into()).collect())
    }
}
