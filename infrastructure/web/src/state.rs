use std::sync::Arc;

use application::{
    connection::connect,
    services::{
        personal::persona::PersonaService,
        prevention::{
            commission::CommissionService, lookup::LookupService, official::OfficialService,
            part::PartService, register::RegisterService, seclusion::SeclusionService,
            transport::TransportService,
        },
        user::UserService,
    },
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
    pub seclusion_service: SeclusionService,
    pub part_service: PartService,
    pub persona_service: PersonaService,
}

impl AppState {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        debug!("Creating AppState with database_url: {}", database_url);

        // Create a single database connection and wrap it in Arc
        let db = Arc::new(connect(database_url).await?);

        Ok(Self {
            user_service: UserService::new(db.clone()),
            register_service: RegisterService::new(db.clone()),
            official_service: OfficialService::new(db.clone()),
            lookup_service: LookupService::new(db.clone()),
            transport_service: TransportService::new(db.clone()),
            commission_service: CommissionService::new(db.clone()),
            seclusion_service: SeclusionService::new(db.clone()),
            part_service: PartService::new(db.clone()),
            persona_service: PersonaService::new(db),
        })
    }
}
