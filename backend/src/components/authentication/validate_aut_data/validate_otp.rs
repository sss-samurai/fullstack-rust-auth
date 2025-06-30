use crate::components::authentication::database::Database;
use crate::components::authentication::models::EmailPayload;
use crate::components::db::AsyncConnectionPool;
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
    Ok(HttpResponse::Ok().json(json!({
        "message": "OTP validated successfully",
        "success": true
    })))
}
