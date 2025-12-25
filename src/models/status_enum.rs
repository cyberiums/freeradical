// Page Status Enum - Maps to MySQL ENUM('draft', 'scheduled', 'published', 'archived')
use diesel::deserialize::{self, FromSql};
use diesel::mysql::Mysql;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Text;
use diesel::expression::AsExpression;
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsExpression)]
#[diesel(sql_type = crate::schema::sql_types::PagesStatusEnum)]
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

// Implement FromSql to read from database
impl FromSql<crate::schema::sql_types::PagesStatusEnum, Mysql> for PageStatus {
    fn from_sql(bytes: diesel::backend::RawValue<Mysql>) -> deserialize::Result<Self> {
        let bytes_ref = <*const [u8] as FromSql<diesel::sql_types::Binary, Mysql>>::from_sql(bytes)?;
        let bytes_slice = unsafe { &*bytes_ref };
        
        match bytes_slice {
            b"draft" => Ok(PageStatus::Draft),
            b"scheduled" => Ok(PageStatus::Scheduled),
            b"published" => Ok(PageStatus::Published),
            b"archived" => Ok(PageStatus::Archived),
            other => Err(format!(
                "Unrecognized enum variant: {}",
                String::from_utf8_lossy(other)
            )
            .into()),
        }
    }
}

// Implement ToSql to write to database
impl ToSql<crate::schema::sql_types::PagesStatusEnum, Mysql> for PageStatus {
    fn to_sql(&self, out: &mut Output<Mysql>) -> serialize::Result {
        out.write_all(self.as_str().as_bytes())?;
        Ok(serialize::IsNull::No)
    }
}
