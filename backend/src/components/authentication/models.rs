use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct EmailPayload {
    pub email: String,
    pub otp: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub purpose: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordClaims {
    pub password: String,
}