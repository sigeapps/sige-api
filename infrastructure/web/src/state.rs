use application::services::{prevention::register::RegisterService, user::UserService};
use database::repositories::{
    commission_repository_impl::SeaOrmCommissionRepository,
    lookup_repository_impl::SeaOrmLookupRepository,
    official_repository_impl::SeaOrmOfficialRepository,
    register_repository_impl::SeaOrmRegisterRepository,
    transport_repository_impl::SeaOrmTransportRepository,
    user_repository_impl::SeaOrmUserRepository,
};
use tracing::debug;

#[derive(Debug, Clone)]
pub struct AppState {
    pub user_service: UserService,
    pub register_service: RegisterService,
    pub lookup_repository: SeaOrmLookupRepository,
    pub transport_repository: SeaOrmTransportRepository,
    pub official_repository: SeaOrmOfficialRepository,
    pub commission_repository: SeaOrmCommissionRepository,
}

impl AppState {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        debug!("Creating AppState with database_url: {}", database_url);

        Ok(Self {
            user_service: UserService::new(database_url).await?,
            register_service: RegisterService::new(database_url).await?,
            official_repository: SeaOrmOfficialRepository::new(database_url).await?,
            lookup_repository: SeaOrmLookupRepository::new(database_url).await?,
            transport_repository: SeaOrmTransportRepository::new(database_url).await?,
            commission_repository: SeaOrmCommissionRepository::new(database_url).await?,
        })
    }
}
