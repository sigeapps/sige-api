use dto::{CreateCommissionDTO, UpdateCommissionDTO, UpdateCommissionStatusDTO};
use official_dto::CreateCommissionOfficialDTO;
use reason_dto::CreateCommissionReasonDTO;
use sea_orm::{prelude::ChronoDateTimeWithTimeZone, FromQueryResult};
use seclusion_dto::{CreateTemporalSeclusionDTO, UpdateTemporalSeclusionDTO};
use seized_transport_dto::UpdateSeizedTransportDTO;
use serde::{Deserialize, Serialize};
use transport_dto::CreateCommissionTransportDTO;

use super::transport::CreateTransportDTO;

#[derive(Debug, Clone, Deserialize, Serialize, FromQueryResult)]
pub struct GetCommissionSummaryDTO {
    id: i32,
    entry_at: Option<ChronoDateTimeWithTimeZone>,
    exit_at: Option<ChronoDateTimeWithTimeZone>,
    status_at: Option<ChronoDateTimeWithTimeZone>,
    brigade: String,
    zone: String,
    reason: String,
    boss: String,
    boss_hierarchy: String,
    officials_count: i32,
    auth_official: String,
    auth_official_hierarchy: String,
    observations: Option<String>,
}

/// This dto creates only relationships of vecs
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommissionAggregateDTO {
    pub commission: CreateCommissionDTO,
    pub reason: CreateCommissionReasonDTO,
    pub officials: Vec<CreateCommissionOfficialDTO>,
    pub transports: Vec<CreateCommissionTransportDTO>,
}

/// This dto creates new tranports and relations
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCommissionExitDTO {
    #[serde(flatten)]
    pub commission: UpdateCommissionDTO,
    pub seclusions: Vec<CreateTemporalSeclusionDTO>,
    pub transports: Vec<CreateTransportDTO>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCommissionStatusAggregateDTO {
    #[serde(flatten)]
    pub commission: UpdateCommissionStatusDTO,
    pub seclusions: Vec<UpdateTemporalSeclusionDTO>,
    pub transports: Vec<UpdateSeizedTransportDTO>,
}

pub mod dto {
    use domain::entities::commission::ActiveModel;
    use sea_orm::{prelude::ChronoDateTimeWithTimeZone, DeriveIntoActiveModel};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, DeriveIntoActiveModel, Deserialize, Clone)]
    pub struct CreateCommissionDTO {
        pub brigade_id: i32,
        #[serde(skip_deserializing)]
        pub boss_id: i32,
        pub authorized_official_id: Option<i32>,
        pub observations: Option<String>,
    }

    #[derive(Debug, Serialize, DeriveIntoActiveModel, Deserialize)]
    pub struct UpdateCommissionDTO {
        #[serde(skip_deserializing)]
        pub id: i32,
        pub observations: String,
        pub exit_at: ChronoDateTimeWithTimeZone,
    }

    #[derive(Debug, Serialize, DeriveIntoActiveModel, Deserialize)]
    pub struct UpdateCommissionStatusDTO {
        #[serde(skip_deserializing)]
        pub id: i32,
        pub status_at: ChronoDateTimeWithTimeZone,
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

pub mod seized_transport_dto {
    use domain::entities::transport::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, DeriveIntoActiveModel, Deserialize, Serialize)]
    pub struct UpdateSeizedTransportDTO {
        #[serde(skip_deserializing)]
        pub id: i32,
        pub status_id: i32,
    }
}

pub mod official_dto {
    use domain::entities::commission_official::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, DeriveIntoActiveModel, Deserialize, Clone, Copy)]
    pub struct CreateCommissionOfficialDTO {
        pub official_id: i32,
        #[serde(skip_deserializing)]
        pub commission_id: i32,
    }
}

pub mod seclusion_dto {
    use domain::entities::temporal_seclusion::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, DeriveIntoActiveModel, Deserialize)]
    pub struct CreateTemporalSeclusionDTO {
        #[serde(skip_deserializing)]
        pub commission_id: i32,
        pub ci: String,
        pub last_name: String,
        pub first_name: String,
        pub status_id: i32,
    }

    #[derive(Debug, Serialize, DeriveIntoActiveModel, Deserialize)]
    pub struct UpdateTemporalSeclusionDTO {
        #[serde(skip_deserializing)]
        pub commission_id: i32,
        pub status_id: i32,
    }
}
