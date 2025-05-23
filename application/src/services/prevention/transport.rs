use domain::entities::{brand, transport, transport_statuses, transport_type, vehicle_model};
use sea_orm::*;

use crate::{
    connection::connect,
    dtos::prevention::transport::{CreateTransportDTO, GetTransportDTO},
};

#[derive(Debug, Clone)]
pub struct TransportService {
    pub db: DatabaseConnection,
}

impl TransportService {
    pub async fn new(db_url: &str) -> Result<Self, DbErr> {
        let db = connect(db_url).await?;

        Ok(TransportService { db })
    }

    pub async fn find(&self, search: Option<String>) -> Result<Vec<GetTransportDTO>, DbErr> {
        let mut query = transport::Entity::find()
            .left_join(transport_type::Entity)
            .left_join(brand::Entity)
            .left_join(vehicle_model::Entity)
            .left_join(transport_statuses::Entity)
            .order_by_desc(transport::Column::Id);

        if let Some(search) = search {
            query = query.filter(transport::Column::Details.contains(search));
        }

        query.into_partial_model::<GetTransportDTO>().all(&self.db).await
    }

    pub async fn create(self, transport: CreateTransportDTO) -> Result<(), DbErr> {
        transport::Entity::insert(transport.into_active_model())
            .exec(&self.db)
            .await?;

        Ok(())
    }
}
