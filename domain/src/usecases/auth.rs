use crate::{entities::user, error::AuthError, repositories::user_repository::UserRepository};
use application::dtos::auth::LoginRequest;
use password_auth::verify_password;

use tokio::task;

/// Caso de uso para autenticar un usuario
pub struct LoginUseCase<R: UserRepository> {
    user_repository: R,
}

impl<R: UserRepository> LoginUseCase<R> {
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, request: LoginRequest) -> Result<Option<user::Model>, AuthError> {
        let user = R::find_by_username(request.username).await;

        task::spawn_blocking(|| {
            Ok(user.filter(|user| verify_password(request.password, &user.password_hash).is_ok()))
        })
        .await?
    }
}
