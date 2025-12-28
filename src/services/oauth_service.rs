use serde::{Deserialize, Serialize};

/// OAuth service for enterprise authentication
pub struct OAuthService;

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthProvider {
    pub name: String,
    pub client_id: String,
    pub authorize_url: String,
    pub token_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<i64>,
    pub refresh_token: Option<String>,
    pub id_token: Option<String>,
}

impl OAuthService {
    /// Get OAuth authorization URL
    pub fn get_authorization_url(provider: &str, redirect_uri: &str, state: &str) -> Result<String, String> {
        let provider_config = match provider {
            "google" => OAuthProvider {
                name: "Google".to_string(),
                client_id: std::env::var("GOOGLE_CLIENT_ID").unwrap_or_default(),
                authorize_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
                token_url: "https://oauth2.googleapis.com/token".to_string(),
            },
            "github" => OAuthProvider {
                name: "GitHub".to_string(),
                client_id: std::env::var("GITHUB_CLIENT_ID").unwrap_or_default(),
                authorize_url: "https://github.com/login/oauth/authorize".to_string(),
                token_url: "https://github.com/login/oauth/access_token".to_string(),
            },
            _ => return Err(format!("Unknown OAuth provider: {}", provider)),
        };
        
        let url = format!(
            "{}?client_id={}&redirect_uri={}&state={}&response_type=code&scope=openid%20email%20profile",
            provider_config.authorize_url,
            provider_config.client_id,
            urlencoding::encode(redirect_uri),
            state
        );
        
        Ok(url)
    }
    
    /// Exchange authorization code for access token
    pub async fn exchange_code_for_token(
        provider: &str,
        code: &str,
        redirect_uri: &str,
    ) -> Result<OAuthToken, String> {
        let client = reqwest::Client::new();
        
        match provider {
            "google" => {
                let params = [
                    ("code", code),
                    ("client_id", &std::env::var("GOOGLE_CLIENT_ID").unwrap_or_default()),
                    ("client_secret", &std::env::var("GOOGLE_CLIENT_SECRET").unwrap_or_default()),
                    ("redirect_uri", redirect_uri),
                    ("grant_type", "authorization_code"),
                ];

                let res = client.post("https://oauth2.googleapis.com/token")
                    .form(&params)
                    .send()
                    .await
                    .map_err(|e| format!("Request failed: {}", e))?;

                if !res.status().is_success() {
                    return Err(format!("Google token exchange failed: {}", res.status()));
                }

                res.json::<OAuthToken>().await.map_err(|e| format!("Failed to parse token: {}", e))
            },
            "github" => {
                let params = [
                    ("code", code),
                    ("client_id", &std::env::var("GITHUB_CLIENT_ID").unwrap_or_default()),
                    ("client_secret", &std::env::var("GITHUB_CLIENT_SECRET").unwrap_or_default()),
                    ("redirect_uri", redirect_uri),
                ];

                let res = client.post("https://github.com/login/oauth/access_token")
                    .header("Accept", "application/json")
                    .form(&params)
                    .send()
                    .await
                    .map_err(|e| format!("Request failed: {}", e))?;

                if !res.status().is_success() {
                    return Err(format!("GitHub token exchange failed: {}", res.status()));
                }

                res.json::<OAuthToken>().await.map_err(|e| format!("Failed to parse token: {}", e))
            },
            _ => Err(format!("Unsupported OAuth provider: {}", provider))
        }
    }

    /// Fetch Google user profile
    pub async fn fetch_google_profile(&self, access_token: &str) -> Result<crate::controllers::oauth_callback_controller::UserProfile, String> {
        let client = reqwest::Client::new();
        let res = client.get("https://www.googleapis.com/oauth2/v2/userinfo")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !res.status().is_success() {
            return Err(format!("Failed to fetch Google profile: {}", res.status()));
        }

        let json: serde_json::Value = res.json().await.map_err(|e| format!("Failed to parse profile: {}", e))?;
        
        Ok(crate::controllers::oauth_callback_controller::UserProfile {
            provider: "google".to_string(),
            provider_user_id: json["id"].as_str().unwrap_or_default().to_string(),
            email: json["email"].as_str().unwrap_or_default().to_string(),
            name: json["name"].as_str().unwrap_or_default().to_string(),
        })
    }

    /// Fetch GitHub user profile
    pub async fn fetch_github_profile(&self, access_token: &str) -> Result<crate::controllers::oauth_callback_controller::UserProfile, String> {
        let client = reqwest::Client::new();
        let res = client.get("https://api.github.com/user")
            .header("Authorization", format!("Bearer {}", access_token))
            .header("User-Agent", "Oxidly-Cloud-Platform")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !res.status().is_success() {
            return Err(format!("Failed to fetch GitHub profile: {}", res.status()));
        }

        let json: serde_json::Value = res.json().await.map_err(|e| format!("Failed to parse profile: {}", e))?;
        
        // GitHub email might be private, need separate call technically, but for now try field
        let email = json["email"].as_str().unwrap_or("").to_string();
        
        Ok(crate::controllers::oauth_callback_controller::UserProfile {
            provider: "github".to_string(),
            provider_user_id: json["id"].as_i64().map(|id| id.to_string()).unwrap_or_default(),
            email, 
            name: json["name"].as_str().unwrap_or(json["login"].as_str().unwrap_or("GitHub User")).to_string(),
        })
    }

    // Helper to store connection (placeholder for now, logic moving to controller)
    pub fn store_connection(
        &self,
        _conn: &crate::models::PooledDatabaseConnection,
        _provider_user_id: &str,
        _provider: &str,
        _access_token: &str,
        _refresh_token: Option<&str>,
        _expires_in: Option<i64>,
    ) -> Result<(), String> {
        // Logic moved to controller to handle user creation first
        Ok(())
    }
}
