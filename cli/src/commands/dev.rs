use colored::*;
use std::process::Command;

pub fn run() {
    println!("{}", "🚀 Starting development server...".green().bold());
    println!("{}", "Server will run on http://127.0.0.1:8000".cyan());
    
    match Command::new("cargo")
        .arg("run")
        .spawn() {
        Ok(mut child) => {
            println!("{}", "✅ Server started successfully".green());
            println!("{}", "Press Ctrl+C to stop".yellow());
            let _ = child.wait();
        }
        Err(e) => {
            eprintln!("{}", format!("❌ Failed to start server: {}", e).red());
        }
    }
}
