use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::email_templates;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Clone)]
#[diesel(table_name = email_templates)]
pub struct EmailTemplate {
    pub id: i32,
    pub tenant_id: Option<i32>,
    pub template_key: String,
    pub subject: String,
    pub body_template: String,
    pub template_type: String,
    pub is_active: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = email_templates)]
pub struct NewEmailTemplate {
    pub tenant_id: Option<i32>,
    pub template_key: String,
    pub subject: String,
    pub body_template: String,
    pub template_type: String,
    pub is_active: bool,
}

impl EmailTemplate {
    /// Get email template by key, with tenant fallback to global default
    pub fn get_by_key(
        key: &str,
        tid: Option<i32>,
        conn: &mut PgConnection,
    ) -> QueryResult<Self> {
        use crate::schema::email_templates::dsl;

        // Try tenant-specific template first
        if let Some(t_id) = tid {
            if let Ok(template) = dsl::email_templates
                .filter(dsl::template_key.eq(key))
                .filter(dsl::tenant_id.eq(Some(t_id)))
                .filter(dsl::is_active.eq(true))
                .first::<EmailTemplate>(conn)
            {
                return Ok(template);
            }
        }

        // Fallback to global default (tenant_id IS NULL)
        dsl::email_templates
            .filter(dsl::template_key.eq(key))
            .filter(dsl::tenant_id.is_null())
            .filter(dsl::is_active.eq(true))
            .first::<EmailTemplate>(conn)
    }

    /// Create or update email template
    pub fn upsert(
        new_template: NewEmailTemplate,
        conn: &mut PgConnection,
    ) -> QueryResult<Self> {
        use crate::schema::email_templates::dsl;

        diesel::insert_into(dsl::email_templates)
            .values(&new_template)
            .on_conflict((dsl::tenant_id, dsl::template_key))
            .do_update()
            .set((
                dsl::subject.eq(&new_template.subject),
                dsl::body_template.eq(&new_template.body_template),
                dsl::updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .get_result(conn)
    }
}
