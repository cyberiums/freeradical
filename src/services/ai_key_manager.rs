use actix_web::web;
use diesel::prelude::*;
use log::{info, error};

use crate::models::db_connection::DatabasePool;
use crate::services::encryption_service;
use crate::services::errors_service::CustomHttpError;

/// AI Provider Key Manager
/// Securely stores and manages API keys for AI providers using AES-256-GCM encryption
pub struct AIKeyManager;

/// AI Provider enum
#[derive(Debug, Clone, PartialEq)]
pub enum AIProviderType {
    OpenAI,
    Anthropic,
    Google,
    Custom(String),
}

impl AIProviderType {
    pub fn as_str(&self) -> &str {
        match self {
            AIProviderType::OpenAI => "openai",
            AIProviderType::Anthropic => "anthropic",
            AIProviderType::Google => "google",
            AIProviderType::Custom(name) => name.as_str(),
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "openai" => AIProviderType::OpenAI,
            "anthropic" => AIProviderType::Anthropic,
            "google" => AIProviderType::Google,
            _ => AIProviderType::Custom(s.to_string()),
        }
    }
}

/// Stored provider key
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::ai_provider_keys)]
pub struct AIProviderKey {
    pub id: i32,
    pub provider_name: String,
    pub key_name: String,
    pub encrypted_key: String,
    pub is_active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub rotated_at: Option<chrono::NaiveDateTime>,
    pub last_used_at: Option<chrono::NaiveDateTime>,
    pub request_count: i32,
    pub token_count: i64,
    pub notes: Option<String>,
}

/// New provider key for insertion
#[derive(Insertable)]
#[diesel(table_name = crate::schema::ai_provider_keys)]
pub struct NewAIProviderKey {
    pub provider_name: String,
    pub key_name: String,
    pub encrypted_key: String,
    pub notes: Option<String>,
}

impl AIKeyManager {
    /// Create a new AI key manager
    pub fn new() -> Self {
        info!("✅ AI Key Manager initialized");
        Self
    }

