use std::process::Command;
use chrono::Utc;
use std::path::PathBuf;

pub struct BackupService;

impl BackupService {
    /// Creates a backup file (MySQL dump or Postgres dump)
    pub fn create_backup(db_url: &str, output_dir: &str) -> Result<String, String> {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        // Parse DB URL to extract connection params
        let (protocol, host, user, password, database) = Self::parse_db_url(db_url)?;

        let filename = if protocol == "mysql" {
            format!("freeradical_backup_{}.sql", timestamp)
        } else {
            format!("freeradical_backup_{}.sql", timestamp)
        };
        let output_path = PathBuf::from(output_dir).join(&filename);
        
        let output = if protocol == "mysql" {
            Command::new("mysqldump")
                .args(&[
                    "-h", &host,
                    "-u", &user,
                    &format!("--password={}", password),
                    &database,
                    "--single-transaction",
                    "--routines",
                    "--triggers",
                ])
                .output()
                .map_err(|e| format!("Failed to execute mysqldump: {}", e))?
        } else {
            // Postgres (pg_dump)
            // PGPASSWORD env var is preferred over passing password in args
            Command::new("pg_dump")
                .env("PGPASSWORD", &password)
                .args(&[
                    "-h", &host,
                    "-U", &user,
                    "-d", &database,
                    "-F", "p", // Plain text format (SQL)
                    "--clean", // Include drop commands
                    "--if-exists",
                ])
                .output()
                .map_err(|e| format!("Failed to execute pg_dump: {}", e))?
        };
        
        if !output.status.success() {
            return Err(format!(
                "Backup tool failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        
        std::fs::write(&output_path, output.stdout)
            .map_err(|e| format!("Failed to write backup: {}", e))?;
        
        log::info!("Database backup created: {:?}", output_path);
        Ok(output_path.to_string_lossy().to_string())
    }
    
    fn parse_db_url(url: &str) -> Result<(String, String, String, String, String), String> {
        // Format: protocol://user:password@host:port/database
        let (protocol, rest) = if let Some(stripped) = url.strip_prefix("mysql://") {
            ("mysql", stripped)
        } else if let Some(stripped) = url.strip_prefix("postgres://") {
            ("postgres", stripped)
        } else if let Some(stripped) = url.strip_prefix("postgresql://") {
            ("postgres", stripped)
        } else {
            return Err("Invalid DB URL format (must be mysql:// or postgres://)".to_string());
        };
        
        let (credentials, rest) = rest.split_once('@')
            .ok_or("Missing @ separator")?;
        
        let (user, password) = credentials.split_once(':')
            .ok_or("Missing password")?;
        
        let (host_port, database) = rest.split_once('/')
            .ok_or("Missing database name")?;
        
        let host = host_port.split(':').next().unwrap_or(host_port);
        
        Ok((
            protocol.to_string(),
            host.to_string(),
            user.to_string(),
            password.to_string(),
            database.to_string(),
        ))
    }
    
    /// Compress backup file using gzip
    pub fn compress_backup(file_path: &str) -> Result<String, String> {
        let output = Command::new("gzip")
            .args(&["-f", file_path])
            .output()
            .map_err(|e| format!("Failed to compress: {}", e))?;
        
        if !output.status.success() {
            return Err(format!(
                "Compression failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        
        let compressed_path = format!("{}.gz", file_path);
        log::info!("Backup compressed: {}", compressed_path);
        Ok(compressed_path)
    }
    /// List all backup files in directory
    pub fn list_backups(backup_dir: &str) -> Result<Vec<String>, String> {
        let entries = std::fs::read_dir(backup_dir)
            .map_err(|e| format!("Failed to read backup dir: {}", e))?;
            
        let mut backups = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| format!("Error reading entry: {}", e))?;
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name() {
                    backups.push(name.to_string_lossy().to_string());
                }
            }
        }
        // Sort newest first
        backups.sort_by(|a, b| b.cmp(a));
        Ok(backups)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_db_url() {
        let url = "mysql://root:password@localhost:3306/freeradical";
        let (proto, host, user, pass, db) = BackupService::parse_db_url(url).unwrap();
        
        assert_eq!(proto, "mysql");
        assert_eq!(host, "localhost");
        assert_eq!(user, "root");
        assert_eq!(pass, "password");
        assert_eq!(db, "freeradical");
        
        let url_pg = "postgres://postgres:secret@postgres:5432/freeradical";
        let (proto, host, user, pass, db) = BackupService::parse_db_url(url_pg).unwrap();
        assert_eq!(proto, "postgres");
        assert_eq!(host, "postgres");
        assert_eq!(user, "postgres");
        assert_eq!(pass, "secret");
        assert_eq!(db, "freeradical");
    }
}
