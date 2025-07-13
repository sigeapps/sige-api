use domain::entities::persona::ActiveModel;
use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
use serde::{Deserialize, Serialize};
use situation::SimpleDivisionDTO;

use crate::dtos::{
    personal::{
        country::GetVerificationDTO,
        persona::{
            course::Course,
            educational::Educational,
            health::Health,
            operational::Operational,
            others::Others,
            personal::{GetPersonalDTO, Personal, UpdatePersonal},
            record::Record,
            situation::{GetSituationDTO, UpdateSituationSummaryDTO},
        },
    },
    prevention::lookup::{GetBaseDTO, GetHierarchyDTO, GetPersonaStateDTO, GetStateDTO},
};

#[derive(Serialize, Deserialize)]
pub struct CreatePersonaDTO {
    pub personal: Personal,
    pub traits: traits::Traits,
    pub relatives: Vec<relative::Relative>,
    pub childrens: Vec<child::Child>,
    pub education: Vec<Educational>,
    pub conyuge: Option<conyuge::Conyuge>,
    pub courses: Vec<course::Course>,
    pub work_experiencies: Vec<labor::Labor>,
    pub health: Health,
    pub operational: Vec<Operational>,
    pub records: Vec<Record>,
    pub situation: situation::Situation,
    pub others: Others,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePersonaSummaryDTO {
    #[serde(flatten)]
    pub situation: UpdateSituationSummaryDTO,
    #[serde(flatten)]
    pub persona: UpdatePersonaSummary,
}

#[derive(Serialize, Deserialize, Debug, DeriveIntoActiveModel)]
pub struct UpdatePersonaSummary {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub genre: String,
    #[serde(rename = "work_state_id")]
    pub state_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct UpdatePersonaDTO {
    pub personal: UpdatePersonal,
    pub traits: Option<traits::Traits>,
    pub relatives: Vec<relative::Relative>,
    pub childrens: Vec<child::Child>,
    pub education: Vec<Educational>,
    pub conyuge: Option<conyuge::Conyuge>,
    pub courses: Vec<Course>,
    pub work_experiencies: Vec<labor::Labor>,
    pub health: Option<Health>,
    pub operational: Vec<Operational>,
    pub records: Vec<Record>,
    pub situation: Option<situation::Situation>,
    pub others: Option<Others>,
}

#[derive(Serialize, Deserialize)]
pub struct GetPersonaDTO {
    pub personal: GetPersonalDTO,
    pub traits: Option<traits::Traits>,
    pub relatives: Vec<relative::Relative>,
    pub childrens: Vec<child::Child>,
    pub education: Vec<Educational>,
    pub conyuge: Option<conyuge::Conyuge>,
    pub courses: Vec<Course>,
    pub work_experiencies: Vec<labor::Labor>,
    pub health: Option<Health>,
    pub operational: Vec<Operational>,
    pub records: Vec<Record>,
    pub situation: Option<GetSituationDTO>,
    pub others: Option<Others>,
}

#[derive(Serialize, Deserialize, DerivePartialModel, Debug, Clone)]
#[sea_orm(entity = "domain::entities::persona::Entity", from_query_result)]
pub struct SimplePersonaResponseDTO {
    pub id: i32,
    pub ci: String,
    pub name: String,
    pub last_name: String,
}

#[derive(Serialize, Deserialize, DerivePartialModel, Clone, Debug)]
#[sea_orm(entity = "domain::entities::persona::Entity", from_query_result)]
pub struct GetPersonaSummaryDTO {
    pub id: i32,
    pub ci: String,
    pub name: String,
    pub last_name: String,
    pub genre: String,
    #[sea_orm(nested)]
    pub verification: Option<GetVerificationDTO>,
    #[sea_orm(nested)]
    pub work_state: GetPersonaStateDTO,
    #[sea_orm(nested)]
    pub division: Option<SimpleDivisionDTO>,
    #[sea_orm(nested)]
    pub state: Option<GetStateDTO>,
    #[sea_orm(nested)]
    pub base: Option<GetBaseDTO>,
    #[sea_orm(nested)]
    pub hierarchy: Option<GetHierarchyDTO>,
}

pub mod course {
    use chrono::NaiveDate;
    use domain::entities::persona_course::ActiveModel;
    use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel, DerivePartialModel)]
    #[sea_orm(entity = "domain::entities::persona_course::Entity", from_query_result)]
    pub struct Course {
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub name: String,
        pub date: NaiveDate,
        pub institution_id: i32,
        pub document: String,
    }
}

