use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use base64::{Engine as _, engine::general_purpose};
use chrono::Utc;
use hex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub purpose: String,
}
pub fn decrypt_encrypted_token(
    token: &str,
    secret: &str,
) -> Result<Claims, Box<dyn std::error::Error>> {
    let encrypted_data = general_purpose::URL_SAFE_NO_PAD.decode(token)?;
    if encrypted_data.len() < 12 {
        return Err("Invalid encrypted data length".into());
    }
    let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    let key_bytes = hex::decode(secret)?;
    if key_bytes.len() != 32 {
        return Err("Invalid AES key length: must be 32 bytes for AES-256-GCM".into());
    }
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let decrypted_data = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| format!("decryption failed: {e}"))?;
    let claims: Claims = serde_json::from_slice(&decrypted_data)?;
    let now = Utc::now().timestamp() as usize;
    if now > claims.exp {
        return Err("Token has expired".into());
    }

    Ok(claims)
}
