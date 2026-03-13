mod db;
mod errors;
mod models;
mod handlers;
mod services;
mod repositories;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

use handlers::user_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let pool = db::db::create_pool().await;

    sqlx::raw_sql(include_str!("../db/migration/001_init.sql"))
        .execute(&pool)
        .await
        .expect("Migration failed");

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("{}:{}", host, port);

    log::info!("Server running at http://{}", addr);

    let pool = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .route("/", web::get().to(user_handler::hello))
            .route("/users",      web::get().to(user_handler::get_users_db))
            .route("/users/{id}",      web::get().to(user_handler::get_user_db))
            .route("/users",      web::post().to(user_handler::create_user_db))
            .route("/users/{id}",      web::delete().to(user_handler::delete_user_db))
            // local simple testing
            .route("/users_local",      web::get().to(user_handler::get_users))
            .route("/users_local/{id}", web::get().to(user_handler::get_user))
            .route("/users_local",      web::post().to(user_handler::create_user))
            .route("/users_local/{id}", web::delete().to(user_handler::delete_user))
    })
    .bind(&addr)?
    .run()
    .await
}
