use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::errors::AppError;
use crate::models::{CreateUser, User};

pub struct UserRepository;

impl UserRepository {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<User>, AppError> {
        sqlx::query_as::<_, User>(
            "SELECT * FROM users ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("DB error find_all: {}", e);
            AppError::InternalError
        })
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, AppError> {
        sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            AppError::InternalError
        })
    }

    pub async fn create(pool: &PgPool, input: &CreateUser) -> Result<User, AppError> {
        sqlx::query_as::<_, User>(
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *"
        )
        .bind(&input.name)
        .bind(&input.email)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("DB error create: {}", e);
            AppError::InternalError
        })
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| {
            log::error!("DB error delete: {}", e);
            AppError::InternalError
        })?;

        Ok(result.rows_affected() > 0)
    }
}
