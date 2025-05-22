use application::services::{
    prevention::{
        commission::CommissionService, lookup::LookupService, official::OfficialService,
        register::RegisterService, transport::TransportService,
    },
    user::UserService,
};
use tracing::debug;

#[derive(Debug, Clone)]
pub struct AppState {
    pub user_service: UserService,
    pub register_service: RegisterService,
    pub lookup_service: LookupService,
    pub transport_service: TransportService,
    pub official_service: OfficialService,
    pub commission_service: CommissionService,
}

impl AppState {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        debug!("Creating AppState with database_url: {}", database_url);

        Ok(Self {
            user_service: UserService::new(database_url).await?,
            register_service: RegisterService::new(database_url).await?,
            official_service: OfficialService::new(database_url).await?,
            lookup_service: LookupService::new(database_url).await?,
            transport_service: TransportService::new(database_url).await?,
            commission_service: CommissionService::new(database_url).await?,
        })
    }
}
