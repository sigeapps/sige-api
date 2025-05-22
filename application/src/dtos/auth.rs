use serde::{Deserialize, Serialize};

use super::user::CreateUserDTO;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub next: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

impl From<RegisterRequest> for CreateUserDTO {
    fn from(dto: RegisterRequest) -> CreateUserDTO {
        CreateUserDTO {
            name: dto.username,
            password_hash: dto.password,
        }
    }
}
