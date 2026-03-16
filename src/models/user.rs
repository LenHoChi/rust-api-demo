use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: Option<String>,
    pub email: Option<String>,
}