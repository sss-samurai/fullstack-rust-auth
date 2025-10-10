use std::sync::Arc;
use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse, error::ErrorUnauthorized, web};
use serde_json::json;

use crate::components::authentication::database::Database;
use crate::components::authentication::models::Claims;
use crate::components::db::AsyncConnectionPool;
use crate::components::utils::user_authentication::generate_encrypted_token::generate_encrypted_token;

pub async fn get_new_token(
    req: HttpRequest,
    pool: web::Data<Arc<AsyncConnectionPool>>,
    // You can optionally pass the secret as app data instead of reading from env every time
    // secret: web::Data<String>
) -> Result<HttpResponse, Error> {
    let extensions = req.extensions();
    let mut conn = match pool.get().await {
        Some(c) => c,
        None => {
            return Ok(HttpResponse::ServiceUnavailable().body("No DB connection available"));
        }
    };

    let tx = match conn.client.transaction().await {
        Ok(tx) => tx,
        Err(e) => {
            return Err(actix_web::error::ErrorInternalServerError(json!({
                "message": "Failed to start transaction",
                "error": e.to_string()
            })));
        }
    };

    if let Some(claims) = extensions.get::<Claims>() {
        println!("Got claims: {:?}", claims);

        if claims.purpose != "refresh_token" {
            return Err(ErrorUnauthorized("Invalid Credentials 001"));
        }

        let (session_uuid, user_uuid) = match (claims.session_uuid, claims.user_uuid) {
            (Some(session), Some(user)) => (session, user),
            _ => {
                return Err(ErrorUnauthorized("Missing session or user UUID"));
            }
        };

        let session_id = match Database::create_new_session(user_uuid, Some(session_uuid), &tx).await {
            Ok(id) => id,
            Err(e) => {
                eprintln!("Failed to create session: {}", e);
                let _ = tx.rollback().await;
                pool.return_connection(conn).await;
                return Err(actix_web::error::ErrorInternalServerError(json!({
                    "success": false,
                    "message": "Failed to create session"
                })));
            }
        };

        let secret = std::env::var("KEY").expect("KEY must be set");
        let (access_token_result, refresh_token_result) = (
            generate_encrypted_token(
                &claims.sub,
                &secret,
                "access_token",
                15,
                Some(session_id),
                Some(user_uuid),
            ),
            generate_encrypted_token(
                &claims.sub,
                &secret,
                "refresh_token",
                2592000,
                Some(session_id),
                Some(user_uuid),
            ),
        );

        match (access_token_result, refresh_token_result) {
            (Ok(access_token), Ok(refresh_token)) => {
                if let Err(e) = tx.commit().await {
                    eprintln!("Failed to commit transaction: {}", e);
                    pool.return_connection(conn).await;
                    return Err(actix_web::error::ErrorInternalServerError(json!({
                        "success": false,
                        "message": "Failed to commit transaction"
                    })));
                }

                pool.return_connection(conn).await;

                let response = json!({
                    "access_token": access_token,
                    "refresh_token": refresh_token,
                    "expires_in": 3600,
                    "success": true
                });

                Ok(HttpResponse::Ok().json(response))
            }
            _ => {
                let _ = tx.rollback().await;
                pool.return_connection(conn).await;
                Err(actix_web::error::ErrorInternalServerError(json!({
                    "success": false,
                    "message": "Token generation failed"
                })))
            }
        }
    } else {
        Err(ErrorUnauthorized(json!({
            "success": false,
            "message": "Missing claims"
        })))
    }
}
