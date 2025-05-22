use domain::entities::{brand, transport, transport_statuses, transport_type, vehicle_model};
use sea_orm::*;

use crate::dtos::prevention::transport::GetTransportDTO;

pub struct TransportService {
    pub db: DatabaseConnection,
}


impl TransportService {
    pub async fn find(&self, search: Option<String>) -> Result<Vec<GetTransportDTO>, DbErr> {
        let mut query  = transport::Entity::find()
            .select_only()
            .column_as(transport::Column::Id, "id")
            .column_as(transport::Column::Details, "details")
            .column_as(transport::Column::Plate, "plate")
            .column_as(transport::Column::Unit, "unit")
            .column_as(transport_type::Column::Name, "type")
            .column_as(brand::Column::Name, "brand")
            .column_as(vehicle_model::Column::Name, "model")
            .join(JoinType::InnerJoin, transport_statuses::Relation::Transport.def())
            .join(JoinType::InnerJoin, transport_type::Relation::Transport.def())
            .join(JoinType::InnerJoin, brand::Relation::Transport.def())
            .join(JoinType::InnerJoin, vehicle_model::Relation::Transport.def());

        if let Some(search) = search {
            query = query.filter(transport::Column::Details.contains(search));
        }

        query.into_model::<GetTransportDTO>().all(&self.db).await
    }
}