pub mod situation {
    use chrono::NaiveDate;
    use domain::entities::persona_situation::ActiveModel;
    use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel, DerivePartialModel)]
    #[sea_orm(
        entity = "domain::entities::persona_situation::Entity",
        from_query_result
    )]
    pub struct Situation {
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub situation_type: String,
        pub date: NaiveDate,
        pub process_id: Option<i32>,

        pub entry_type: Option<String>,
        pub division_id: Option<i32>,
        pub state_id: Option<i32>,
        pub base_id: Option<i32>,
        pub hierarchy_id: Option<i32>,
        pub charge_id: Option<i32>,
        pub division_origin_id: Option<i32>,
        pub organism_origin_id: Option<i32>,
        pub requested_by_id: i32,
    }

    #[derive(Serialize, Deserialize, Debug, DeriveIntoActiveModel)]
    pub struct UpdateSituationSummaryDTO {
        pub state_id: i32,
        pub base_id: i32,
        pub division_id: i32,
    }

    // DTOs simplificados para evitar conflictos de alias
    #[derive(Serialize, Deserialize, DerivePartialModel, Clone, Debug)]
    #[sea_orm(entity = "domain::entities::division::Entity", from_query_result)]
    pub struct SimpleDivisionDTO {
        pub id: i32,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, DerivePartialModel)]
    #[sea_orm(entity = "domain::entities::state::Entity", from_query_result)]
    pub struct SimpleStateDTO {
        pub id: i32,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, DerivePartialModel)]
    #[sea_orm(entity = "domain::entities::base::Entity", from_query_result)]
    pub struct SimpleBaseDTO {
        pub id: i32,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, DerivePartialModel)]
    #[sea_orm(entity = "domain::entities::hierarchy::Entity", from_query_result)]
    pub struct SimpleHierarchyDTO {
        pub id: i32,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, DerivePartialModel)]
    #[sea_orm(entity = "domain::entities::charge::Entity", from_query_result)]
    pub struct SimpleChargeDTO {
        pub id: i32,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, DerivePartialModel)]
    #[sea_orm(
        entity = "domain::entities::organism::Entity",
        alias = "organism_origin",
        from_query_result
    )]
    pub struct SimpleOrganismDTO {
        pub id: i32,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel)]
    pub struct UpdateSituationDTO {
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub situation_type: String,
        pub date: NaiveDate,
        pub process_id: Option<i32>,

        pub entry_type: Option<String>,
        pub division_id: Option<i32>,
        pub state_id: Option<i32>,
        pub base_id: Option<i32>,
        pub hierarchy_id: Option<i32>,
        pub charge_id: Option<i32>,
        pub division_origin_id: Option<i32>,
        pub organism_origin_id: Option<i32>,
        pub requested_by_id: i32,
    }

    #[derive(Serialize, Deserialize, DerivePartialModel)]
    #[sea_orm(
        entity = "domain::entities::persona_situation::Entity",
        from_query_result
    )]
    pub struct GetSituationDTO {
        pub id: i32,
        pub situation_type: String,
        pub date: NaiveDate,
        pub process_id: Option<i32>,
        pub entry_type: Option<String>,
        #[sea_orm(nested)]
        pub division: Option<SimpleDivisionDTO>,
        #[sea_orm(nested)]
        pub state: Option<SimpleStateDTO>,
        #[sea_orm(nested)]
        pub base: Option<SimpleBaseDTO>,
        #[sea_orm(nested)]
        pub hierarchy: Option<SimpleHierarchyDTO>,
        #[sea_orm(nested)]
        pub charge: Option<SimpleChargeDTO>,
        #[sea_orm(nested)]
        pub division_origin: Option<SimpleDivisionDTO>,
        #[sea_orm(nested)]
        pub organism_origin: Option<SimpleOrganismDTO>,
        // TODO: Verificar si es necesario
        pub requested_by_id: i32,
    }
}

