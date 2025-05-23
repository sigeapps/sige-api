use chrono::NaiveDate;
use dto::CreateCommissionDTO;
use official_dto::CreateCommissionOfficialDTO;
use reason_dto::CreateCommissionReasonDTO;
use serde::{Deserialize, Serialize};
use transport_dto::CreateCommissionTransportDTO;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommissionAggregateDTO {
    pub commission: CreateCommissionDTO,
    pub reason: CreateCommissionReasonDTO,
    pub officials: Vec<CreateCommissionOfficialDTO>,
    pub transports: Vec<CreateCommissionTransportDTO>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCommissionExitDTO {
    commission_exit_at: NaiveDate,
    commission_observations: String,
    commission_persons: String,
    commission_transports: Vec<CreateCommissionTransportDTO>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCommissionStatusDTO {
    commission_persons: String,
    commission_transports: String,
}

pub mod dto {
    use domain::entities::commission::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, DeriveIntoActiveModel, Deserialize)]
    pub struct CreateCommissionDTO {
        pub brigade_id: i32,
        pub authorized_official_id: Option<i32>,
        pub status: String,
        pub observations: Option<String>,
    }
}

pub mod reason_dto {
    use domain::entities::commission_reason::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, DeriveIntoActiveModel, Deserialize)]
    pub struct CreateCommissionReasonDTO {
        #[serde(skip_deserializing)]
        pub commission_id: i32,
        pub name: String,
        pub description: String,
        pub zone: Option<String>,
        pub municipality_id: i32,
    }
}

pub mod transport_dto {
    use domain::entities::commission_transport::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, DeriveIntoActiveModel, Deserialize)]
    pub struct CreateCommissionTransportDTO {
        pub transport_id: i32,
        #[serde(skip_deserializing)]
        pub commission_id: i32,
    }
}

pub mod official_dto {
    use domain::entities::commission_official::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, DeriveIntoActiveModel, Deserialize)]
    pub struct CreateCommissionOfficialDTO {
        pub official_id: i32,
        #[serde(skip_deserializing)]
        pub commission_id: i32,
    }
}
