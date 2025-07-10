use chrono::NaiveDateTime;
use domain::entities::correspondence::ActiveModel;
use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateCorrespondenceRequest {
    #[serde(flatten)]
    pub dto: CreateCorrespondence,
    pub documents: Vec<document::CreateCorrespondenceDocument>,
}

#[derive(DeriveIntoActiveModel, Deserialize, Serialize)]
pub struct CreateCorrespondence {
    pub r#type: String,
    pub date_time: NaiveDateTime,
    pub process_date_time: NaiveDateTime,
}

#[derive(DerivePartialModel, Deserialize, Serialize)]
#[sea_orm(entity = "domain::entities::correspondence::Entity", from_query_result)]
pub struct CorrespondenceResponse {
    #[serde(flatten)]
    #[sea_orm(nested)]
    pub dto: CorrespondenceSummary,
    #[sea_orm(skip)]
    pub documents: Vec<document::CorrespondenceDocumentResponse>,
}

#[derive(DerivePartialModel, Deserialize, Serialize)]
#[sea_orm(entity = "domain::entities::correspondence::Entity", from_query_result)]
pub struct CorrespondenceSummary {
    pub id: i32,
    pub r#type: String,
    pub date_time: NaiveDateTime,
    pub process_date_time: NaiveDateTime,
    #[sea_orm(skip)]
    pub document_count: i32,
}

pub mod document {
    use domain::entities::correspondence_document::ActiveModel;
    use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
    use serde::{Deserialize, Serialize};

    #[derive(DeriveIntoActiveModel, Deserialize, Serialize)]
    pub struct CreateCorrespondenceDocument {
        #[serde(skip)]
        pub correspondence_id: i32,
        pub type_id: i32,
        pub name: String,
    }

    #[derive(DerivePartialModel, Deserialize, Serialize)]
    #[sea_orm(
        entity = "domain::entities::correspondence_document::Entity",
        from_query_result
    )]
    pub struct CorrespondenceDocumentResponse {
        #[sea_orm(nested)]
        pub r#type: DocumentTypeResponse,
        pub name: String,
    }

    #[derive(DerivePartialModel, Deserialize, Serialize)]
    #[sea_orm(entity = "domain::entities::document_type::Entity", from_query_result)]
    pub struct DocumentTypeResponse {
        pub id: i32,
        pub name: String,
    }
}