    /// Store a new API key (encrypted)
    pub async fn store_key(
        &self,
        pool: web::Data<DatabasePool>,
        provider: AIProviderType,
        key_name: String,
        api_key: String,
        notes: Option<String>,
    ) -> Result<i32, CustomHttpError> {
        use crate::schema::ai_provider_keys;

        info!("Storing new API key for provider: {}", provider.as_str());

        // Encrypt the API key
        let encrypted_key = encryption_service::encrypt(&api_key)
            .map_err(|e| {
                error!("Encryption failed: {}", e);
                CustomHttpError::InternalServerError(format!("Encryption error: {}", e))
            })?;

        let new_key = NewAIProviderKey {
            provider_name: provider.as_str().to_string(),
            key_name,
            encrypted_key,
            notes,
        };

        web::block(move || -> Result<i32, diesel::result::Error> {
            let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;

            diesel::insert_into(ai_provider_keys::table)
                .values(&new_key)
                .returning(ai_provider_keys::id)
                .get_result(&mut conn)
        })
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))?
        .map_err(|e| {
            error!("Database error storing key: {}", e);
            CustomHttpError::InternalServerError(format!("Database error: {}", e))
        })
    }

    /// Retrieve and decrypt an API key
    pub async fn get_key(
        &self,
        pool: web::Data<DatabasePool>,
        provider: AIProviderType,
        name_filter: Option<String>,
    ) -> Result<String, CustomHttpError> {
        use crate::schema::ai_provider_keys::dsl::*;

        info!("Retrieving API key for provider: {}", provider.as_str());

        let provider_str = provider.as_str().to_string();

        let encrypted = web::block(move || -> Result<String, diesel::result::Error> {
            let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;

            let mut query = ai_provider_keys
                .filter(provider_name.eq(&provider_str))
                .filter(is_active.eq(true))
                .select(encrypted_key)
                .order(last_used_at.desc().nulls_last())
                .into_boxed();

            // Apply optional name filter
            if let Some(name_val) = name_filter {
                query = query.filter(key_name.eq(name_val));
            }

            query.first(&mut conn)
        })
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))?
        .map_err(|e| {
            error!("Key not found: {}", e);
            CustomHttpError::NotFound(format!("No active key found for provider: {}", provider.as_str()))
        })?;

        // Decrypt the key
        encryption_service::decrypt(&encrypted)
            .map_err(|e| {
                error!("Decryption failed: {}", e);
                CustomHttpError::InternalServerError(format!("Decryption error: {}", e))
            })
    }

    /// Update usage statistics
    pub async fn track_usage(
        &self,
        pool: web::Data<DatabasePool>,
        provider: AIProviderType,
        key_name_opt: Option<String>,
        tokens_used: usize,
    ) -> Result<(), CustomHttpError> {
        use crate::schema::ai_provider_keys::dsl::*;

        let provider_str = provider.as_str().to_string();

        web::block(move || -> Result<(), diesel::result::Error> {
            let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;

            let base_filter = ai_provider_keys
                .filter(provider_name.eq(&provider_str))
                .filter(is_active.eq(true));

            if let Some(name) = key_name_opt {
                diesel::update(base_filter.filter(key_name.eq(name)))
                    .set((
                        request_count.eq(request_count + 1),
                        token_count.eq(token_count + tokens_used as i64),
                        last_used_at.eq(chrono::Utc::now().naive_utc()),
                    ))
                    .execute(&mut conn)?;
            } else {
                diesel::update(base_filter)
                    .set((
                        request_count.eq(request_count + 1),
                        token_count.eq(token_count + tokens_used as i64),
                        last_used_at.eq(chrono::Utc::now().naive_utc()),
                    ))
                    .execute(&mut conn)?;
            }

            Ok(())
        })
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))?
        .map_err(|e| CustomHttpError::InternalServerError(format!("Usage tracking error: {}", e)))
    }

    /// Rotate an API key
    pub async fn rotate_key(
        &self,
        pool: web::Data<DatabasePool>,
        key_id: i32,
        new_api_key: String,
        reason: Option<String>,
    ) -> Result<(), CustomHttpError> {
        use crate::schema::{ai_provider_keys, ai_key_rotation_history};

        info!("Rotating API key ID: {}", key_id);

        // Encrypt the new key
        let encrypted_key = encryption_service::encrypt(&new_api_key)
            .map_err(|e| CustomHttpError::InternalServerError(format!("Encryption error: {}", e)))?;

        web::block(move || -> Result<(), diesel::result::Error> {
            let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;

            // Start transaction
            conn.transaction(|conn| {
                // Update the key
                diesel::update(ai_provider_keys::table.filter(ai_provider_keys::id.eq(key_id)))
                    .set((
                        ai_provider_keys::encrypted_key.eq(&encrypted_key),
                        ai_provider_keys::rotated_at.eq(chrono::Utc::now().naive_utc()),
                        ai_provider_keys::updated_at.eq(chrono::Utc::now().naive_utc()),
                    ))
                    .execute(conn)?;

                // Record rotation history
                diesel::insert_into(ai_key_rotation_history::table)
                    .values((
                        ai_key_rotation_history::provider_key_id.eq(key_id),
                        ai_key_rotation_history::reason.eq(reason),
                    ))
                    .execute(conn)?;

                Ok(())
            })
        })
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))?
        .map_err(|e| {
            error!("Key rotation failed: {}", e);
            CustomHttpError::InternalServerError(format!("Rotation error: {}", e))
        })
    }

    /// Deactivate a key
    pub async fn deactivate_key(
        &self,
        pool: web::Data<DatabasePool>,
        key_id: i32,
    ) -> Result<(), CustomHttpError> {
        use crate::schema::ai_provider_keys::dsl::*;

        info!("Deactivating API key ID: {}", key_id);

        web::block(move || -> Result<(), diesel::result::Error> {
            let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;

            diesel::update(ai_provider_keys.filter(id.eq(key_id)))
                .set(is_active.eq(false))
                .execute(&mut conn)?;

            Ok(())
        })
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))?
        .map_err(|e| CustomHttpError::InternalServerError(format!("Deactivation error: {}", e)))
    }

    /// List all keys for a provider
    pub async fn list_keys(
        &self,
        pool: web::Data<DatabasePool>,
        provider: AIProviderType,
    ) -> Result<Vec<AIProviderKey>, CustomHttpError> {
        use crate::schema::ai_provider_keys::dsl::*;

        let provider_str = provider.as_str().to_string();

        web::block(move || -> Result<Vec<AIProviderKey>, diesel::result::Error> {
            let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;

            ai_provider_keys
                .filter(provider_name.eq(&provider_str))
                .order(created_at.desc())
                .load::<AIProviderKey>(&mut conn)
        })
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))?
        .map_err(|e| CustomHttpError::InternalServerError(format!("List error: {}", e)))
    }
}

impl Default for AIKeyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_type_conversion() {
        assert_eq!(AIProviderType::OpenAI.as_str(), "openai");
        assert_eq!(AIProviderType::from_str("openai"), AIProviderType::OpenAI);
        assert_eq!(AIProviderType::from_str("anthropic"), AIProviderType::Anthropic);
    }
}
