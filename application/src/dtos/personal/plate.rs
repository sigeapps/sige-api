use chrono::NaiveDateTime;
use domain::entities::plate::ActiveModel;
use sea_orm::DeriveIntoActiveModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlateRequestDTO {
    pub modified_personas: Vec<persona::ModifyPersonaDTO>,
    #[serde(flatten)]
    pub plate: CreatePlateDTO,
}

#[derive(Serialize, Deserialize, Debug, Clone, DeriveIntoActiveModel)]
pub struct CreatePlateDTO {
    base_id: i32,
    state_id: i32,
    division_id: i32,
    datetime: NaiveDateTime,
}

pub mod persona {
    use domain::entities::{persona, plate_persona::ActiveModel};
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};

    pub trait PlatePersona {
        fn into_persona(self) -> Result<persona::ActiveModel, sea_orm::DbErr>;
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
