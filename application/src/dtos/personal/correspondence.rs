use chrono::NaiveDateTime;
use domain::entities::correspondence::ActiveModel;
use sea_orm::DeriveIntoActiveModel;
use serde::Serialize;

#[derive(Serialize)]
pub struct CreateCorrespondenceRequest {
    #[serde(flatten)]
    pub dto: CreateCorrespondence,
    pub documents: Vec<document::CreateCorrespondenceDocument>,
}

#[derive(DeriveIntoActiveModel, Serialize)]
pub struct CreateCorrespondence {
    pub r#type: String,
    pub date_time: NaiveDateTime,
    pub process_date_time: NaiveDateTime,
}

pub mod document {
    use domain::entities::correspondence_document::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::Serialize;

    #[derive(DeriveIntoActiveModel, Serialize)]
    pub struct CreateCorrespondenceDocument {
        #[sea_orm(skip_deserializing)]
        pub correspondence_id: i32,
        pub type_id: i32,
        pub name: String,
    }
}
