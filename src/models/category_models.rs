// Category Models
// Simple category model matching module_category table schema

use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    pub uuid: String,
    pub page_uuid: String,
    pub title: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::module_category)]
pub struct NewCategory {
    pub uuid: String,
    pub page_uuid: String,
    pub title: String,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::module_category)]
pub struct UpdateCategory {
    pub page_uuid: Option<String>,
    pub title: Option<String>,
}
