pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub struct LoginResponse {
    pub token: String,
}
