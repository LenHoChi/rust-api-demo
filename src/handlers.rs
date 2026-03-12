use actix_web::{web, HttpResponse};
use log::info;

use crate::errors::AppError;
use crate::models::{CreateUser, User};

pub async fn hello() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "message": "Hello from Rust API!" }))
}

pub async fn get_users() -> HttpResponse {
    info!("GET /users");
    let users = vec![
        User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() },
        User { id: 2, name: "Bob".to_string(),   email: "bob@example.com".to_string() },
    ];
    HttpResponse::Ok().json(users)
}

pub async fn get_user(path: web::Path<u32>) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    info!("GET /users/{}", id);

    // Simulate: chỉ có id 1 và 2
    if id == 0 || id > 2 {
        return Err(AppError::NotFound(format!("User with id {} not found", id)));
    }

    let user = User {
        id,
        name: format!("User {}", id),
        email: format!("user{}@example.com", id),
    };
    Ok(HttpResponse::Ok().json(user))
}

pub async fn create_user(body: web::Json<CreateUser>) -> Result<HttpResponse, AppError> {
    info!("POST /users");

    // Validate input
    let name = body.name.clone()
        .filter(|n| !n.trim().is_empty())
        .ok_or_else(|| AppError::BadRequest("Field 'name' is required".to_string()))?;

    let email = body.email.clone()
        .filter(|e| e.contains('@'))
        .ok_or_else(|| AppError::BadRequest("Field 'email' is invalid or missing".to_string()))?;

    let new_user = User { id: 42, name, email };
    Ok(HttpResponse::Created().json(new_user))
}