use actix_web::{web, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::PgPool;

use crate::{auth::create_token, errors::errors::AppError, models::*};

pub async fn register(
    pool: web::Data<PgPool>,
    body: web::Json<RegisterInput>,
) -> Result<HttpResponse, AppError> {
    let name = body.name.clone()
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| AppError::BadRequest("name is required".into()))?;

    let email = body.email.clone()
        .filter(|s| s.contains('@'))
        .ok_or_else(|| AppError::BadRequest("email is invalid".into()))?;

    let password = body.password.clone()
        .filter(|s| s.len() >= 6)
        .ok_or_else(|| AppError::BadRequest("password must be at least 6 chars".into()))?;

    // Check email
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)"
    )
    .bind(&email)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|_| AppError::InternalError)?;

    if exists {
        return Err(AppError::Conflict("Email already registered".into()));
    }

    let hashed = hash(&password, DEFAULT_COST)
        .map_err(|_| AppError::InternalError)?;

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3)
         RETURNING *"
    )
    .bind(&name)
    .bind(&email)
    .bind(&hashed)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|_| AppError::InternalError)?;

    let token = create_token(&user.id)?;

    Ok(HttpResponse::Created().json(AuthResponse {
        token,
        user: user.into(),
    }))
}

pub async fn login(
    pool: web::Data<PgPool>,
    body: web::Json<LoginInput>,
) -> Result<HttpResponse, AppError> {
    let email = body.email.clone()
        .ok_or_else(|| AppError::BadRequest("email is required".into()))?;

    let password = body.password.clone()
        .ok_or_else(|| AppError::BadRequest("password is required".into()))?;

    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&email)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|_| AppError::InternalError)?
    .ok_or_else(|| AppError::Unauthorized("Invalid email or password".into()))?;

    let valid = verify(&password, &user.password)
        .map_err(|_| AppError::InternalError)?;

    if !valid {
        return Err(AppError::Unauthorized("Invalid email or password".into()));
    }

    let token = create_token(&user.id)?;

    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user: user.into(),
    }))
}