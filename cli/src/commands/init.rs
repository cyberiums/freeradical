use colored::*;
use std::fs;
use std::path::Path;

const DEFAULT_TEMPLATE: &str = include_str!("../../templates/default/config.toml");
const GITIGNORE: &str = include_str!("../../templates/default/.gitignore");

pub fn run(name: String, template: String) {
    println!("{}", format!("🚀 Initializing FreeRadical CMS project: {}", name).green().bold());
    
    // Create project directory
    let project_path = Path::new(&name);
    if project_path.exists() {
        eprintln!("{}", format!("❌ Error: Directory '{}' already exists", name).red());
        std::process::exit(1);
    }
    
    fs::create_dir_all(&project_path).expect("Failed to create project directory");
    
    // Create subdirectories
    let dirs = vec!["uploads", "templates", "migrations"];
    for dir in dirs {
        fs::create_dir_all(project_path.join(dir)).expect("Failed to create directory");
    }
    
    // Write config file
    fs::write(
        project_path.join("config.toml"),
        DEFAULT_TEMPLATE
    ).expect("Failed to write config.toml");
    
    // Write .gitignore
    fs::write(
        project_path.join(".gitignore"),
        GITIGNORE
    ).expect("Failed to write .gitignore");
    
    // Write .env.example
    let env_example = r#"# Database Configuration
DATABASE_URL=mysql://user:password@localhost/freeradical

# Server Configuration
SERVER_HOST=127.0.0.1
SERVER_PORT=8000

# Security
JWT_SECRET=your-secret-key-change-this-in-production

# Redis Cache (optional)
REDIS_URL=redis://127.0.0.1:6379

# Application
APP_BASE_URL=http://localhost:8000
"#;
    fs::write(
        project_path.join(".env.example"),
        env_example
    ).expect("Failed to write .env.example");
    
    // Write README
    let readme = format!(r#"# {}

FreeRadical CMS Project

## Quick Start

1. Copy `.env.example` to `.env` and configure your database
2. Run migrations: `freeradical migrate run`
3. Start development server: `freeradical dev`

## Documentation

Visit https://github.com/yourusername/freeradical for full documentation.
"#, name);
    
    fs::write(
        project_path.join("README.md"),
        readme
    ).expect("Failed to write README.md");
    
    println!("{}", "✅ Project initialized successfully!".green().bold());
    println!("\n{}", "Next steps:".yellow().bold());
    println!("  cd {}", name);
    println!("  cp .env.example .env");
    println!("  # Configure your .env file");
    println!("  freeradical migrate run");
    println!("  freeradical dev");
}
