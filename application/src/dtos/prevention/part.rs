use development_dto::{CreatePartDevelopmentDTO, GetPartDevelopmentDTO, UpdatePartDevelopmentDTO};
use dto::{CreatePartDTO, GetPartDTO, UpdatePartDTO};
use official_dto::{CreatePartOfficialDTO, GetPartOfficialDTO};
use responsability_dto::{
    CreatePartResponsabilityDTO, GetPartResponsabilityDTO, UpdatePartResponsabilityDTO,
};
use sea_orm::{prelude::ChronoDate, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct GetPartSummaryDTO {
    pub id: i32,
    pub date: ChronoDate,
    pub officials_count: i64,
    pub developments_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPartAggregateDTO {
    #[serde(flatten)]
    pub part: GetPartDTO,
    pub officials: Vec<GetPartOfficialDTO>,
    pub responsabilities: GetPartResponsabilityDTO,
    pub developments: Vec<GetPartDevelopmentDTO>,
}

/// This dto creates relationships and basic part data
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePartAggregateDTO {
    pub part: CreatePartDTO,
    pub officials: Vec<CreatePartOfficialDTO>,
    pub developments: Vec<CreatePartDevelopmentDTO>,
    pub responsability: CreatePartResponsabilityDTO,
}

/// This dto updates part with new developments
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePartWithDevelopmentDTO {
    #[serde(flatten)]
    pub part: UpdatePartDTO,
    pub developments: Vec<CreatePartDevelopmentDTO>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePartCompleteDTO {
    #[serde(flatten)]
    pub part: UpdatePartDTO,
    pub responsabilities: UpdatePartResponsabilityDTO,
    pub developments: Vec<UpdatePartDevelopmentDTO>,
}

pub mod dto {
    use domain::entities::part::ActiveModel;
    use sea_orm::{
        prelude::{ChronoDate, DateTime},
        DeriveIntoActiveModel, DerivePartialModel,
    };
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, DerivePartialModel)]
    #[sea_orm(entity = "domain::entities::part::Entity", from_query_result)]
    pub struct GetPartDTO {
        pub id: i32,
        pub date: ChronoDate,
        pub created_at: DateTime,
    }

    #[derive(Debug, Serialize, DeriveIntoActiveModel, Deserialize, Clone)]
    pub struct CreatePartDTO {
        #[serde(skip_deserializing)]
        pub date: ChronoDate,
    }

    #[derive(Debug, Serialize, DeriveIntoActiveModel, Deserialize)]
    pub struct UpdatePartDTO {
        #[serde(skip_deserializing)]
        pub id: i32,
        pub date: ChronoDate,
    }
}

pub mod official_dto {
    use domain::entities::part_official::ActiveModel;
    use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
    use serde::{Deserialize, Serialize};

    use crate::dtos::prevention::official::GetOfficialDTO;

    #[derive(Debug, Serialize, DeriveIntoActiveModel, Deserialize, Clone)]
    pub struct CreatePartOfficialDTO {  
        pub official_id: i32,
        #[serde(skip_deserializing)]
        pub part_id: i32,
        pub r#type: String,
    }

    #[derive(Debug, Serialize, Deserialize, DerivePartialModel)]
    #[sea_orm(entity = "domain::entities::part_official::Entity", from_query_result)]
    pub struct GetPartOfficialDTO {
        pub id: i32,
        #[sea_orm(nested)]
        pub official: GetOfficialDTO,
        pub r#type: String,
    }
}

pub mod responsability_dto {
    use domain::entities::part_responsability::ActiveModel;
    use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
    use serde::{Deserialize, Serialize};

    use crate::dtos::prevention::official::GetOfficialDTO;

    #[derive(Debug, Serialize, DeriveIntoActiveModel, Deserialize)]
    pub struct CreatePartResponsabilityDTO {
        #[serde(skip_deserializing)]
        pub part_id: i32,
        pub official_service_ci: String,
        pub official_receive_ci: String,
        pub official_block_ci: String,
        pub official_writer_ci: String,
        pub official_deliver_ci: String,
        pub official_receive_custody_ci: String,
        pub official_hr_ci: String,
        pub official_supervisor_ci: String,
        pub official_director_ci: String,
        pub official_dae_ci: String,
        pub official_die_ci: String,
    }

    #[derive(Debug, Serialize, Deserialize, DerivePartialModel)]
    #[sea_orm(
        entity = "domain::entities::part_responsability::Entity",
        from_query_result
    )]
    pub struct GetPartResponsabilityDTO {
        pub id: i32,
        #[serde(skip_deserializing)]
        pub part_id: i32,
        #[sea_orm(nested, alias = "official_service_ci")]
        pub official_service_ci: GetOfficialDTO,
        #[sea_orm(nested, alias = "official_receive_ci")]
        pub official_receive_ci: GetOfficialDTO,
        #[sea_orm(nested, alias = "official_block_ci")]
        pub official_block_ci: GetOfficialDTO,
        #[sea_orm(nested, alias = "official_writer_ci")]
        pub official_writer_ci: GetOfficialDTO,
        #[sea_orm(nested, alias = "official_deliver_ci")]
        pub official_deliver_ci: GetOfficialDTO,
        #[sea_orm(nested, alias = "official_receive_custody_ci")]
        pub official_receive_custody_ci: GetOfficialDTO,
        #[sea_orm(nested, alias = "official_hr_ci")]
        pub official_hr_ci: GetOfficialDTO,
        #[sea_orm(nested, alias = "official_supervisor_ci")]
        pub official_supervisor_ci: GetOfficialDTO,
        #[sea_orm(nested, alias = "official_director_ci")]
        pub official_director_ci: GetOfficialDTO,
        #[sea_orm(nested, alias = "official_dae_ci")]
        pub official_dae_ci: GetOfficialDTO,
        #[sea_orm(nested, alias = "official_die_ci")]
        pub official_die_ci: GetOfficialDTO,
    }

    #[derive(Debug, Serialize, DeriveIntoActiveModel, Deserialize)]
    pub struct UpdatePartResponsabilityDTO {
        pub id: i32,
        #[serde(skip_deserializing)]
        pub part_id: i32,
        pub official_service_ci: String,
        pub official_receive_ci: String,
        pub official_block_ci: String,
        pub official_writer_ci: String,
        pub official_deliver_ci: String,
        pub official_receive_custody_ci: String,
        pub official_hr_ci: String,
        pub official_supervisor_ci: String,
        pub official_director_ci: String,
        pub official_dae_ci: String,
        pub official_die_ci: String,
    }
}

