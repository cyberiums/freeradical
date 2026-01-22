use super::Model;
use super::PooledDatabaseConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::schema::users;

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, Serialize, Deserialize, ToSchema)]
#[diesel(primary_key(id))]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub token: Option<String>,
    pub two_factor_secret: Option<String>,
    pub two_factor_enabled: bool,
}

#[derive(Debug, AsChangeset, Insertable, Clone, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = users)]
pub struct MutUser {
    pub uuid: Option<String>,
    pub username: String,
    pub password: Option<String>,
    pub token: Option<String>,
    pub two_factor_secret: Option<String>,
    pub two_factor_enabled: Option<bool>,
}

impl Model<User, MutUser, String> for User {
    fn create(new: &MutUser, db: &mut PooledDatabaseConnection) -> Result<usize, diesel::result::Error> {
        diesel::insert_into(users::table).values(new).execute(db)
    }

    fn read_one(id: String, db: &mut PooledDatabaseConnection) -> Result<User, diesel::result::Error> {
        use users::dsl::username;
        users::table
            .filter(username.eq(id))
            .select(User::as_select())
            .first(db)
    }

    fn read_all(db: &mut PooledDatabaseConnection) -> Result<Vec<User>, diesel::result::Error> {
        users::table
            .select(User::as_select())
            .load(db)
    }

    fn update(
        id: String,
        new: &MutUser,
        db: &mut PooledDatabaseConnection,
    ) -> Result<usize, diesel::result::Error> {
        use users::dsl::username;
        diesel::update(users::table.filter(username.eq(id)))
            .set(new)
            .execute(db)
    }

    fn delete(_: String, _: &mut PooledDatabaseConnection) -> Result<usize, diesel::result::Error> {
        todo!()
    }
}

impl User {
    pub fn update_with_token(
        new: &MutUser,
        db: &mut PooledDatabaseConnection,
    ) -> Result<usize, diesel::result::Error> {
        use users::dsl::username;
        diesel::update(users::table.filter(username.eq(new.username.clone())))
            .set(new)
            .execute(db)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub two_factor_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Enable2faRequest {
    pub secret: String,
    pub code: String,
}
