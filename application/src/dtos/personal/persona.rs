use domain::entities::persona::ActiveModel;
use domain::entities::{hierarchy, sea_orm_active_enums::TypeEnum};
use sea_orm::{DeriveIntoActiveModel, DerivePartialModel, Iterable};
use serde::{Deserialize, Serialize};
use situation::SimpleDivisionDTO;
use utoipa::openapi::schema::SchemaType;
use utoipa::openapi::{ObjectBuilder, Type};
use utoipa::{PartialSchema, ToSchema};

use crate::dtos::{
    personal::persona::{
        course::Course,
        educational::Educational,
        health::Health,
        operational::Operational,
        others::Others,
        personal::{GetPersonalDTO, Personal, UpdatePersonal},
        record::Record,
        situation::{GetSituationDTO, UpdateSituationSummaryDTO},
    },
    prevention::lookup::{GetBaseDTO, GetHierarchyDTO, GetPersonaStateDTO, GetStateDTO},
};

// DTO universal para crear persona (RESTful)
#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreatePersonaDTO {
    pub r#type: PersonaType,
    pub personal: Personal,
    pub situation: Option<situation::Situation>,

    pub traits: Option<traits::Traits>,
    pub health: Option<Health>,
    pub relatives: Option<Vec<relative::Relative>>,
    pub childrens: Option<Vec<child::Child>>,
    pub education: Option<Vec<Educational>>,
    pub conyuge: Option<conyuge::Conyuge>,
    pub courses: Option<Vec<course::Course>>,
    pub work_experiencies: Option<Vec<labor::Labor>>,
    pub operational: Option<Vec<Operational>>,
    pub records: Option<Vec<Record>>,
    pub others: Option<Others>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PersonaType(pub TypeEnum);

impl From<PersonaType> for TypeEnum {
    fn from(persona_type: PersonaType) -> Self {
        persona_type.0
    }
}

impl Default for PersonaType {
    fn default() -> Self {
        PersonaType(TypeEnum::Civil)
    }
}

impl From<TypeEnum> for PersonaType {
    fn from(type_enum: TypeEnum) -> Self {
        PersonaType(type_enum)
    }
}

impl sea_orm::IntoActiveValue<TypeEnum> for PersonaType {
    fn into_active_value(self) -> sea_orm::ActiveValue<TypeEnum> {
        sea_orm::ActiveValue::Set(self.0)
    }
}

impl ToSchema for PersonaType {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("PersonaType")
    }
}

impl PartialSchema for PersonaType {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        ObjectBuilder::new()
            .schema_type(SchemaType::new(Type::String))
            .enum_values(Some(
                TypeEnum::iter()
                    .map(|v| format!("{:?}", v))
                    .collect::<Vec<String>>(),
            ))
            .into()
    }
}

// DTO separado para crear persona completa (backward compatibility)
#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreatePersonaCompleteDTO {
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

// DTO básico para crear persona con información mínima (RESTful)
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CreatePersonaBasicDTO {
    pub ci: String,
    pub name: String,
    pub last_name: String,
    pub genre: String,
    pub birthdate: String,
    pub age: i32,
    pub persona_type: String, // "funcionario", "imputado", "civil"

    // Información mínima de situación
    pub state_id: Option<i32>,
    pub base_id: Option<i32>,
    pub division_id: Option<i32>,
    pub requested_by_id: i32,

    // Campos opcionales básicos
    pub email: Option<String>,
    pub birthplace: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub status_civil: Option<String>,
    pub passport_number: Option<String>,
    pub passport_expiration: Option<String>,
    pub passport_years_valid: Option<i32>,
    pub front_photo: Option<String>,
    pub back_photo: Option<String>,
    pub coordinates: Option<String>,
    pub bank_account: Option<String>,
    pub homeland_ci: Option<String>,
    pub vehicle_license: Option<String>,
}

