use crate::components::authentication::database::Database;
use crate::components::authentication::models::EmailPayload;
use crate::components::db::AsyncConnectionPool;
use crate::components::utils::user_authentication::{
    generate_password_token::generate_password_token, get_real_ip::get_real_ip,
};
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
    Database::compare_otp(db_data, &pool).await.map_err(|e| {
        println!("{:?}", e);
        actix_web::error::ErrorInternalServerError(json!({
            "message": "OTP validation failed",
            "success": false
        }))
    })?;
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let purpose = "create_password";
    match generate_password_token(&payload.email, &secret, purpose) {
        Ok(token) => Ok(HttpResponse::Ok().json(json!({
            "message": "OTP validated successfully",
            "success": true,
            "token": token
        }))),
        Err(e) => {
            eprintln!("Failed to generate token: {}", e);
            Err(actix_web::error::ErrorInternalServerError(json!({
                "message": "Some Error Occured Plz Rtry otp validation",
                "success": false,
            })))
        }
    }
}
