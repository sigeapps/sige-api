use domain::repositories::commission_repository::CommissionAggregate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateCommissionAggregateDto {
    pub commission: CreateCommissionDto,
    pub reason: CreateCommissionReasonDto,
    pub officials: Vec<CreateCommissionOfficialDto>,
    pub transports: Vec<CreateCommissionTransportDto>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateCommissionDto {
    pub brigade_id: i32,
    pub authorized_official_id: Option<i32>,
    pub status: String,
    pub observations: Option<String>,
}

impl Into<domain::entities::commission::ActiveModel> for CreateCommissionDto {
    fn into(self) -> domain::entities::commission::ActiveModel {
        use sea_orm::ActiveValue::Set;
        domain::entities::commission::ActiveModel {
            brigade_id: Set(self.brigade_id),
            authorized_official_id: Set(self.authorized_official_id),
            status: Set(self.status),
            observations: Set(self.observations),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateCommissionReasonDto {
    pub commission_id: i32,
    pub name: String,
    pub description: String,
    pub zone: Option<String>,
    pub municipality_id: i32,
}

impl Into<domain::entities::commission_reason::ActiveModel> for CreateCommissionReasonDto {
    fn into(self) -> domain::entities::commission_reason::ActiveModel {
        use sea_orm::ActiveValue::Set;
        domain::entities::commission_reason::ActiveModel {
            commission_id: Set(self.commission_id),
            name: Set(self.name),
            description: Set(self.description),
            zone: Set(self.zone),
            municipality_id: Set(self.municipality_id),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateCommissionOfficialDto {
    pub official_id: i32,
}

impl Into<domain::entities::commission_official::ActiveModel> for CreateCommissionOfficialDto {
    fn into(self) -> domain::entities::commission_official::ActiveModel {
        use sea_orm::ActiveValue::Set;
        domain::entities::commission_official::ActiveModel {
            official_id: Set(self.official_id),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateCommissionTransportDto {
    pub transport_id: i32,
}

impl Into<domain::entities::commission_transport::ActiveModel> for CreateCommissionTransportDto {
    fn into(self) -> domain::entities::commission_transport::ActiveModel {
        use sea_orm::ActiveValue::Set;
        domain::entities::commission_transport::ActiveModel {
            transport_id: Set(self.transport_id),
            ..Default::default()
        }
    }
}

impl Into<CommissionAggregate> for CreateCommissionAggregateDto {
    fn into(self) -> CommissionAggregate {
        CommissionAggregate {
            commission: self.commission.into(),
            reason: self.reason.into(),
            officials: self.officials.into_iter().map(|o| o.into()).collect(),
            transports: self.transports.into_iter().map(|t| t.into()).collect(),
        }
    }
}
