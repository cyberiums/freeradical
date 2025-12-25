use std::path::Path;
use std::sync::{Arc, Mutex};
use handlebars::Handlebars;
use liquid::{Parser, ParserBuilder, Template};
use std::collections::HashMap;

pub enum TemplateEngine {
    Handlebars,
    Liquid,
}

pub struct TemplateService {
    handlebars: Arc<Mutex<Handlebars<'static>>>,
    liquid_templates: Arc<Mutex<HashMap<String, liquid::Template>>>,
    liquid_parser: Parser,
}

impl TemplateService {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(false); // Legacy support
        
        let liquid_parser = ParserBuilder::with_stdlib()
            .build()
            .expect("Failed to build liquid parser");

        Self {
            handlebars: Arc::new(Mutex::new(handlebars)),
            liquid_templates: Arc::new(Mutex::new(HashMap::new())),
            liquid_parser,
        }
    }

    pub fn register_helpers(&self) {
        // Handlebars helpers would be registered here
        // Currently done in helpers::default::register_helpers
    }

    pub fn load_templates(&self, dir: &str) -> Result<(), String> {
        // Load Handlebars
        {
            let mut hb = self.handlebars.lock().unwrap();
            hb.register_templates_directory(".hbs", dir)
                .map_err(|e| e.to_string())?;
        }

        // Load Liquid (Manual Directory Scan would go here, skipping for now)
        // Ideally we walk the directory and compile .liquid files
        Ok(())
    }

    pub fn render(&self, template_name: &str, data: &serde_json::Value) -> Result<String, String> {
        // Check extension
        if template_name.ends_with(".liquid") {
             let templates = self.liquid_templates.lock().unwrap();
             if let Some(template) = templates.get(template_name) {
                 // Convert serde_json::Value to liquid::Object
                 let globals = liquid::to_object(data)
                    .map_err(|e| format!("Liquid data error: {}", e))?;
                 
                 return template.render(&globals).map_err(|e| e.to_string());
             }
             Err(format!("Liquid template not found: {}", template_name))
        } else {
            // Default to Handlebars
            let hb = self.handlebars.lock().unwrap();
            hb.render(template_name, data).map_err(|e| e.to_string())
        }
    }
    
    // Helper to register liquid template (for testing/loading)
    pub fn register_liquid_template(&self, name: &str, content: &str) -> Result<(), String> {
        let template = self.liquid_parser.parse(content)
            .map_err(|e| e.to_string())?;
            
        let mut templates = self.liquid_templates.lock().unwrap();
        templates.insert(name.to_string(), template);
        Ok(())
    }

    // Getter for legacy main.rs code if needed
    pub fn get_handlebars(&self) -> Arc<Mutex<Handlebars<'static>>> {
        self.handlebars.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_handlebars_rendering() {
        let service = TemplateService::new();
        let hb_ref = service.get_handlebars();
        hb_ref.lock().unwrap()
            .register_template_string("t1", "Hello {{name}}")
            .unwrap();
            
        let res = service.render("t1", &json!({"name": "World"})).unwrap();
        assert_eq!(res, "Hello World");
    }

    #[test]
    fn test_liquid_rendering() {
        let service = TemplateService::new();
        service.register_liquid_template("t1.liquid", "Hello {{name}}").unwrap();
        
        let res = service.render("t1.liquid", &json!({"name": "Liquid"})).unwrap();
        assert_eq!(res, "Hello Liquid");
    }
}
