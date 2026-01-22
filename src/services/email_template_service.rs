use handlebars::Handlebars;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use log::{info, error, warn};
use actix_web::web;
use crate::models::db_connection::DatabasePool;
use crate::models::email_template_model::EmailTemplate;

#[derive(Clone)]
pub struct EmailTemplateService {
    handlebars: Arc<Mutex<Handlebars<'static>>>,
    pool: web::Data<DatabasePool>,
}

impl EmailTemplateService {
    pub fn new(pool: web::Data<DatabasePool>) -> Self {
        // Initialize Handlebars
        let mut handlebars = Handlebars::new();
        
        let template_dir = PathBuf::from("./templates/emails");
        if template_dir.exists() {
             info!("Loading email templates from {:?}", template_dir);
             
             // Simple manual recursion for known depth/structure or use a helper
             // Structure: templates/emails/{category}/{template}.hbs
             if let Ok(entries) = std::fs::read_dir(&template_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        // Scan subdirectory (e.g. "auth", "billing")
                         if let Ok(sub_entries) = std::fs::read_dir(&path) {
                            for sub_entry in sub_entries.flatten() {
                                let sub_path = sub_entry.path();
                                if sub_path.is_file() && sub_path.extension().map_or(false, |e| e == "hbs") {
                                    if let (Some(cat), Some(name)) = (path.file_name().and_then(|s| s.to_str()), sub_path.file_stem().and_then(|s| s.to_str())) {
                                        // Register as "category/name" (e.g. "billing/invoice_paid")
                                        let key = format!("{}/{}", cat, name);
                                        if let Err(e) = handlebars.register_template_file(&key, &sub_path) {
                                            error!("Failed to register template {}: {}", key, e);
                                        } else {
                                            info!("Registered email template: {}", key);
                                        }
                                    }
                                }
                            }
                         }
                    } else if path.is_file() && path.extension().map_or(false, |e| e == "hbs") {
                        // Root level templates
                        if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                             if let Err(e) = handlebars.register_template_file(name, &path) {
                                error!("Failed to register template {}: {}", name, e);
                            }
                        }
                    }
                }
             }
        } else {
            warn!("Email template directory not found: {:?}", template_dir);
        }

        Self {
            handlebars: Arc::new(Mutex::new(handlebars)),
            pool,
        }
    }

    pub fn render<T>(&self, template_name: &str, data: &T, tenant_id: Option<i32>) -> Result<String, String>
    where
        T: Serialize,
    {
        // 1. Check DB for custom template override
        let mut conn = self.pool.get().map_err(|e| format!("DB Connection Failed: {}", e))?;
        
        match EmailTemplate::get_by_key(template_name, tenant_id, &mut conn) {
            Ok(db_template) => {
                // If found in DB, use it directly (skip Handlebars file cache)
                // Note: We need to compile the string template on the fly or cache it.
                // For simplicity/accuracy, we render it on the fly using a temp registry or the main one if possible.
                // Since we have the template string in `db_template.body_template`, we can render it.
                
                let handlebars = self.handlebars.lock().map_err(|e| e.to_string())?;
                handlebars.render_template(&db_template.body_template, data).map_err(|e| {
                     error!("Failed to render DB template '{}': {}", template_name, e);
                     format!("DB Template render error: {}", e)
                })
            },
            Err(_) => {
                // Not found in DB -> Fallback to File
                let handlebars = self.handlebars.lock().map_err(|e| e.to_string())?;
                handlebars.render(template_name, data).map_err(|e| {
                    error!("Failed to render email template '{}': {}", template_name, e);
                    format!("Template render error: {}", e)
                })
            }
        }
    }
    
    // Legacy support: Only used for reloads (usually admin) - keeping FS only for now or TODO: update to clear DB cache if we had one.
    pub fn reload_templates(&self) -> Result<(), String> {
        let mut handlebars = self.handlebars.lock().map_err(|e| e.to_string())?;
        handlebars.clear_templates();
        
        let template_dir = PathBuf::from("./templates/emails");
        if template_dir.exists() {
             if let Ok(entries) = std::fs::read_dir(&template_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() && path.extension().map_or(false, |e| e == "hbs") {
                        if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                            let _ = handlebars.register_template_file(name, &path);
                        }
                    }
                }
             }
             info!("Reloaded email templates");
             Ok(())
        } else {
            Err("Template directory not found".to_string())
        }
    }
}
