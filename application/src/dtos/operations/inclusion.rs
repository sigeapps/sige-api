use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// Importar las entidades necesarias para DTOs de lectura

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
#[serde(tag = "type")]
pub enum CreateInclusion {
    #[serde(rename = "flagrant")]
    Flagrant {
        #[serde(flatten)]
        base: CreateCompleteInclusionBase,
        flagrant: flagrant::CreateFlagrant,
    },
    #[serde(rename = "complainant")]
    Complainant {
        #[serde(flatten)]
        base: CreateCompleteInclusionBase,
        complainant: complainant::CreateComplainant,
    },
    #[serde(rename = "init_order")]
    InitOrder {
        #[serde(flatten)]
        base: CreateCompleteInclusionBase,
        init_order: init_order::CreateInitOrder,
    },
    #[serde(rename = "investigation")]
    Investigation {
        #[serde(flatten)]
        base: CreateCompleteInclusionBase,
        investigation: investigation::CreateInvestigation,
    },
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct CreateCompleteInclusionBase {
    pub record: inclusion_record::CreateInclusionRecord,
    pub involved_objects: Vec<involved_object::CreateInvolvedObject>,
    pub diligencies: Vec<diligence::CreateDiligence>,
    pub technical_expertises: Vec<technical_expertise::CreateTechnicalExpertise>,
    pub arrests: Vec<arrest::CreateArrest>,
    pub confiscated_items: Vec<confiscated_item::CreateConfiscatedItem>,
    pub judicial_presentations: judicial_presentation::CreateJudicialPresentation,
    pub acusseds: Vec<i32>,
}

pub mod inclusion_record {
    use chrono::NaiveDateTime;
    use domain::entities::inclusion_records::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    use crate::enums::InclusionType;

    #[derive(Debug, Serialize, Deserialize, DeriveIntoActiveModel, ToSchema, Clone)]
    pub struct CreateInclusionRecord {
        pub record_id: i32,
        #[serde(skip_deserializing)]
        pub r#type: InclusionType,
        pub reason: String,
        pub evidences_file_path: Option<String>,
        #[schema(value_type = String)]
        pub date_time: NaiveDateTime,
        pub auth_persona_id: i32,
    }
}

pub mod complainant {
    use chrono::NaiveDate;
    use domain::entities::complainants::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    #[derive(Debug, Serialize, Deserialize, DeriveIntoActiveModel, ToSchema, Clone)]
    pub struct CreateComplainant {
        pub ci: String,
        pub name: String,
        pub last_name: String,
        pub age: Option<i32>,
        #[schema(value_type = String)]
        pub birth_date: NaiveDate,
    }
}

pub mod involved_object {
    use domain::entities::involved_objects::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    #[derive(Debug, Serialize, Deserialize, DeriveIntoActiveModel, ToSchema, Clone)]
    pub struct CreateInvolvedObject {
        pub r#type: String,
        pub serial: Option<String>,
        pub physical_signs: Option<String>,
        pub description: String,
        pub photo_path: Option<String>,
    }
}

pub mod diligence {
    use chrono::NaiveDateTime;
    use domain::entities::diligencies::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    #[derive(Debug, Serialize, Deserialize, DeriveIntoActiveModel, ToSchema, Clone)]
    pub struct CreateDiligence {
        #[schema(value_type = String)]
        pub date_time: NaiveDateTime,
        pub brigade_boss_id: i32,
        pub nomenclature: String,
        pub inspections: Option<String>,
        pub evidences: Option<String>,
    }
}

pub mod technical_expertise {
    use chrono::NaiveDateTime;
    use domain::entities::technical_expertises::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    #[derive(Debug, Serialize, Deserialize, DeriveIntoActiveModel, ToSchema, Clone)]
    pub struct CreateTechnicalExpertise {
        #[schema(value_type = String)]
        pub date_time: NaiveDateTime,
        pub evidence_type: String,
        pub description: String,
        pub chain_of_custody: Option<String>,
        #[schema(value_type = String)]
        pub results_date_time: NaiveDateTime,
        pub results: Option<String>,
        pub documents_path: Option<String>,
    }
}

pub mod arrest {
    use chrono::NaiveDateTime;
    use domain::entities::arrests::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    #[derive(Debug, Serialize, Deserialize, DeriveIntoActiveModel, ToSchema, Clone)]
    pub struct CreateArrest {
        #[schema(value_type = String)]
        pub date_time: NaiveDateTime,
        pub detained_person_id: i32,
        pub arrest_type: String,
        pub reason: String,
        pub photo_path: Option<String>,
    }
}

pub mod confiscated_item {
    use chrono::NaiveDateTime;
    use domain::entities::confiscated_items::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    #[derive(Debug, Serialize, Deserialize, DeriveIntoActiveModel, ToSchema, Clone)]
    pub struct CreateConfiscatedItem {
        #[schema(value_type = String)]
        pub date_time: NaiveDateTime,
        pub item_type: String,
        pub description: String,
        pub photo_path: Option<String>,
    }
}

pub mod judicial_presentation {
    use chrono::NaiveDateTime;
    use domain::entities::judicial_presentations::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    #[derive(Debug, Serialize, Deserialize, DeriveIntoActiveModel, ToSchema, Clone)]
    pub struct CreateJudicialPresentation {
        #[schema(value_type = String)]
        pub date_time: NaiveDateTime,
        pub authority_assigned: String,
        pub authority_name: String,
        pub authority_phone: i32,
        pub assigned_court: Option<String>,
        pub authority_decision: Option<String>,
        pub confinement_place: String,
    }
}

pub mod flagrant {
    use domain::entities::flagrants::ActiveModel;
    use sea_orm::{prelude::Decimal, DeriveIntoActiveModel};
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    #[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
    pub struct CreateFlagrant {
        #[serde(flatten)]
        pub base: Base,
        pub personas_ids: Vec<i32>,
    }

    #[derive(Debug, Serialize, Deserialize, DeriveIntoActiveModel, ToSchema, Clone)]
    pub struct Base {
        #[schema(value_type = i32)]
        pub latitude: Decimal,
        #[schema(value_type = i32)]
        pub longitude: Decimal,
    }
}

pub mod init_order {
    use domain::entities::init_orders::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    #[derive(Debug, Serialize, Deserialize, DeriveIntoActiveModel, ToSchema, Clone)]
    pub struct CreateInitOrder {
        pub annotated_id: i32,
        pub organism_id: i32,
        #[schema(value_type = String)]
        pub r#type: String,
        #[schema(value_type = String)]
        pub instruction: String,
    }
}

pub mod investigation {
    use domain::entities::investigations::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    #[derive(Debug, Serialize, Deserialize, DeriveIntoActiveModel, ToSchema, Clone)]
    pub struct CreateInvestigation {
        pub annotated_investigation_id: i32,
        pub source: String,
        pub classification: String,
        pub r#type: String,
    }
}
