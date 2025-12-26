use diesel::prelude::*;
use diesel::{Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use super::page_models::Page;
use super::{Model, PooledDatabaseConnection};
use crate::schema::module_category;
use crate::schema::modules;

#[derive(Debug, Identifiable, Associations, Serialize, Deserialize, Queryable, Selectable, PartialEq, Clone, Eq, Hash)]
#[diesel(belongs_to(Page, foreign_key = page_uuid))]
#[diesel(belongs_to(ModuleCategory, foreign_key = category_uuid))]
#[diesel(primary_key(uuid))]
#[diesel(table_name = modules)]
pub struct Module {
    pub uuid: String,
    pub page_uuid: String,
    pub category_uuid: Option<String>,
    pub title: String,
    pub content: String,
    pub field_type: Option<String>,
    pub field_config: Option<String>,
    pub validation_rules: Option<String>,
}

#[derive(Insertable, AsChangeset, Deserialize, Serialize, Clone)]
#[diesel(table_name = modules)]
pub struct MutModule {
    pub uuid: Option<String>,
    pub title: String,
    pub page_uuid: String,
    pub category_uuid: Option<String>,
    pub content: String,
    pub field_type: Option<String>,
    pub field_config: Option<String>,
    pub validation_rules: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoryDTO {
    pub uuid: String,
    pub title: String,
    pub modules: Vec<Module>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FieldsDTO {
    pub modules: Vec<Module>,
    pub categories: Option<Vec<CategoryDTO>>
}

#[derive(
    Debug, Identifiable, Associations, Serialize, Deserialize, Queryable, Selectable, PartialEq, Clone, Eq, Hash,
)]
#[diesel(primary_key(uuid))]
#[diesel(belongs_to(Page, foreign_key = page_uuid))]
#[diesel(table_name = module_category)]
pub struct ModuleCategory {
    pub uuid: String,
    pub page_uuid: String,
    pub title: String
}

#[derive(
    Debug, Serialize, Deserialize, AsChangeset, Insertable, PartialEq, Clone, Eq, Hash,
)]
#[diesel(table_name = module_category)]
pub struct MutCategory {
    pub title: String,
    pub page_uuid: String,
    pub uuid: Option<String>
}

impl ModuleCategory {
    pub fn join(_id: String, db: &mut PooledDatabaseConnection) -> Result<Vec<Module>, diesel::result::Error> {
        use module_category::dsl::uuid;
        let categories = module_category::table.filter(uuid.eq(_id)).first::<Self>(db)?;
        Module::belonging_to(&categories).load::<Module>(db)
    }
}

impl Model<Self, MutCategory, String, ModuleCategory> for ModuleCategory {
    fn create(new: &MutCategory, db: &mut PooledDatabaseConnection) -> Result<usize, diesel::result::Error> {
        diesel::insert_into(module_category::table)
            .values(new)
            .on_conflict_do_nothing()
            .execute(db)
    }

    fn read_one(_id: String, db: &mut PooledDatabaseConnection) -> Result<ModuleCategory, diesel::result::Error> {
        use module_category::dsl::uuid;
        module_category::table.filter(uuid.eq(_id)).first::<ModuleCategory>(db)
    }

    fn read_all(_db: &mut PooledDatabaseConnection) -> Result<Vec<ModuleCategory>, diesel::result::Error> {
        unimplemented!()
    }

    fn update(
        _id: String,
        new: &MutCategory,
        db: &mut PooledDatabaseConnection,
    ) -> Result<usize, diesel::result::Error> {
        use module_category::dsl::uuid;
        diesel::update(module_category::table.filter(uuid.eq(_id)))
            .set(new)
            .execute(db)
    }

    fn delete(_id: String, db: &mut PooledDatabaseConnection) -> Result<usize, diesel::result::Error> {
        use module_category::dsl::uuid;
        diesel::delete(module_category::table.filter(uuid.eq(_id))).execute(db)
    }
}

impl Model<Self, MutModule, String, Module> for Module {
    fn create(
        new_module: &MutModule,
        db: &mut PooledDatabaseConnection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::insert_into(modules::table)
            .values(new_module)
            .execute(db)
    }

    fn read_one(mod_id: String, db: &mut PooledDatabaseConnection) -> Result<Module, diesel::result::Error> {
        use modules::dsl::uuid;
        modules::table.filter(uuid.eq(mod_id)).first::<Self>(db)
    }

    fn read_all(db: &mut PooledDatabaseConnection) -> Result<Vec<Module>, diesel::result::Error> {
        use modules::dsl::category_uuid;
        modules::table
            .filter(category_uuid.is_null())
            .load::<Module>(db)
            .map(|modules| modules.into_iter().map(|m| m.into()).collect())
    }

    fn delete(mod_id: String, db: &mut PooledDatabaseConnection) -> Result<usize, diesel::result::Error> {
        use modules::dsl::uuid;
        diesel::delete(modules::table.filter(uuid.eq(mod_id))).execute(db)
    }

    fn update(
        mod_id: String,
        new_module: &MutModule,
        db: &mut PooledDatabaseConnection,
    ) -> Result<usize, diesel::result::Error> {
        use modules::dsl::uuid;
        diesel::update(modules::table.filter(uuid.eq(mod_id)))
            .set(new_module)
            .execute(db)
    }
}