use database::repositories::{
    register_repository_impl::SeaOrmRegisterRepository, user_repository_impl::SeaOrmUserRepository,
};
use tracing::debug;

#[derive(Debug, Clone)]
pub struct AppState {
    pub user_repository: SeaOrmUserRepository,
    pub register_repository: SeaOrmRegisterRepository,
}

impl AppState {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        debug!("Creating AppState with database_url: {}", database_url);

        Ok(Self {
            user_repository: SeaOrmUserRepository::new(database_url).await?,
            register_repository: SeaOrmRegisterRepository::new(database_url).await?,
        })
    }
}
