use crate::auth::{FilterByClaims, UserClaims};
use chrono::Local;
use domain::entities::{
    division, organism,
    register::{self, Column},
};
use sea_orm::*;

use crate::{
    api::ApiContext,
    auth::{HasBaseId, UserStamp},
    dtos::{
        prevention::register::{CreateRegisterDTO, GetRegisterDTO, UpdateRegisterExitDTO},
        CommonQueryFilterDTO, PaginationDTO,
    },
    impl_filter_by_base_id,
};

impl HasBaseId for register::ActiveModel {
    fn set_base_id(mut self, id: i32) -> Self {
        self.base_id = Set(id);

        self
    }
}

impl_filter_by_base_id!(register, BaseId);

// Safety: Uso de unwrap en servicio ya que si el usuario es None sera manejado por el extension de Axum

#[derive(Debug, Clone)]
pub struct RegisterService {}

impl RegisterService {
    pub async fn find_by_id(
        ctx: ApiContext,
        register_id: i32,
    ) -> Result<Option<GetRegisterDTO>, DbErr> {
        let register = register::Entity::find_by_id(register_id)
            .filter_by_claims(ctx.claims.unwrap())
            .find_also_related(organism::Entity)
            .find_also_related(division::Entity)
            .one(&ctx.db)
            .await?;

        let register = register.map(GetRegisterDTO::from);

        Ok(register)
    }

    pub async fn find_by_ci(ctx: ApiContext, ci: String) -> Result<Option<GetRegisterDTO>, DbErr> {
        let register = register::Entity::find()
            .filter(register::Column::Ci.eq(ci))
            .find_also_related(organism::Entity)
            .find_also_related(division::Entity)
            .one(&ctx.db)
            .await?;

        let register = register.map(GetRegisterDTO::from);

        Ok(register)
    }

    pub async fn find(
        ctx: ApiContext,
        filter: CommonQueryFilterDTO,
    ) -> Result<Vec<GetRegisterDTO>, DbErr> {
        let mut query = register::Entity::find().filter_by_claims(ctx.clone().claims.unwrap());

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
            .filter(register::Column::BaseId.eq(ctx.claims.unwrap().user.base.id))
            .limit(pagination.limit)
            .offset(pagination.offset)
            .find_also_related(organism::Entity)
            .find_also_related(division::Entity)
            .all(&ctx.db)
            .await?;

        Ok(registers.into_iter().map(GetRegisterDTO::from).collect())
    }

    pub async fn get_pagination(
        ctx: ApiContext,
        filter: CommonQueryFilterDTO,
    ) -> Result<PaginationDTO, DbErr> {
        let mut query = register::Entity::find().filter_by_claims(ctx.claims.unwrap());

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

        let paginator = query.paginate(&ctx.db, pagination.limit);

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

    pub async fn create(ctx: ApiContext, register: CreateRegisterDTO) -> Result<(), DbErr> {
        let register = register.into_active_model().stamp_user(ctx.claims.unwrap());

        register::Entity::insert(register).exec(&ctx.db).await?;

        Ok(())
    }

    pub async fn update_exit(
        ctx: ApiContext,
        register: UpdateRegisterExitDTO,
        register_id: i32,
    ) -> Result<(), DbErr> {
        let mut register = register.into_active_model();

        register.id = Set(register_id);
        register.exit_date = Set(Some(Local::now().naive_local()));

        register::Entity::update(register).exec(&ctx.db).await?;

        Ok(())
    }
}
