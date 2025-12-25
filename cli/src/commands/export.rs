use colored::*;
use reqwest::blocking::Client;
use std::fs;
use std::error::Error;

pub fn run(resource: String, output: String) {
    println!("{}", format!("📤 Exporting {} to {}", resource, output).cyan());
    
    // Try to read config for API URL
    let api_url = "http://localhost:8000";
    let endpoint = match resource.as_str() {
        "pages" => "/api/pages",
        "modules" => "/api/modules", 
        "media" => "/api/media",
        _ => {
            eprintln!("{}", format!("❌ Unknown resource: {}", resource).red());
            println!("Valid resources: pages, modules, media");
            return;
        }
    };
    
    match export_resource(api_url, endpoint, &output) {
        Ok(_) => {
            println!("{}", format!("✅ Exported {} to {}", resource, output).green().bold());
        }
        Err(e) => {
            eprintln!("{}", format!("❌ Export failed: {}", e).red());
            println!("{}", "💡 Make sure the FreeRadical server is running on http://localhost:8000".yellow());
        }
    }
}

fn export_resource(api_url: &str, endpoint: &str, output_file: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}{}", api_url, endpoint);
    
    println!("{}", format!("🌐 Fetching from {}...", url).cyan());
    
    let response = client.get(&url)
        .send()?;
    
    if !response.status().is_success() {
        return Err(format!("API returned status: {}", response.status()).into());
    }
    
    let data = response.text()?;
    
    println!("{}", "💾 Writing to file...".cyan());
    fs::write(output_file, data)?;
    
    Ok(())
}
