use domain::{
    entities::register, error::RepositoryError,
    repositories::register_repository::RegisterRepository,
};
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RegisterExitInput {
    observations: String,
}

pub struct RegisterExitUseCase<R: RegisterRepository> {
    pub register_repository: R,
}

impl<R: RegisterRepository> RegisterExitUseCase<R> {
    pub fn new(register_repository: R) -> Self {
        RegisterExitUseCase {
            register_repository,
        }
    }
    pub async fn execute(
        &self,
        register_exit: RegisterExitInput,
        register_id: i32,
    ) -> Result<(), RepositoryError> {
        let old_register = self
            .register_repository
            .find_by_id(register_id)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .ok_or_else(|| RepositoryError::NotFound("Register not founded".to_string()))?;

        let mut active_register: register::ActiveModel = old_register.into();

        active_register.observations = Set(Some(register_exit.observations));

        self.register_repository.update(active_register).await?;

        Ok(())
    }
}
