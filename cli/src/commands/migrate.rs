use colored::*;
use std::process::Command;

pub fn run(action: String) {
    match action.as_str() {
        "run" => run_migrations(),
        "rollback" => rollback_migration(),
        "status" => show_status(),
        _ => {
            eprintln!("{}", format!("❌ Unknown action: {}", action).red());
            println!("Valid actions: run, rollback, status");
        }
    }
}

fn run_migrations() {
    println!("{}", "🔄 Running database migrations...".cyan());
    
    match Command::new("diesel")
        .arg("migration")
        .arg("run")
        .output() {
        Ok(output) => {
            if output.status.success() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
                println!("{}", "✅ Migrations completed successfully".green().bold());
            } else {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                eprintln!("{}", "❌ Migration failed".red());
            }
        }
        Err(e) => {
            eprintln!("{}", format!("❌ Failed to execute diesel: {}", e).red());
            println!("{}", "💡 Install diesel CLI: cargo install diesel_cli".yellow());
        }
    }
}

fn rollback_migration() {
    println!("{}", "⏮️  Rolling back last migration...".cyan());
    
    match Command::new("diesel")
        .arg("migration")
        .arg("revert")
        .output() {
        Ok(output) => {
            if output.status.success() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
                println!("{}", "✅ Rollback completed successfully".green().bold());
            } else {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                eprintln!("{}", "❌ Rollback failed".red());
            }
        }
        Err(e) => {
            eprintln!("{}", format!("❌ Failed to execute diesel: {}", e).red());
        }
    }
}

fn show_status() {
    println!("{}", "📊 Migration status:".cyan());
    
    match Command::new("diesel")
        .arg("migration")
        .arg("list")
        .output() {
        Ok(output) => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        Err(e) => {
            eprintln!("{}", format!("❌ Failed to execute diesel: {}", e).red());
            println!("{}", "💡 Install diesel CLI: cargo install diesel_cli".yellow());
        }
    }
}
