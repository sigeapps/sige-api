use chrono::NaiveDateTime;
use domain::entities::plate::ActiveModel;
use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
use serde::{Deserialize, Serialize};

use crate::dtos::prevention::lookup::{GetBaseDTO, GetSimpleDivisionDTO, GetStateDTO};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlateRequestDTO {
    pub modified_personas: Vec<persona::ModifyPersonaDTO>,
    #[serde(flatten)]
    pub plate: CreatePlateDTO,
}

#[derive(Serialize, Deserialize)]
pub struct PlateResponseDTO {
    #[serde(flatten)]
    pub plate: GetPlateDTO,
    pub modified_personas: Vec<persona::ModifyPersonaResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone, DeriveIntoActiveModel)]
pub struct CreatePlateDTO {
    base_id: i32,
    state_id: i32,
    division_id: i32,
    datetime: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, DerivePartialModel)]
#[sea_orm(entity = "domain::entities::plate::Entity", from_query_result)]
pub struct GetPlateDTO {
    #[sea_orm(nested)]
    base: GetBaseDTO,
    #[sea_orm(nested)]
    state: GetStateDTO,
    #[sea_orm(nested)]
    division: GetSimpleDivisionDTO,
    datetime: NaiveDateTime,
}

pub mod persona {
    use domain::entities::{persona, plate_persona::ActiveModel};
    use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
    use serde::{Deserialize, Serialize};

    use crate::dtos::{
        personal::persona::SimplePersonaResponseDTO, prevention::lookup::GetPersonaStateDTO,
    };

    pub trait PlatePersona {
        fn into_persona(self) -> Result<persona::ActiveModel, sea_orm::DbErr>;
    }

    #[derive(Serialize, Deserialize, DerivePartialModel)]
    #[sea_orm(entity = "domain::entities::plate_persona::Entity", from_query_result)]
    pub struct ModifyPersonaResponse {
        #[sea_orm(nested, alias = "new_state")]
        pub new_state: GetPersonaStateDTO,
        #[sea_orm(nested, alias = "old_state")]
        pub old_state: GetPersonaStateDTO,
        #[sea_orm(nested)]
        pub persona: SimplePersonaResponseDTO,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, DeriveIntoActiveModel, Copy)]
    pub struct ModifyPersonaDTO {
        #[serde(skip_deserializing)]
        pub plate_id: i32,
        persona_id: i32,
        old_state_id: i32,
        new_state_id: i32,
    }

    impl PlatePersona for ActiveModel {
        fn into_persona(self) -> Result<persona::ActiveModel, sea_orm::DbErr> {
            let model = persona::ActiveModel {
                state_id: self.new_state_id.into(),
                id: self.persona_id,
                ..Default::default()
            };

            Ok(model)
        }
    }
}
