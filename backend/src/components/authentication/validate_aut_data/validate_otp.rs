use std::sync::Arc;
use actix_web::{web, HttpResponse};
use rand::Rng;
use serde_json::json;
use crate::components::db::AsyncConnectionPool;
use crate::components::utils::user_authentication::send_mail::send_mail;
use crate::components::authentication::database::Database;
use crate::components::authentication::models::EmailPayload;

pub async fn validate_otp(
    payload: web::Json<EmailPayload>,
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
     Database::compare_otp(db_data, &pool).await.map_err(|e| {
        eprintln!("Failed to save OTP to database: {}", e);
        actix_web::error::ErrorInternalServerError("Database save failed")
    })?;
    Ok(HttpResponse::Ok().json(json!({
        "message": "OTP sent and saved successfully",
        "success": true
    })))
}
