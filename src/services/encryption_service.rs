use ring::aead::{Aad, BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey, UnboundKey, AES_256_GCM};
use ring::error::Unspecified;
use ring::rand::{SecureRandom, SystemRandom};
use base64::{Engine as _, engine::general_purpose};
use std::env;

const NONCE_LEN: usize = 12;

/// Non-repeating nonce sequence for AEAD
struct OneNonceSequence(Option<[u8; NONCE_LEN]>);

impl OneNonceSequence {
    fn new(nonce: [u8; NONCE_LEN]) -> Self {
        OneNonceSequence(Some(nonce))
    }
}

impl NonceSequence for OneNonceSequence {
    fn advance(&mut self) -> Result<Nonce, Unspecified> {
        self.0.take().map(|nonce| Nonce::assume_unique_for_key(nonce)).ok_or(Unspecified)
    }
}

/// Get encryption key from environment variable
fn get_encryption_key() -> Result<Vec<u8>, String> {
    let key_base64 = env::var("ENCRYPTION_KEY")
        .unwrap_or_else(|_| {
            // Default key for development - CHANGE IN PRODUCTION!
            base64::engine::general_purpose::STANDARD.encode(&[0u8; 32])
        });
    
    general_purpose::STANDARD.decode(key_base64)
        .map_err(|e| format!("Invalid ENCRYPTION_KEY: {}", e))
}

/// Encrypt data using AES-256-GCM
pub fn encrypt(plaintext: &str) -> Result<String, String> {
    let rng = SystemRandom::new();
    
    // Generate random nonce
    let mut nonce_bytes = [0u8; NONCE_LEN];
    rng.fill(&mut nonce_bytes).map_err(|_| "Failed to generate nonce")?;
    
    // Get encryption key
    let key_bytes = get_encryption_key()?;
    if key_bytes.len() != 32 {
        return Err("Encryption key must be 32 bytes (256 bits)".to_string());
    }
    
    let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes)
        .map_err(|_| "Invalid encryption key")?;
    
    let nonce_sequence = OneNonceSequence::new(nonce_bytes);
    let mut sealing_key = SealingKey::new(unbound_key, nonce_sequence);
    
    // Encrypt data
    let mut in_out = plaintext.as_bytes().to_vec();
    sealing_key.seal_in_place_append_tag(Aad::empty(), &mut in_out)
        .map_err(|_| "Encryption failed")?;
    
    // Combine nonce + ciphertext and encode as base64
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&in_out);
    
    Ok(general_purpose::STANDARD.encode(result))
}

/// Decrypt data using AES-256-GCM
pub fn decrypt(ciphertext_base64: &str) -> Result<String, String> {
    // Decode from base64
    let combined = general_purpose::STANDARD.decode(ciphertext_base64)
        .map_err(|e| format!("Invalid base64: {}", e))?;
    
    if combined.len() < NONCE_LEN {
        return Err("Ciphertext too short".to_string());
    }
    
    // Split nonce and ciphertext
    let (nonce_bytes, ciphertext) = combined.split_at(NONCE_LEN);
    let mut nonce_array = [0u8; NONCE_LEN];
    nonce_array.copy_from_slice(nonce_bytes);
    
    // Get decryption key
    let key_bytes = get_encryption_key()?;
    if key_bytes.len() != 32 {
        return Err("Encryption key must be 32 bytes (256 bits)".to_string());
    }
    
    let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes)
        .map_err(|_| "Invalid decryption key")?;
    
    let nonce_sequence = OneNonceSequence::new(nonce_array);
    let mut opening_key = OpeningKey::new(unbound_key, nonce_sequence);
    
    // Decrypt data
    let mut in_out = ciphertext.to_vec();
    let plaintext = opening_key.open_in_place(Aad::empty(), &mut in_out)
        .map_err(|_| "Decryption failed")?;
    
    String::from_utf8(plaintext.to_vec())
        .map_err(|e| format!("Invalid UTF-8: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let plaintext = "my-secret-api-key-12345";
        let encrypted = encrypt(plaintext).expect("Encryption failed");
        let decrypted = decrypt(&encrypted).expect("Decryption failed");
        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_different_encryptions() {
        let plaintext = "test-key";
        let encrypted1 = encrypt(plaintext).expect("Encryption 1 failed");
        let encrypted2 = encrypt(plaintext).expect("Encryption 2 failed");
        
        // Different nonces should produce different ciphertexts
        assert_ne!(encrypted1, encrypted2);
        
        // Both should decrypt to same plaintext
        assert_eq!(decrypt(&encrypted1).unwrap(), plaintext);
        assert_eq!(decrypt(&encrypted2).unwrap(), plaintext);
    }
}
