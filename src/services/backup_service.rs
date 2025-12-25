use std::process::Command;
use chrono::Utc;
use std::path::PathBuf;

pub struct BackupService;

impl BackupService {
    /// Creates a MySQL dump backup file with timestamp
    pub fn create_backup(db_url: &str, output_dir: &str) -> Result<String, String> {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("freeradical_backup_{}.sql", timestamp);
        let output_path = PathBuf::from(output_dir).join(&filename);
        
        // Parse DB URL to extract connection params
        let (host, user, password, database) = Self::parse_db_url(db_url)?;
        
        let output = Command::new("mysqldump")
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
            .map_err(|e| format!("Failed to execute mysqldump: {}", e))?;
        
        if !output.status.success() {
            return Err(format!(
                "mysqldump failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        
        std::fs::write(&output_path, output.stdout)
            .map_err(|e| format!("Failed to write backup: {}", e))?;
        
        log::info!("Database backup created: {:?}", output_path);
        Ok(output_path.to_string_lossy().to_string())
    }
    
    fn parse_db_url(url: &str) -> Result<(String, String, String, String), String> {
        // Format: mysql://user:password@host:port/database
        let without_protocol = url.strip_prefix("mysql://")
            .ok_or("Invalid DB URL format")?;
        
        let (credentials, rest) = without_protocol.split_once('@')
            .ok_or("Missing @ separator")?;
        
        let (user, password) = credentials.split_once(':')
            .ok_or("Missing password")?;
        
        let (host_port, database) = rest.split_once('/')
            .ok_or("Missing database name")?;
        
        let host = host_port.split(':').next().unwrap_or(host_port);
        
        Ok((
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
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_db_url() {
        let url = "mysql://root:password@localhost:3306/freeradical";
        let (host, user, pass, db) = BackupService::parse_db_url(url).unwrap();
        
        assert_eq!(host, "localhost");
        assert_eq!(user, "root");
        assert_eq!(pass, "password");
        assert_eq!(db, "freeradical");
    }
}
