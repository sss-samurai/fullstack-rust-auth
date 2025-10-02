use crate::components::authentication::database::Database;
use crate::components::authentication::models::EmailPayload;
use crate::components::db::AsyncConnectionPool;
use crate::components::utils::user_authentication::generate_encrypted_token::generate_encrypted_token;
use actix_web::{HttpResponse, web};
use serde_json::json;
use std::sync::Arc;
pub async fn validate_otp(
    payload: web::Json<EmailPayload>,
    pool: web::Data<Arc<AsyncConnectionPool>>,
) -> Result<HttpResponse, actix_web::Error> {
    let db_data = EmailPayload {
        email: payload.email.clone(),
        otp: payload.otp.clone(),
    };
    Database::compare_otp(&db_data, &pool).await.map_err(|_e| {
        actix_web::error::ErrorInternalServerError(json!({
            "message": "OTP validation failed",
            "success": false
        }))
    })?;
    let secret = std::env::var("KEY").expect("KEY must be set");
    let purpose = "create_user";
    match generate_encrypted_token(&payload.email, &secret, purpose, 10,None) {
        Ok(token) => match Database::save_temp_email(db_data, &pool).await {
            Ok(_) => Ok(HttpResponse::Ok().json(json!({
                "message": "Validated successfully",
                "success": true,
                "token": token
            }))),
            Err(_) => Err(actix_web::error::ErrorInternalServerError(json!({
                "success": false,
                "message": "Validation failed"
            }))),
        },
        Err(_) => Err(actix_web::error::ErrorInternalServerError(json!({
            "success": false,
            "message": "Some error occurred. Please retry OTP validation"
        }))),
    }
}
