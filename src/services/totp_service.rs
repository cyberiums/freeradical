use totp_rs::{Algorithm, TOTP, Secret};

pub struct TotpService;

impl TotpService {
    /// Generates a new random secret and returns (secret, qr_code_base64)
    pub fn generate_secret(username: &str) -> Result<(String, String), String> {
        let secret = Secret::generate_secret();
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            secret.to_bytes().expect("Failed to convert secret to bytes"),
            Some("FreeRadical CMS".to_string()),
            username.to_string(),
        ).map_err(|e| format!("Error creating TOTP: {}", e))?;
        
        let qr_code = totp.get_qr_base64().map_err(|e| format!("Error generating QR: {}", e))?;
        
        // Return encoded secret for storage, and QR code for display
        Ok((secret.to_encoded().to_string(), qr_code))
    }
    
    /// Verifies a token against a stored secret
    pub fn verify(secret_encoded: &str, token: &str) -> Result<bool, String> {
        let secret = Secret::Encoded(secret_encoded.to_string());
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            secret.to_bytes().map_err(|e| format!("Invalid secret: {}", e))?,
            None,
            "".to_string(),
        ).map_err(|e| format!("Error creating TOTP: {}", e))?;
        
        Ok(totp.check_current(token).unwrap_or(false))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_totp_flow() {
        // 1. Generate
        let (secret, qr) = TotpService::generate_secret("testuser").expect("Generate failed");
        assert!(!secret.is_empty());
        // Verify QR code was generated (format may vary)
        assert!(!qr.is_empty(), "QR code should not be empty");
        
        // 2. Verify (Generate a valid token for the secret)
        // We need to generate a token to test verify. Can use library logic.
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            Secret::Encoded(secret.clone()).to_bytes().unwrap(),
            None,
            "".to_string()
        ).unwrap();
        
        let valid_token = totp.generate_current().unwrap();
        
        // 3. Check verify
        let is_valid = TotpService::verify(&secret, &valid_token).unwrap();
        assert!(is_valid);
        
        // 4. Check invalid
        let is_invalid = TotpService::verify(&secret, "000000").unwrap();
        assert!(!is_invalid);
    }
}
