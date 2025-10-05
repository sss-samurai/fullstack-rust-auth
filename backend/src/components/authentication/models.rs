use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct EmailPayload {
    pub email: String,
    pub otp: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub purpose: String,
    pub uuid: Option<Uuid>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordClaims {
    pub password: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoginPayload {
    pub password: String,
    pub email: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoginOtp {
    pub otp: String,
}