pub mod development_dto {
    use chrono::NaiveDate;
    use domain::entities::part_development::ActiveModel;
    use sea_orm::{prelude::Json, DeriveIntoActiveModel, DerivePartialModel};
    use serde::{Deserialize, Serialize};

    use crate::dtos::prevention::lookup::GetNoveltyDTO;

    #[derive(Debug, Serialize, DeriveIntoActiveModel, Deserialize)]
    pub struct CreatePartDevelopmentDTO {
        pub data: String,
        pub date: sea_orm::prelude::ChronoDate,
        #[serde(skip_deserializing)]
        pub part_id: i32,
        pub type_id: i32,
    }

    #[derive(Debug, Serialize, Deserialize, DerivePartialModel)]
    #[sea_orm(
        entity = "domain::entities::part_development::Entity",
        from_query_result
    )]
    pub struct GetPartDevelopmentDTO {
        pub id: i32,
        pub data: String,
        pub date: NaiveDate,
        #[sea_orm(nested)]
        pub novelty: GetNoveltyDTO,
    }

    #[derive(Debug, Serialize, DeriveIntoActiveModel, Deserialize)]
    pub struct UpdatePartDevelopmentDTO {
        pub id: i32,
        #[serde(skip_deserializing)]
        pub part_id: i32,
        pub data: String,
        pub date: NaiveDate,
        pub type_id: i32,
    }
}
