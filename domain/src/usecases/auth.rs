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
        let user = R::find_by_username(request.username.clone()) // Use associated function call and clone username
            .await;
        // TODO: Handle the user result (e.g., check password, generate token)
    }
}
