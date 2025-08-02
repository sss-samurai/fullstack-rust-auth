use crate::components::authentication::database::Database;
use crate::components::authentication::models::EmailPayload;
use crate::components::db::AsyncConnectionPool;
use crate::components::utils::user_authentication::send_mail::send_mail;
use actix_web::{HttpResponse, web};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

pub async fn get_otp(
    payload: web::Json<EmailPayloadForApi>,
    pool: web::Data<Arc<AsyncConnectionPool>>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut rng = rand::thread_rng();
    let otp: u32 = rng.gen_range(100_000..1_000_000);
    send_mail(&payload.email, otp).await.map_err(|e| {
        eprintln!("Failed to send OTP email: {}", e);
        actix_web::error::ErrorInternalServerError("Email send failed")
    })?;
    let db_data = EmailPayload {
        email: payload.email.clone(),
        otp: Some(otp.to_string()),
    };
    Database::save_otp(db_data, &pool).await.map_err(|e| {
        eprintln!("Failed to save OTP to database: {}", e);
        actix_web::error::ErrorInternalServerError("Database save failed")
    })?;
    Ok(HttpResponse::Ok().json(json!({
        "message": "OTP sent and saved successfully",
        "success": true
    })))
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct EmailPayloadForApi {
    pub email: String,
}
