use sea_orm::DerivePartialModel;
use serde::{Deserialize, Serialize};

use crate::dtos::personal::{
    country::GetVerificationDTO,
    persona::{
        course::Course, educational::Educational, health::Health, operational::Operational,
        others::Others, personal::Personal, record::Record,
    },
};

#[derive(Serialize, Deserialize)]
pub struct CreatePersonaDTO {
    pub personal: Personal,
    pub traits: traits::Traits,
    pub relatives: Vec<relative::Relative>,
    pub childrens: Vec<child::Child>,
    pub education: Vec<Educational>,
    pub conyuge: Option<conyuge::Conyuge>,
    pub courses: Vec<Course>,
    pub work_experiencies: Vec<labor::Labor>,
    pub health: Health,
    pub operational: Vec<Operational>,
    pub records: Vec<Record>,
    pub others: Others,
}

#[derive(Serialize, Deserialize, DerivePartialModel)]
#[sea_orm(entity = "domain::entities::persona::Entity", from_query_result)]
pub struct GetPersonaSummaryDTO {
    pub id: i32,
    pub ci: String,
    pub name: String,
    pub last_name: String,
    pub genre: String,
    #[sea_orm(nested)]
    pub verification: Option<GetVerificationDTO>,
}

pub mod course {
    use chrono::NaiveDate;
    use domain::entities::persona_course::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel)]
    pub struct Course {
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub name: String,
        pub date: NaiveDate,
        pub institution_id: i32,
        pub document: String,
    }
}

pub mod educational {
    use chrono::NaiveDate;
    use domain::entities::persona_education::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel)]
    pub struct Educational {
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
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel)]
    pub struct Health {
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub allergies: Option<String>,
        pub operations: Option<String>,
        pub blood_group: String,
        pub has_fractures: Option<bool>,
        pub fractures: Option<String>,
    }
}

pub mod operational {
    use chrono::NaiveDate;
    use domain::entities::persona_operational::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel)]
    pub struct Operational {
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub phone: String,
        pub organism_id: i32,
        pub charge_id: i32,
        pub file: String,
        pub time: i32,
        pub start_at: NaiveDate,
        pub end_at: NaiveDate,
    }
}

pub mod others {
    use domain::entities::persona::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel)]
    pub struct Others {
        pub others: String,
    }
}

pub mod personal {
    use domain::entities::persona::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel)]
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
}

pub mod record {
    use domain::entities::persona_record::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel)]
    pub struct Record {
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub name: String,
        pub r#type: String,
    }
}

pub mod conyuge {
    use chrono::NaiveDate;
    use domain::entities::persona_conyuge::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, DeriveIntoActiveModel)]
    pub struct Conyuge {
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
    use domain::entities::persona_work_experience::ActiveModel;
    use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, DerivePartialModel, DeriveIntoActiveModel)]
    #[sea_orm(
        entity = "domain::entities::persona_work_experience::Entity",
        from_query_result
    )]
    pub struct Labor {
        #[serde(skip_deserializing)]
        pub persona_id: i32,
        pub boss_phone: String,
        pub description: String,
        pub organism_id: i32,
        pub charge_id: i32,
        pub hierarchy_id: i32,
        pub photo: String,
        pub is_active: bool,
    }
}
