use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::user::User;

#[derive(Deserialize)]
pub struct RegisterInput {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginInput {
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserPublic,
}

#[derive(Serialize, Deserialize)]
pub struct UserPublic {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

impl From<User> for UserPublic {
    fn from(u: User) -> Self {
        UserPublic { id: u.id, name: u.name, email: u.email }
    }
}