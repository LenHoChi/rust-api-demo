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
}
