use async_trait::async_trait;
use crate::models::config_models::LocalConfig;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use aws_sdk_s3::Client;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::config::Region;

const UPLOAD_DIR: &str = "uploads";

#[async_trait]
pub trait StorageService {
    async fn save(&self, filename: &str, data: &[u8], mime_type: &str) -> Result<String, String>;
    async fn delete(&self, filename: &str) -> Result<(), String>;
    fn get_url(&self, filename: &str) -> String;
}

#[derive(Clone)]
pub struct LocalStorage {
    base_url: String,
}

impl LocalStorage {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }
}

#[async_trait]
impl StorageService for LocalStorage {
    async fn save(&self, filename: &str, data: &[u8], _mime_type: &str) -> Result<String, String> {
        // Create upload directory if it doesn't exist
        fs::create_dir_all(UPLOAD_DIR).map_err(|e| e.to_string())?;

        let file_path = PathBuf::from(UPLOAD_DIR).join(filename);
        let storage_path = file_path.to_str().unwrap().to_string();

        let mut file = fs::File::create(&file_path).map_err(|e| e.to_string())?;
        file.write_all(data).map_err(|e| e.to_string())?;

        Ok(storage_path)
    }

    async fn delete(&self, filename: &str) -> Result<(), String> {
        let file_path = PathBuf::from(UPLOAD_DIR).join(filename);
        if file_path.exists() {
             fs::remove_file(file_path).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    fn get_url(&self, filename: &str) -> String {
        format!("{}/uploads/{}", self.base_url, filename)
    }
}

#[derive(Clone)]
pub struct S3Storage {
    client: Client,
    bucket: String,
    region: String,
    cdn_url: Option<String>,
}

impl S3Storage {
    pub async fn new(access_key: String, secret_key: String, region: String, bucket: String, cdn_url: Option<String>) -> Self {
        let region_provider = RegionProviderChain::first_try(Region::new(region.clone()));
        
        let creds = aws_sdk_s3::config::Credentials::new(
            access_key,
            secret_key,
            None,
            None,
            "static_creds",
        );

        let config = aws_config::from_env()
            .region(region_provider)
            .credentials_provider(creds)
            .load()
            .await;

        let client = Client::new(&config);

        Self {
            client,
            bucket,
            region,
            cdn_url,
        }
    }
}

#[async_trait]
impl StorageService for S3Storage {
    async fn save(&self, filename: &str, data: &[u8], mime_type: &str) -> Result<String, String> {
        let body = aws_sdk_s3::primitives::ByteStream::from(data.to_vec());

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(filename)
            .body(body)
            .content_type(mime_type)
            // .acl(aws_sdk_s3::types::ObjectCannedAcl::PublicRead) // Optional: Make public if needed, or use bucket policy
            .send()
            .await
            .map_err(|e| e.to_string())?;

        Ok(filename.to_string()) // S3 key is the storage path
    }

    async fn delete(&self, filename: &str) -> Result<(), String> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(filename)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn get_url(&self, filename: &str) -> String {
        match &self.cdn_url {
            Some(cdn) => format!("{}/{}", cdn.trim_end_matches('/'), filename),
            None => format!("https://{}.s3.{}.amazonaws.com/{}", self.bucket, self.region, filename),
        }
    }
}

#[derive(Clone)]
pub enum StorageBackend {
    Local(LocalStorage),
    S3(S3Storage),
}

impl StorageBackend {
    pub async fn from_config(config: &LocalConfig) -> Self {
        if let (Some(ak), Some(sk), Some(reg), Some(buck)) = (
            &config.aws_access_key_id,
            &config.aws_secret_access_key,
            &config.aws_region,
            &config.aws_s3_bucket,
        ) {
            log::info!("Using S3 Storage Backend");
            StorageBackend::S3(S3Storage::new(
                ak.clone(), 
                sk.clone(), 
                reg.clone(), 
                buck.clone(),
                config.cdn_url.clone()
            ).await)
        } else {
            log::info!("Using Local Storage Backend");
            StorageBackend::Local(LocalStorage::new(config.base_url.clone()))
        }
    }
}

#[async_trait]
impl StorageService for StorageBackend {
    async fn save(&self, filename: &str, data: &[u8], mime_type: &str) -> Result<String, String> {
        match self {
            StorageBackend::Local(s) => s.save(filename, data, mime_type).await,
            StorageBackend::S3(s) => s.save(filename, data, mime_type).await,
        }
    }

    async fn delete(&self, filename: &str) -> Result<(), String> {
        match self {
            StorageBackend::Local(s) => s.delete(filename).await,
            StorageBackend::S3(s) => s.delete(filename).await,
        }
    }

    fn get_url(&self, filename: &str) -> String {
        match self {
            StorageBackend::Local(s) => s.get_url(filename),
            StorageBackend::S3(s) => s.get_url(filename),
        }
    }
}
