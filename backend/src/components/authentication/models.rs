use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct EmailPayload {
    pub email: String,
    pub otp: Option<String>,
}