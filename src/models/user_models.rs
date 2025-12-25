use super::Model;
use super::PooledDatabaseConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, Serialize, Deserialize)]
#[diesel(primary_key(uuid))]
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
    fn create(new: &MutUser, db: &mut PooledDatabaseConnection) -> Result<usize, diesel::result::Error> {
        match db {
            PooledDatabaseConnection::MySQL(ref mut conn) => {
                diesel::insert_into(users::table).values(new).execute(conn)
            }
            PooledDatabaseConnection::Postgres(ref mut conn) => {
                diesel::insert_into(users::table).values(new).execute(conn)
            }
        }
    }

    fn read_one(id: String, db: &mut PooledDatabaseConnection) -> Result<User, diesel::result::Error> {
        use users::dsl::username;

        match db {
            PooledDatabaseConnection::MySQL(ref mut conn) => {
                users::table
                    .filter(username.eq(id))
                    .select(User::as_select())
                    .first(conn)
            }
            PooledDatabaseConnection::Postgres(ref mut conn) => {
                users::table
                    .filter(username.eq(id))
                    .select(User::as_select())
                    .first(conn)
            }
        }
    }

    fn read_all(_: &mut PooledDatabaseConnection) -> Result<Vec<User>, diesel::result::Error> {
        unimplemented!()
    }

    fn update(
        id: String,
        new: &MutUser,
        db: &mut PooledDatabaseConnection,
    ) -> Result<usize, diesel::result::Error> {
        use users::dsl::username;
        match db {
            PooledDatabaseConnection::MySQL(ref mut conn) => {
                diesel::update(users::table.filter(username.eq(id)))
                    .set(new)
                    .execute(conn)
            }
            PooledDatabaseConnection::Postgres(ref mut conn) => {
                diesel::update(users::table.filter(username.eq(id)))
                    .set(new)
                    .execute(conn)
            }
        }
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

        match db {
            PooledDatabaseConnection::MySQL(ref mut conn) => {
                diesel::update(users::table.filter(username.eq(new.username.clone())))
                    .set(new)
                    .execute(conn)
            }
            PooledDatabaseConnection::Postgres(ref mut conn) => {
                diesel::update(users::table.filter(username.eq(new.username.clone())))
                    .set(new)
                    .execute(conn)
            }
        }
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
