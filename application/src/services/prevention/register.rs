use chrono::NaiveDate;
use domain::entities::{
    division, organism,
    register::{self},
};
use sea_orm::*;

use crate::{
    connection::connect,
    dtos::prevention::register::{CreateRegisterDTO, GetRegisterDTO, UpdateRegisterDTO},
};

#[derive(Debug, Clone)]
pub struct RegisterService {
    db: DatabaseConnection,
}

impl RegisterService {
    pub async fn new(db_url: &str) -> Result<Self, DbErr> {
        let db = connect(db_url).await?;

        Ok(RegisterService { db })
    }

    pub async fn find_by_id(&self, register_id: i32) -> Result<Option<GetRegisterDTO>, DbErr> {
        let register = register::Entity::find_by_id(register_id)
            .find_also_related(organism::Entity)
            .find_also_related(division::Entity)
            .one(&self.db)
            .await?;

        let register = register.map(GetRegisterDTO::from);

        Ok(register)
    }

    pub async fn find(
        &self,
        search: Option<String>,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<Vec<GetRegisterDTO>, DbErr> {
        let mut query = register::Entity::find();

        if let Some(from_date) = from_date {
            query = query.filter(register::Column::EntryDate.gte(from_date));
        }

        if let Some(to_date) = to_date {
            query = query.filter(register::Column::EntryDate.lte(to_date));
        }

        if let Some(search) = search {
            query = query.filter(register::Column::Observations.contains(search));
        }

        let registers = query
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .find_also_related(organism::Entity)
            .find_also_related(division::Entity)
            .all(&self.db)
            .await?;

        Ok(registers.into_iter().map(GetRegisterDTO::from).collect())
    }

    pub async fn create(&self, register: CreateRegisterDTO) -> Result<(), DbErr> {
        let register = register.into_active_model();

        register::Entity::insert(register).exec(&self.db).await?;

        Ok(())
    }

    pub async fn update_exit(
        &self,
        register: UpdateRegisterDTO,
        register_id: i32,
    ) -> Result<(), DbErr> {
        register::Entity::update(register.into_active_model(register_id))
            .exec(&self.db)
            .await?;

        Ok(())
    }
}
