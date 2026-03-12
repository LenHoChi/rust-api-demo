// mod errors;
// mod handlers;
// mod models; 
// -> if not exist lib.rs file ()

use demo_rest_api::handlers;
// use demo_rest_api::errors;
// use demo_rest_api::models;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("{}:{}", host, port);

    log::info!("Server running at http://{}", addr);

    HttpServer::new(|| {
        App::new()
            .route("/",              web::get().to(handlers::hello))
            .route("/users",         web::get().to(handlers::get_users))
            .route("/users/{id}",    web::get().to(handlers::get_user))
            .route("/users",         web::post().to(handlers::create_user))
    })
    .bind(&addr)?
    .run()
    .await
}