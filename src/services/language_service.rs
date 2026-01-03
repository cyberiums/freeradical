use diesel::prelude::*;

/// Language service for managing multi-language support
pub struct LanguageService;

impl LanguageService {
    /// Get all enabled languages
    pub fn get_enabled_languages(conn: &mut diesel::pg::PgConnection) -> Result<Vec<Language>, diesel::result::Error> {
        use crate::schema::languages::dsl::*;
        
        languages
            .filter(enabled.eq(true))
            .order(is_default.desc())
            .load::<Language>(conn)
    }
    
    /// Get default language
    pub fn get_default_language(conn: &mut diesel::pg::PgConnection) -> Result<Language, diesel::result::Error> {
        use crate::schema::languages::dsl::*;
        
        languages
            .filter(is_default.eq(true))
            .first::<Language>(conn)
    }
    
    /// Create a new language
    pub fn create_language(
        conn: &mut diesel::pg::PgConnection,
        new_lang: NewLanguage
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::languages;
        
        diesel::insert_into(languages::table)
            .values(&new_lang)
            .execute(conn)
    }
    
    /// Get language by code
    pub fn get_language_by_code(
        conn: &mut diesel::pg::PgConnection,
        language_code: &str
    ) -> Result<Option<Language>, diesel::result::Error> {
        use crate::schema::languages::dsl::*;
        
        languages
            .filter(code.eq(language_code))
            .first::<Language>(conn)
            .optional()
    }
    
    /// Get translation for a page
    pub fn get_page_translation(
        conn: &mut diesel::pg::PgConnection,
        target_page_id: i32,
        lang_id: i32
    ) -> Result<Option<PageTranslation>, diesel::result::Error> {
        use crate::schema::page_translations::dsl::*;
        
        page_translations
            .filter(page_id.eq(target_page_id))
            .filter(language_id.eq(lang_id))
            .first::<PageTranslation>(conn)
            .optional()
    }
    
    /// Save page translation
    pub fn save_page_translation(
        conn: &mut diesel::pg::PgConnection,
        translation: NewPageTranslation
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::page_translations;
        
        diesel::insert_into(page_translations::table)
            .values(&translation)
            .execute(conn)
    }
}

// Re-export models
pub use crate::models::language_models::{Language, NewLanguage};
pub use crate::models::translation_models::{PageTranslation, NewPageTranslation};
