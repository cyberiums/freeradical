use colored::*;
use reqwest::blocking::Client;
use std::fs;
use std::error::Error;
use serde_json::Value;

pub fn run(file: String) {
    println!("{}", format!("📥 Importing from {}", file).cyan());
    
    match import_from_file(&file) {
        Ok(count) => {
            println!("{}", format!("✅ Successfully imported {} items", count).green().bold());
        }
        Err(e) => {
            eprintln!("{}", format!("❌ Import failed: {}", e).red());
        }
    }
}

fn import_from_file(file_path: &str) -> Result<usize, Box<dyn Error>> {
    println!("{}", "📖 Reading file...".cyan());
    let content = fs::read_to_string(file_path)?;
    
    println!("{}", "🔍 Parsing JSON...".cyan());
    let data: Value = serde_json::from_str(&content)?;
    
    // Detect resource type from structure
    let items = if let Some(arr) = data.as_array() {
        arr
    } else if let Some(obj) = data.as_object() {
        // Handle wrapped response {pages: [...]}
        if let Some(Value::Array(arr)) = obj.values().next() {
            arr
        } else {
            return Err("Invalid JSON structure".into());
        }
    } else {
        return Err("Expected JSON array or object".into());
    };
    
    let api_url = "http://localhost:8000";
    let client = Client::new();
    
    println!("{}", format!("🚀 Importing {} items...", items.len()).cyan());
    
    let mut success_count = 0;
    for (i, item) in items.iter().enumerate() {
        let endpoint = detect_endpoint(item);
        let url = format!("{}{}", api_url, endpoint);
        
        match client.post(&url)
            .json(&item)
            .send() {
            Ok(response) if response.status().is_success() => {
                success_count += 1;
                print!(".");
                if (i + 1) % 50 == 0 {
                    println!(" {}/{}", i + 1, items.len());
                }
            }
            Ok(response) => {
                eprintln!("\n{}", format!("⚠️  Item {} failed: {}", i, response.status()).yellow());
            }
            Err(e) => {
                eprintln!("\n{}", format!("❌ Item {} error: {}", i, e).red());
            }
        }
    }
    
    println!();
    Ok(success_count)
}

fn detect_endpoint(item: &Value) -> &str {
    if item.get("page_title").is_some() || item.get("pageTitle").is_some() {
        "/api/pages"
    } else if item.get("module_type").is_some() {
        "/api/modules"
    } else if item.get("filename").is_some() {
        "/api/media"
    } else {
        "/api/pages" // default
    }
}
