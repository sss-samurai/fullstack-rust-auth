// use rand::RngCore;
// use crate::components::db::AsyncConnectionPool;
// use crate::components::authentication::database::Database;
// use uuid::Uuid;
// use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse, error::ErrorUnauthorized, web};

// fn generate_session_id() -> String {
//     let mut bytes = [0u8; 32];
//     rand::thread_rng().fill_bytes(&mut bytes);
//     hex::encode(bytes)
// }

// pub async fn create_session(
//     user_id: Uuid,
//     pool: &AsyncConnectionPool,
// ) -> Result<HttpResponse, Error> {
//     let session_id = generate_session_id();
    
//     match Database::create_new_session(user_id, &session_id, pool).await {
//         Ok(_) => Ok(HttpResponse::Ok().json({
//             serde_json::json!({
//                 "message": "Session created successfully",
//                 "success": true,
//                 "session_id": session_id
//             })
//         })),
//         Err(err) => Err(err),
//     }
// }
