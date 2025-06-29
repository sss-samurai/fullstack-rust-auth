

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
mod config;
mod components;
use crate::components::db::AsyncConnectionPool;
use crate::components::router::main_router::main_router;
use crate::config::cors_config::cors_config;
use std::env;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // loads .env file
    print!("{}", "conn_str".to_string());

    let db_host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let db_port = env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
    let db_user = env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string());
    let db_password = env::var("DB_PASSWORD").unwrap_or_default();
    let db_name = env::var("DB_NAME").unwrap_or_else(|_| "postgres".to_string());
    let conn_str = format!(
        "host={} port={} user={} password={} dbname={}",
        db_host, db_port, db_user, db_password, db_name
    );
    let db_pool = Arc::new(AsyncConnectionPool::new(&conn_str, 10).await);
    println!("SSS is running...");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(cors_config())
            .configure(main_router)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
