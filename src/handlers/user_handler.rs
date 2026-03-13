use actix_web::{web, HttpResponse};
use log::info;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::errors::AppError;
use crate::models::CreateUser;
use crate::models::User;
use crate::services::user_service::UserService;

pub async fn get_users_db(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    // let users = UserService::get_all(pool.get_ref()).await?;
    let users = match UserService::get_all(pool.get_ref()).await {
        Ok(u) => u,
        Err(e) => return Err(e),
    };
    Ok(HttpResponse::Ok().json(users))
}

pub async fn get_user_db(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let user = UserService::get_by_id(pool.get_ref(), path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn create_user_db(
    pool: web::Data<PgPool>,
    body: web::Json<CreateUser>,
) -> Result<HttpResponse, AppError> {
    let user = UserService::create(
        pool.get_ref(),
        body.name.clone(),
        body.email.clone(),
    ).await?;
    Ok(HttpResponse::Created().json(user))
}

pub async fn delete_user_db(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let user = UserService::delete(pool.get_ref(), path.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

// simple testing

pub async fn hello() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "message": "Hello from Rust API!" }))
}

pub async fn get_users() -> HttpResponse {
    info!("GET /users");
    let users = vec![
        User {
            id: uuid::Uuid::new_v4(),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
            created_at: chrono::Utc::now(),
        },
        User {
            id: uuid::Uuid::new_v4(),
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
            created_at: chrono::Utc::now(),
        },
    ];
    HttpResponse::Ok().json(users)
}

pub async fn get_user(path: web::Path<String>) -> Result<HttpResponse, AppError> {
    let id: String = path.into_inner();
    info!("GET /users/{}", id);

    // if id == 0 || id > 2 {
    //     return Err(AppError::NotFound(format!("User with id {} not found", id)));
    // }
    // let uuid = uuid::Uuid::parse_str(&id)
    //     .map_err(|_| AppError::NotFound(format!("User with id {} not found", id)))?;

    let uuid = match uuid::Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => return Err(AppError::NotFound(format!("User with id {} not found", id))),
    };

    let user = User {
        id: uuid::Uuid::new_v4(),
        name: format!("User {}", id),
        email: format!("user{}@example.com", id),
        created_at: chrono::Utc::now(),
    };
    Ok(HttpResponse::Ok().json(user))
}

pub async fn create_user(body: web::Json<CreateUser>) -> Result<HttpResponse, AppError> {
    info!("POST /users");

    // Validate input
    let name = body
        .name
        .clone()
        .filter(|n| !n.trim().is_empty())
        .ok_or_else(|| AppError::BadRequest("Field 'name' is required".to_string()))?;

    let email = body
        .email
        .clone()
        .filter(|e| e.contains('@'))
        .ok_or_else(|| AppError::BadRequest("Field 'email' is invalid or missing".to_string()))?;

    let created_at = chrono::Utc::now();

    let new_user = User {
        id: uuid::Uuid::new_v4(),
        name,
        email,
        created_at,
    };
    Ok(HttpResponse::Created().json(new_user))
}

pub async fn delete_user(path: web::Path<String>) -> Result<HttpResponse, AppError> {
    let id: String = path.into_inner();
    let uuid = match uuid::Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => return Err(AppError::NotFound(format!("User with id {} not found", id))),
    };
    
    Ok(HttpResponse::Ok().json("Success"))
}
