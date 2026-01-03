// Page Status Enum - PostgreSQL VARCHAR mapping
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Text;
use diesel::expression::AsExpression;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsExpression, diesel::query_builder::QueryId, ToSchema)]
#[diesel(sql_type = crate::schema::sql_types::PagesStatus)]
#[serde(rename_all = "lowercase")]
pub enum PageStatus {
    Draft,
    Scheduled,
    Published,
    Archived,
}

impl PageStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            PageStatus::Draft => "draft",
            PageStatus::Scheduled => "scheduled",
            PageStatus::Published => "published",
            PageStatus::Archived => "archived",
        }
    }
}


// PostgreSQL implementations
impl FromSql<crate::schema::sql_types::PagesStatus, Pg> for PageStatus {
    fn from_sql(bytes: diesel::backend::RawValue<Pg>) -> deserialize::Result<Self> {
        let text = <String as FromSql<Text, Pg>>::from_sql(bytes)?;
        
        match text.as_str() {
            "draft" => Ok(PageStatus::Draft),
            "scheduled" => Ok(PageStatus::Scheduled),
            "published" => Ok(PageStatus::Published),
            "archived" => Ok(PageStatus::Archived),
            other => Err(format!("Unrecognized enum variant: {}", other).into()),
        }
    }
}

impl ToSql<crate::schema::sql_types::PagesStatus, Pg> for PageStatus {
    fn to_sql(&self, out: &mut Output<Pg>) -> serialize::Result {
        <str as ToSql<Text, Pg>>::to_sql(self.as_str(), out)
    }
}
