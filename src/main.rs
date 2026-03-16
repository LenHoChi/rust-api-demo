use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use demo_api::{
    db,
    handlers::{auth_handler, user_handler},
    auth::middleware::jwt_validator,
};
use dotenv::dotenv;
use std::env;

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
        /*
        jwt = header.payload.signature
        "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJjYjNmMDMxMi1jZjY5LTQ2OGMtYmMzNC1lODIwNDQ5YzViMmYiLCJleHAiOjE3NzM3MzY3Mzl9._motywePaikqx4V_CoXxWzqtDuaFeRtnNCnCevTbC8s",
        ->
        eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9
        .
        eyJzdWIiOiJjYjNmMDMxMi1jZjY5LTQ2OGMtYmMzNC1lODIwNDQ5YzViMmYiLCJleHAiOjE3NzM3MzY3Mzl9
        .
        _motywePaikqx4V_CoXxWzqtDuaFeRtnNCnCevTbC8s
        P1 header
        {
        "typ": "JWT",
        "alg": "HS256"
        }
        P2 payload
        {
        "sub": "cb3f0312-cf69-468c-bc34-e820449c5b2f", // user_id
        "exp": 1773736739 // time exp
        }
        P3 signature is created by JWT_SECRET, to server check the token is edit or not
         */
        let auth = HttpAuthentication::bearer(jwt_validator);
        App::new()
            .app_data(pool.clone())
             // Public routes
            .route("/auth/register", web::post().to(auth_handler::register))
            .route("/auth/login",    web::post().to(auth_handler::login))
            // Protected routes — need JWT
            .service(
                web::scope("/api")
                    .wrap(auth)
                    .route("/users/me", web::get().to(user_handler::get_me))
            )
            .route("/", web::get().to(user_handler::hello))
            // database testing
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