pub mod educational {
    use chrono::NaiveDate;
    use domain::entities::persona_education::ActiveModel;
    use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel, DerivePartialModel)]
    #[sea_orm(
        entity = "domain::entities::persona_education::Entity",
        from_query_result
    )]
    pub struct Educational {
        #[serde(skip_deserializing)]
        pub id: i32,
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub end_date: NaiveDate,
        pub grade: String,
        pub institution_id: i32,
        pub profession_id: i32,
        pub photo: String,
    }
}

pub mod health {
    use domain::entities::persona_health::ActiveModel;
    use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel, DerivePartialModel)]
    #[sea_orm(entity = "domain::entities::persona_health::Entity", from_query_result)]
    pub struct Health {
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub allergies: Option<String>,
        pub operations: Option<String>,
        pub blood_group: String,
        pub has_fractures: Option<bool>,
        pub fractures: Option<String>,
        pub known_conditions: Option<String>,
        pub known_conditions_description: Option<String>,
    }
}

pub mod operational {
    use chrono::NaiveDate;
    use domain::entities::persona_operational::ActiveModel;
    use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel, DerivePartialModel)]
    #[sea_orm(
        entity = "domain::entities::persona_operational::Entity",
        from_query_result
    )]
    pub struct Operational {
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub organism_id: i32,
        pub withdrawal_type: String,
        pub hierarchy_id: i32,
        pub charge_id: i32,
        pub start_at: NaiveDate,
        pub end_at: NaiveDate,
        pub time: String,
        pub boss_name: String,
        pub boss_phone: String,
        pub description: Option<String>,
        pub is_active: Option<bool>,
        pub file: Option<String>,
    }

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel)]
    pub struct GetOperationalDTO {
        pub id: i32,
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub organism_id: i32,
        pub withdrawal_type: String,
        pub hierarchy_id: i32,
        pub charge_id: i32,
        pub start_at: NaiveDate,
        pub end_at: NaiveDate,
        pub time: String,
        pub boss_name: String,
        pub boss_phone: String,
        pub description: Option<String>,
        pub is_active: Option<bool>,
        pub file: Option<String>,
    }
}

pub mod others {
    use domain::entities::persona::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel)]
    pub struct Others {
        pub others: Option<String>,
    }
}

pub mod personal {
    use domain::entities::persona::ActiveModel;
    use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, DerivePartialModel)]
    #[sea_orm(entity = "domain::entities::persona::Entity", from_query_result)]
    pub struct GetPersonalDTO {
        pub id: i32,
        pub ci: String,
        pub front_photo: Option<String>,
        pub back_photo: Option<String>,
        pub passport_number: Option<String>,
        pub passport_expiration: Option<String>,
        pub passport_years_valid: Option<i32>,
        pub name: String,
        pub last_name: String,
        pub birthdate: String,
        pub email: String,
        pub age: i32,
        pub birthplace: String,
        pub address: String,
        pub phone: String,
        pub coordinates: Option<String>,
        pub genre: String,
        pub status_civil: String,
        pub bank_account: String,
        pub homeland_ci: String,
        pub vehicle_license: String,
        pub others: Option<String>,
    }

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel, DerivePartialModel)]
    #[sea_orm(entity = "domain::entities::persona::Entity", from_query_result)]
    pub struct Personal {
        pub passport_number: Option<String>,
        pub passport_expiration: Option<String>,
        pub passport_years_valid: Option<i32>,
        pub name: String,
        pub last_name: String,
        pub birthdate: String,
        pub birthplace: String,
        pub address: String,
        pub phone: String,
        pub email: String,
        pub coordinates: Option<String>,
        pub homeland_ci: String,
        pub bank_account: String,
        pub front_photo: String,
        pub back_photo: String,
        pub ci: String,
        pub genre: String,
        pub status_civil: String,
        pub vehicle_license: String,
        pub age: i32,
        #[serde(skip_deserializing)]
        pub others: String,
    }

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel, DerivePartialModel)]
    #[sea_orm(entity = "domain::entities::persona::Entity", from_query_result)]
    pub struct UpdatePersonal {
        #[serde(skip_deserializing)]
        pub id: i32,
        pub passport_number: Option<String>,
        pub passport_expiration: Option<String>,
        pub passport_years_valid: Option<i32>,
        pub birthdate: String,
        pub birthplace: String,
        pub address: String,
        pub phone: String,
        pub email: String,
        pub coordinates: Option<String>,
        pub homeland_ci: String,
        pub bank_account: String,
        pub front_photo: String,
        pub back_photo: String,
        pub genre: String,
        pub status_civil: String,
        pub vehicle_license: String,
        pub age: i32,
        #[serde(skip_deserializing)]
        pub others: String,
    }
}