// DTO para actualizar información básica de persona
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct UpdatePersonaBasicDTO {
    pub name: Option<String>,
    pub last_name: Option<String>,
    pub genre: Option<String>,
    pub birthdate: Option<String>,
    pub age: Option<i32>,
    pub email: Option<String>,
    pub birthplace: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub status_civil: Option<String>,
    pub passport_number: Option<String>,
    pub passport_expiration: Option<String>,
    pub passport_years_valid: Option<i32>,
    pub front_photo: Option<String>,
    pub back_photo: Option<String>,
    pub coordinates: Option<String>,
    pub bank_account: Option<String>,
    pub homeland_ci: Option<String>,
    pub vehicle_license: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct UpdatePersonaSummaryDTO {
    #[serde(flatten)]
    pub situation: UpdateSituationSummaryDTO,
    #[serde(flatten)]
    pub persona: UpdatePersonaSummary,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CreatePersonaSummaryDTO {
    #[serde(flatten)]
    pub situation: UpdateSituationSummaryDTO,
    #[serde(flatten)]
    pub persona: CreatePersonaSummary,
}

#[derive(Serialize, Deserialize, Debug, DeriveIntoActiveModel, ToSchema)]
pub struct CreatePersonaSummary {
    pub ci: String,
    pub genre: String,
}

#[derive(Serialize, Deserialize, Debug, DeriveIntoActiveModel, ToSchema)]
pub struct UpdatePersonaSummary {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub genre: String,
    #[serde(rename = "work_state_id")]
    pub state_id: i32,
}

#[derive(Serialize, Deserialize, ToSchema)]
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

#[derive(Serialize, Deserialize, ToSchema)]
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

#[derive(Serialize, Deserialize, DerivePartialModel, Debug, Clone, ToSchema)]
#[sea_orm(entity = "domain::entities::persona::Entity", from_query_result)]
pub struct SimplePersonaResponseDTO {
    pub id: i32,
    pub ci: String,
    pub name: String,
    pub last_name: String,
}

#[derive(Serialize, Deserialize, DerivePartialModel, Debug, Clone, ToSchema)]
#[sea_orm(entity = "domain::entities::persona::Entity", from_query_result)]
pub struct PersonaSummary {
    pub id: i32,
    pub ci: String,
    pub name: String,
    pub last_name: String,
    #[sea_orm(from_expr = "hierarchy::Column::Name")]
    pub hierarchy: String,
}

#[derive(Serialize, Deserialize, DerivePartialModel, Clone, Debug, ToSchema)]
#[sea_orm(entity = "domain::entities::persona::Entity", from_query_result)]
pub struct GetPersonaSummaryDTO {
    pub id: i32,
    pub ci: String,
    pub name: String,
    pub last_name: String,
    pub genre: String,
    pub r#type: String,
    #[sea_orm(nested)]
    pub work_state: Option<GetPersonaStateDTO>,
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
    use utoipa::ToSchema;

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel, DerivePartialModel, ToSchema)]
    #[sea_orm(entity = "domain::entities::persona_course::Entity", from_query_result)]
    pub struct Course {
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub name: String,
        #[schema(value_type = String)]
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
    use utoipa::ToSchema;

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel, DerivePartialModel, ToSchema)]
    #[sea_orm(
        entity = "domain::entities::persona_situation::Entity",
        from_query_result
    )]
    pub struct Situation {
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub situation_type: String,
        #[schema(value_type = String)]
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

    #[derive(Serialize, Deserialize, Debug, DeriveIntoActiveModel, ToSchema)]
    pub struct UpdateSituationSummaryDTO {
        pub state_id: i32,
        pub base_id: i32,
        pub division_id: i32,
    }

    // DTOs simplificados para evitar conflictos de alias
    #[derive(Serialize, Deserialize, DerivePartialModel, Clone, Debug, ToSchema)]
    #[sea_orm(entity = "domain::entities::division::Entity", from_query_result)]
    pub struct SimpleDivisionDTO {
        pub id: i32,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, DerivePartialModel, ToSchema)]
    #[sea_orm(entity = "domain::entities::state::Entity", from_query_result)]
    pub struct SimpleStateDTO {
        pub id: i32,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, DerivePartialModel, ToSchema)]
    #[sea_orm(entity = "domain::entities::base::Entity", from_query_result)]
    pub struct SimpleBaseDTO {
        pub id: i32,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, DerivePartialModel, ToSchema)]
    #[sea_orm(entity = "domain::entities::hierarchy::Entity", from_query_result)]
    pub struct SimpleHierarchyDTO {
        pub id: i32,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, DerivePartialModel, ToSchema)]
    #[sea_orm(entity = "domain::entities::charge::Entity", from_query_result)]
    pub struct SimpleChargeDTO {
        pub id: i32,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, DerivePartialModel, ToSchema)]
    #[sea_orm(
        entity = "domain::entities::organism::Entity",
        alias = "organism_origin",
        from_query_result
    )]
    pub struct SimpleOrganismDTO {
        pub id: i32,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel, ToSchema)]
    pub struct UpdateSituationDTO {
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub situation_type: String,
        #[schema(value_type = String)]
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

    #[derive(Serialize, Deserialize, DerivePartialModel, ToSchema)]
    #[sea_orm(
        entity = "domain::entities::persona_situation::Entity",
        from_query_result
    )]
    pub struct GetSituationDTO {
        pub id: i32,
        pub situation_type: String,
        #[schema(value_type = String)]
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
    use utoipa::ToSchema;

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel, DerivePartialModel, ToSchema)]
    #[sea_orm(
        entity = "domain::entities::persona_education::Entity",
        from_query_result
    )]
    pub struct Educational {
        #[serde(skip_deserializing)]
        pub id: i32,
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        #[schema(value_type = String)]
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
    use utoipa::ToSchema;

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel, DerivePartialModel, ToSchema)]
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
    use utoipa::ToSchema;

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel, DerivePartialModel, ToSchema)]
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
        #[schema(value_type = String)]
        pub start_at: NaiveDate,
        #[schema(value_type = String)]
        pub end_at: NaiveDate,
        pub time: String,
        pub boss_name: String,
        pub boss_phone: String,
        pub description: Option<String>,
        pub is_active: Option<bool>,
        pub file: Option<String>,
    }

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel, ToSchema)]
    pub struct GetOperationalDTO {
        pub id: i32,
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub organism_id: i32,
        pub withdrawal_type: String,
        pub hierarchy_id: i32,
        pub charge_id: i32,
        #[schema(value_type = String)]
        pub start_at: NaiveDate,
        #[schema(value_type = String)]
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
    use utoipa::ToSchema;

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel, ToSchema)]
    pub struct Others {
        pub others: Option<String>,
    }
}

