use domain::entities::{brigade, charge, hierarchy, official};
use sea_orm::*;

use crate::{
    connection::connect,
    dtos::prevention::official::{CreateOfficialDTO, GetOfficialDTO},
};

#[derive(Debug, Clone)]
pub struct OfficialService {
    db: DatabaseConnection,
}

impl OfficialService {
    pub async fn new(db_url: &str) -> Result<Self, DbErr> {
        let db = connect(db_url).await?;

        Ok(OfficialService { db })
    }

    pub async fn find(
        self,
        search: Option<String>,
        brigade_id: Option<i32>,
    ) -> Result<Vec<GetOfficialDTO>, DbErr> {
        let mut query = official::Entity::find()
            .select_only()
            .column_as(official::Column::Id, "id")
            .column_as(official::Column::Ci, "ci")
            .column_as(official::Column::Phone, "phone")
            .column_as(official::Column::LastName, "last_name")
            .column_as(official::Column::FirstName, "first_name")
            .column_as(brigade::Column::Name, "brigade")
            .column_as(hierarchy::Column::Name, "hierarchy")
            .column_as(charge::Column::Name, "charge")
            .join(JoinType::InnerJoin, brigade::Relation::Official.def())
            .join(JoinType::InnerJoin, hierarchy::Relation::Official.def())
            .join(JoinType::InnerJoin, charge::Relation::Official.def());

        if let Some(search) = search {
            query = query.filter(official::Column::Ci.contains(search));
        }

        if let Some(brigade_id) = brigade_id {
            query = query.filter(official::Column::BrigadeId.eq(brigade_id));
        }

        query.into_model::<GetOfficialDTO>().all(&self.db).await
    }

    pub async fn find_by_id(self, id: i32) -> Result<Option<GetOfficialDTO>, DbErr> {
        let official = official::Entity::find_by_id(id)
            .select_only()
            .column_as(official::Column::Id, "id")
            .column_as(official::Column::Ci, "ci")
            .column_as(official::Column::Phone, "phone")
            .column_as(official::Column::LastName, "last_name")
            .column_as(official::Column::FirstName, "first_name")
            .column_as(brigade::Column::Name, "brigade")
            .column_as(hierarchy::Column::Name, "hierarchy")
            .column_as(charge::Column::Name, "charge")
            .join(JoinType::InnerJoin, brigade::Relation::Official.def())
            .join(JoinType::InnerJoin, hierarchy::Relation::Official.def())
            .join(JoinType::InnerJoin, charge::Relation::Official.def())
            .into_model::<GetOfficialDTO>()
            .one(&self.db)
            .await?;

        Ok(official)
    }

    pub async fn create(self, official: CreateOfficialDTO) -> Result<(), DbErr> {
        official::Entity::insert(official.into_active_model())
            .exec(&self.db)
            .await?;

        Ok(())
    }
}
