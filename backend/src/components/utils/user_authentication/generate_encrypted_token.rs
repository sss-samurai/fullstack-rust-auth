use aes_gcm::aead::{Aead, KeyInit, rand_core};
use aes_gcm::{Aes256Gcm, Key, Nonce}; // Or `Aes128Gcm`
use base64::{Engine as _, engine::general_purpose}; // <-- Add this
use chrono::Utc;
use hex;
use rand_core::{OsRng, RngCore};
use uuid::Uuid;

use crate::components::authentication::models::Claims;

pub fn generate_encrypted_token(
    email: &str,
    secret: &str,
    purpose: &str,
    time_in_minutes: i64,
    session_uuid: Option<Uuid>,
    user_uuid: Option<Uuid>,
) -> Result<String, Box<dyn std::error::Error>> {
    let now = Utc::now().timestamp() as usize;
    let expiration = now + (time_in_minutes * 60) as usize;

    let claims = Claims {
        sub: email.to_owned(),
        exp: expiration,
        iat: now,
        purpose: purpose.to_string(),
        session_uuid: session_uuid,
        user_uuid: user_uuid,
    };

    let serialized = serde_json::to_vec(&claims)?;
    let key_bytes = hex::decode(secret)?;

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, serialized.as_ref())
        .map_err(|e| format!("encryption error: {e}"))?;

    let mut encrypted = nonce_bytes.to_vec();
    encrypted.extend_from_slice(&ciphertext);

    Ok(general_purpose::URL_SAFE_NO_PAD.encode(&encrypted))
}
