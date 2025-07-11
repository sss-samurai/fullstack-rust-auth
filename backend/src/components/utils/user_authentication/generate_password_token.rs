use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,      
    exp: usize,       
    iat: usize,       
    purpose: String,  
}

pub fn generate_password_token(
    email: &str,
    secret: &str,
    purpose: &str,
    time_in_minutes: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now().timestamp() as usize;
    let expiration = now + (time_in_minutes * 60) as usize;

    let claims = Claims {
        sub: email.to_owned(),
        exp: expiration,
        iat: now,
        purpose: purpose.to_string(),
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
}
