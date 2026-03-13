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
}
