use std::sync::Arc;

use domain::entities::{
    division, organism,
    register::{self, Column},
};
use sea_orm::*;

use crate::dtos::{
    prevention::register::{CreateRegisterDTO, GetRegisterDTO, UpdateRegisterExitDTO},
    CommonQueryFilterDTO, PaginationDTO,
};

#[derive(Debug, Clone)]
pub struct RegisterService {
    db: Arc<DatabaseConnection>,
}

impl RegisterService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        RegisterService { db }
    }

    pub async fn find_by_id(&self, register_id: i32) -> Result<Option<GetRegisterDTO>, DbErr> {
        let register = register::Entity::find_by_id(register_id)
            .find_also_related(organism::Entity)
            .find_also_related(division::Entity)
            .one(&*self.db)
            .await?;

        let register = register.map(GetRegisterDTO::from);

        Ok(register)
    }

    pub async fn find_by_ci(&self, ci: String) -> Result<Option<GetRegisterDTO>, DbErr> {
        let register = register::Entity::find()
            .filter(register::Column::Ci.eq(ci))
            .find_also_related(organism::Entity)
            .find_also_related(division::Entity)
            .one(&*self.db)
            .await?;

        let register = register.map(GetRegisterDTO::from);

        Ok(register)
    }

    pub async fn find(&self, filter: CommonQueryFilterDTO) -> Result<Vec<GetRegisterDTO>, DbErr> {
        let mut query = register::Entity::find();

        if let Some(from_date) = &filter.from_date {
            query = query.filter(Column::EntryDate.gte(*from_date));
        }

        if let Some(to_date) = &filter.to_date {
            query = query.filter(Column::EntryDate.lte(*to_date));
        }

        if let Some(search) = &filter.search {
            query = query.filter(
                Condition::any()
                    .add(Column::Ci.contains(search))
                    .add(Column::FirstName.contains(search))
                    .add(Column::LastName.contains(search))
                    .add(Column::Observations.contains(search)),
            );
        }

        if let Some(ci) = &filter.ci {
            query = query.filter(Column::Ci.eq(ci));
        }

        if let Some(sort) = &filter.sort {
            match sort.as_str() {
                "-entry_date" => {
                    query = query.order_by_desc(Column::EntryDate);
                }
                "entry_date" => {
                    query = query.order_by_asc(Column::EntryDate);
                }
                "-exit_date" => {
                    query = query.order_by_desc(Column::ExitDate);
                }
                "exit_date" => {
                    query = query.order_by_asc(Column::ExitDate);
                }
                "-last_name" => {
                    query = query.order_by_desc(Column::LastName);
                }
                "last_name" => {
                    query = query.order_by_asc(Column::LastName);
                }
                "-first_name" => {
                    query = query.order_by_desc(Column::FirstName);
                }
                "first_name" => {
                    query = query.order_by_asc(Column::FirstName);
                }
                "-ci" => {
                    query = query.order_by_desc(Column::Ci);
                }
                "ci" => {
                    query = query.order_by_asc(Column::Ci);
                }
                _ => {
                    query = query.order_by_desc(Column::EntryDate);
                }
            }
        } else {
            query = query.order_by_desc(Column::EntryDate);
        }

        let pagination = &filter.into_pagination();

        let registers = query
            .limit(pagination.limit)
            .offset(pagination.offset)
            .find_also_related(organism::Entity)
            .find_also_related(division::Entity)
            .all(&*self.db)
            .await?;

        Ok(registers.into_iter().map(GetRegisterDTO::from).collect())
    }

    pub async fn get_pagination(
        &self,
        filter: CommonQueryFilterDTO,
    ) -> Result<PaginationDTO, DbErr> {
        let mut query = register::Entity::find();

        if let Some(from_date) = &filter.from_date {
            query = query.filter(Column::EntryDate.gte(*from_date));
        }

        if let Some(to_date) = &filter.to_date {
            query = query.filter(Column::EntryDate.lte(*to_date));
        }

        if let Some(search) = &filter.search {
            query = query.filter(
                Condition::any()
                    .add(Column::Ci.contains(search))
                    .add(Column::FirstName.contains(search))
                    .add(Column::LastName.contains(search))
                    .add(Column::Observations.contains(search)),
            );
        }

        if let Some(ci) = &filter.ci {
            query = query.filter(Column::Ci.eq(ci));
        }

        let pagination = filter.into_pagination();

        let paginator = query.paginate(&*self.db, pagination.limit);

        let total_count = paginator.num_items().await?;
        let page_count = paginator.num_pages().await?;

        Ok(PaginationDTO {
            page: pagination.page,
            limit: pagination.limit,
            page_count,
            total_count,
            offset: pagination.offset,
        })
    }

    pub async fn create(&self, register: CreateRegisterDTO) -> Result<(), DbErr> {
        let register = register.into_active_model();

        register::Entity::insert(register).exec(&*self.db).await?;

        Ok(())
    }

    pub async fn update_exit(
        &self,
        register: UpdateRegisterExitDTO,
        register_id: i32,
    ) -> Result<(), DbErr> {
        let mut register = register.into_active_model();

        register.id = Set(register_id);

        register::Entity::update(register).exec(&*self.db).await?;

        Ok(())
    }
}
