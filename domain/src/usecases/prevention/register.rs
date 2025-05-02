use crate::repositories::user_repository::UserRepository;

pub struct RegisterExitUseCase<R: UserRepository> {
    pub user_repository: R,
}

impl R for RegisterExitUseCase<R> {
    fn execute(&self) {}
}
