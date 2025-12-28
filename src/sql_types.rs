use diesel::sql_types::SqlType;

#[derive(SqlType)]
#[diesel(postgres_type(name = "vector"))]
pub struct Vector;
