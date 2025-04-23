use application::dtos::auth::LoginRequest;

use crate::repositories::user_repository::UserRepository;

pub struct LoginUseCase<R: UserRepository> {
    user_repository: R,
}

impl<R: UserRepository> LoginUseCase<R> {
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, request: LoginRequest) {
        // AI, there is an error!
        let user = self
            .user_repository
            .find_by_username(&request.username)
            .await;
    }
}
