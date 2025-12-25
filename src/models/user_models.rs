use super::Model;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, Serialize, Deserialize)]
#[diesel(primary_key(uuid))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(table_name = users)]
pub struct User {
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub token: Option<String>,
    pub two_factor_secret: Option<String>,
    pub two_factor_enabled: bool,
}

#[derive(Debug, AsChangeset, Insertable, Clone, Serialize, Deserialize)]
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
    fn create(new: &MutUser, db: &mut diesel::MysqlConnection) -> Result<usize, diesel::result::Error> {
        diesel::insert_into(users::table).values(new).execute(db)
    }

    fn read_one(id: String, db: &mut diesel::MysqlConnection) -> Result<User, diesel::result::Error> {
        use users::dsl::username;

        Ok(users::table
            .filter(username.eq(id))
            .select(User::as_select())
            .first(db)?)
    }

    fn read_all(_: &mut diesel::MysqlConnection) -> Result<Vec<User>, diesel::result::Error> {
        unimplemented!()
    }

    fn update(
        id: String,
        new: &MutUser,
        db: &mut diesel::MysqlConnection,
    ) -> Result<usize, diesel::result::Error> {
        use users::dsl::username;
        let update = diesel::update(users::table.filter(username.eq(id)))
            .set(new)
            .execute(db)?;

        Ok(update)
    }

    fn delete(_: String, _: &mut diesel::MysqlConnection) -> Result<usize, diesel::result::Error> {
        todo!()
    }
}

impl User {
    pub fn update_with_token(
        new: &MutUser,
        db: &mut diesel::MysqlConnection,
    ) -> Result<usize, diesel::result::Error> {
        use users::dsl::username;

        let res = diesel::update(users::table.filter(username.eq(new.username.clone())))
            .set(new)
            .execute(db)?;

        Ok(res)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub two_factor_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enable2faRequest {
    pub secret: String,
    pub code: String,
}
