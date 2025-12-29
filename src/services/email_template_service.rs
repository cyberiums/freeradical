use handlebars::Handlebars;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use log::{info, error, warn};

#[derive(Clone)]
pub struct EmailTemplateService {
    handlebars: Arc<Mutex<Handlebars<'static>>>,
}

impl EmailTemplateService {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        
        // Register helper for date formatting if needed
        // handlebars.register_helper("format_date", Box::new(format_date_helper));

        // Load templates from directory
        let template_dir = PathBuf::from("./templates/emails");
        
        if template_dir.exists() {
             info!("Loading email templates from {:?}", template_dir);
             if let Err(e) = handlebars.register_templates_directory(".hbs", &template_dir) {
                 error!("Failed to register email templates: {}", e);
             }
        } else {
            warn!("Email template directory not found: {:?}", template_dir);
        }

        Self {
            handlebars: Arc::new(Mutex::new(handlebars)),
        }
    }

    pub fn render<T>(&self, template_name: &str, data: &T) -> Result<String, String>
    where
        T: Serialize,
    {
        let handlebars = self.handlebars.lock().map_err(|e| e.to_string())?;
        
        // Try rendering with the specific template
        // Note: register_templates_directory registers files relative to the dir. 
        // e.g. "auth/welcome.hbs" -> "auth/welcome"
        
        handlebars.render(template_name, data).map_err(|e| {
            error!("Failed to render email template '{}': {}", template_name, e);
            format!("Template render error: {}", e)
        })
    }
    
    pub fn reload_templates(&self) -> Result<(), String> {
        let mut handlebars = self.handlebars.lock().map_err(|e| e.to_string())?;
        handlebars.clear_templates();
        
        let template_dir = PathBuf::from("./templates/emails");
        if template_dir.exists() {
             handlebars.register_templates_directory(".hbs", &template_dir).map_err(|e| e.to_string())?;
             info!("Reloaded email templates");
             Ok(())
        } else {
            Err("Template directory not found".to_string())
        }
    }
}
