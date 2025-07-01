use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,              
    exp: usize,              
    purpose: String,         
}

pub fn generate_password_token(email: &str, secret: &str, purpose: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(5))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: email.to_owned(),
        exp: expiration,
        purpose: purpose.to_string(),
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
}