pub mod personal {
    use domain::entities::persona::ActiveModel;
    use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    use crate::dtos::personal::persona::PersonaType;

    #[derive(Serialize, Deserialize, DerivePartialModel, ToSchema)]
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

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel, ToSchema)]
    // #[sea_orm(entity = "domain::entities::persona::Entity", from_query_result)]
    pub struct Personal {
        pub passport_number: Option<String>,
        pub passport_expiration: Option<String>,
        #[serde(skip_deserializing)]
        pub r#type: PersonaType, // Cambiar de PersonaType a TypeEnum
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
        pub others: Option<String>,
    }

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel, DerivePartialModel, ToSchema)]
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
        pub others: Option<String>,
    }
}

pub mod record {
    use chrono::NaiveDate;
    use domain::entities::persona_record::ActiveModel;
    use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel, DerivePartialModel, ToSchema)]
    #[sea_orm(entity = "domain::entities::persona_record::Entity", from_query_result)]
    pub struct Record {
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub name: String,
        pub r#type: String,
        pub requested_by_id: i32,
        #[schema(value_type = String)]
        pub date: NaiveDate,
        pub description: String,
    }

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel, ToSchema)]
    pub struct GetRecordDTO {
        pub id: i32,
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub name: String,
        pub r#type: String,
        pub requested_by_id: i32,
        #[schema(value_type = String)]
        pub date: NaiveDate,
        pub description: String,
    }
}

pub mod conyuge {
    use chrono::NaiveDate;
    use domain::entities::persona_conyuge::ActiveModel;
    use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel, DerivePartialModel, ToSchema)]
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
        #[schema(value_type = String)]
        pub birthdate: NaiveDate,
        pub age: i32,
        pub phone: String,
    }

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel, ToSchema)]
    pub struct GetConyugeDTO {
        pub id: i32,
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub ci: String,
        pub name: String,
        pub last_name: String,
        #[schema(value_type = String)]
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
    use utoipa::ToSchema;

    use crate::dtos::WithId;

    #[derive(Serialize, Deserialize, DerivePartialModel, DeriveIntoActiveModel, ToSchema)]
    #[sea_orm(
        entity = "domain::entities::persona_relative::Entity",
        from_query_result
    )]
    pub struct Relative {
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub name: String,
        pub last_name: String,
        #[schema(value_type = String)]
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
    use utoipa::ToSchema;

    #[derive(Serialize, Deserialize, DerivePartialModel, DeriveIntoActiveModel, ToSchema)]
    #[sea_orm(
        entity = "domain::entities::persona_children::Entity",
        from_query_result
    )]
    pub struct Child {
        pub name: String,
        pub last_name: String,
        pub age: i32,
        #[schema(value_type = String)]
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
    use utoipa::ToSchema;

    #[derive(Serialize, Deserialize, DerivePartialModel, DeriveIntoActiveModel, ToSchema)]
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
    use utoipa::ToSchema;

    use crate::dtos::WithId;

    #[derive(Serialize, Deserialize, DerivePartialModel, DeriveIntoActiveModel, ToSchema)]
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
        #[schema(value_type = String)]
        pub start_at: NaiveDate,
        #[schema(value_type = String)]
        pub end_at: NaiveDate,
        pub time: String,
        pub photo: Option<String>,
    }

    pub type GetLaborDTO = WithId<Labor>;
}
