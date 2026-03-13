use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::errors::AppError;
use crate::models::{CreateUser, User};
use crate::repositories::user_repository::UserRepository;

pub struct UserService;

impl UserService {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<User>, AppError> {
        UserRepository::find_all(pool).await
    }

    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<User, AppError> {
        UserRepository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User {} not found", id)))
    }

    pub async fn create(
        pool: &PgPool,
        name: Option<String>,
        email: Option<String>,
    ) -> Result<User, AppError> {
        let name = name
            .filter(|s| !s.trim().is_empty())
            .ok_or_else(|| AppError::BadRequest("name is required".into()))?;

        let email = email
            .filter(|s| s.contains('@'))
            .ok_or_else(|| AppError::BadRequest("email is invalid".into()))?;

        UserRepository::create(
            pool,
            &CreateUser {
                name: Some(name),
                email: Some(email),
            },
        )
        .await
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
        let deleted =  UserRepository::delete(pool, id).await?;

        if !deleted {
            return Err(AppError::NotFound(format!("User {} not found", id)));
        }

        Ok(())
    }
}
