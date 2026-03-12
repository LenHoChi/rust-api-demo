use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    // #[serder(skip_serializing)]
    // pub password: String,
    // pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: Option<String>,
    pub email: Option<String>,
}

// #[derive(Deserialize)]
// pub struct RegisterInput {
//     pub name: Option<String>,
//     pub email: Option<String>,
//     pub password: Option<String>,
// }

// #[derive(Deserialize)]
// pub struct LoginInput {
//     pub email: Option<String>,
//     pub password: Option<String>,
// }

// #[derive(Deserialize)]
// pub struct AuthResponse {
//     pub token: String,
//     pub user: UserPublic,
// }

// #[derive(Deserialize)]
// pub struct UserPublic {
//     pub id: Uuid,
//     pub name: String,
//     pub email: String,
// }