pub mod record {
    use chrono::NaiveDate;
    use domain::entities::persona_record::ActiveModel;
    use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel, DerivePartialModel)]
    #[sea_orm(entity = "domain::entities::persona_record::Entity", from_query_result)]
    pub struct Record {
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub name: String,
        pub r#type: String,
        pub requested_by_id: i32,
        pub date: NaiveDate,
        pub description: String,
    }

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel)]
    pub struct GetRecordDTO {
        pub id: i32,
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub name: String,
        pub r#type: String,
        pub requested_by_id: i32,
        pub date: NaiveDate,
        pub description: String,
    }
}

pub mod conyuge {
    use chrono::NaiveDate;
    use domain::entities::persona_conyuge::ActiveModel;
    use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel, DerivePartialModel)]
    #[sea_orm(
        entity = "domain::entities::persona_conyuge::Entity",
        from_query_result
    )]
    pub struct Conyuge {
        #[serde(skip_deserializing)]
        pub id: i32,
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub ci: String,
        pub name: String,
        pub last_name: String,
        pub birthdate: NaiveDate,
        pub age: i32,
        pub phone: String,
    }

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel)]
    pub struct GetConyugeDTO {
        pub id: i32,
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub ci: String,
        pub name: String,
        pub last_name: String,
        pub birthdate: NaiveDate,
        pub age: i32,
        pub phone: String,
    }
}

pub mod relative {
    use chrono::NaiveDate;
    use domain::entities::persona_relative::ActiveModel;
    use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
    use serde::{Deserialize, Serialize};

    use crate::dtos::WithId;

    #[derive(Serialize, Deserialize, DerivePartialModel, DeriveIntoActiveModel)]
    #[sea_orm(
        entity = "domain::entities::persona_relative::Entity",
        from_query_result
    )]
    pub struct Relative {
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub name: String,
        pub last_name: String,
        pub birthdate: NaiveDate,
        pub age: i32,
        pub phone: String,
        pub address: String,
        pub relationship_id: i32,
    }

    pub type GetRelativeDTO = WithId<Relative>;
}

pub mod child {
    use chrono::NaiveDate;
    use domain::entities::persona_children::ActiveModel;
    use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, DerivePartialModel, DeriveIntoActiveModel)]
    #[sea_orm(
        entity = "domain::entities::persona_children::Entity",
        from_query_result
    )]
    pub struct Child {
        pub name: String,
        pub last_name: String,
        pub age: i32,
        pub birthdate: NaiveDate,
        pub grade: String,
        #[serde(skip_deserializing)]
        pub persona_id: i32,
    }
}

pub mod traits {
    use domain::entities::persona_traits::ActiveModel;
    use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, DerivePartialModel, DeriveIntoActiveModel)]
    #[sea_orm(entity = "domain::entities::persona_traits::Entity", from_query_result)]
    pub struct Traits {
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub height: i32,
        pub others: Option<String>,
        pub build: String,
        pub skin_color: String,
        pub has_tattoos: Option<bool>,
        pub eyes_color: String,
        pub eyes_type: String,
        pub hair_color: String,
        pub hair_type: String,
        pub eyebrow_type: String,
        pub nose_type: String,
        pub face_type: String,
        pub lips_type: String,
        pub hands_type: String,
    }
}

pub mod labor {
    use chrono::NaiveDate;
    use domain::entities::persona_work_experience::ActiveModel;
    use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
    use serde::{Deserialize, Serialize};

    use crate::dtos::WithId;

    #[derive(Serialize, Deserialize, DerivePartialModel, DeriveIntoActiveModel)]
    #[sea_orm(
        entity = "domain::entities::persona_work_experience::Entity",
        from_query_result
    )]
    pub struct Labor {
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub enterprise_name: String,
        pub charge: String,
        pub boss_name: String,
        pub boss_phone: String,
        pub is_active: Option<bool>,
        pub description: String,
        pub start_at: NaiveDate,
        pub end_at: NaiveDate,
        pub time: String,
        pub photo: Option<String>,
    }

    pub type GetLaborDTO = WithId<Labor>;
}
