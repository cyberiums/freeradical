use serde::{Deserialize, Serialize};

#[derive(serde::Deserialize, Clone, Debug)]
pub struct LocalConfig {
    pub bind_address: String,
    pub bind_port: u16,
    // MySQL fields are now optional - only required if using MySQL
    pub mysql_username: Option<String>,
    pub mysql_password: Option<String>,
    pub mysql_database: Option<String>,
    pub mysql_url: Option<String>,
    pub mysql_port: Option<u16>,
    pub jwt_key: String,
    pub max_req: u16,
    pub base_url: String,
    // CDN Configuration (Optional)
    pub cdn_url: Option<String>,
    // AWS S3 Configuration (Optional)
    pub aws_access_key_id: Option<String>,
    pub aws_secret_access_key: Option<String>,
    pub aws_region: Option<String>,
    pub aws_s3_bucket: Option<String>,
    // Read Replica Database URL (Optional)
    pub database_url_read: Option<String>,
    // Redis Configuration
    pub redis_cluster_nodes: Option<String>,
}
