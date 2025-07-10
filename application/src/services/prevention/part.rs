use domain::entities::{
    brigade, charge, hierarchy, novelty, official, part, part_development, part_official,
    part_responsability,
};
use sea_orm::entity::prelude::*;
use sea_orm::*;

use crate::{
    api::ApiContext,
    auth::{FilterByClaims, HasBaseId, UserClaims, UserStamp},
    dtos::prevention::{
        official::GetOfficialDTO,
        part::{
            development_dto::GetPartDevelopmentDTO, dto::GetPartDTO,
            official_dto::GetPartOfficialDTO, responsability_dto::GetPartResponsabilityDTO,
            CreatePartAggregateDTO, GetPartAggregateDTO, GetPartSummaryDTO, UpdatePartCompleteDTO,
        },
    },
    impl_filter_by_claims,
};

impl HasBaseId for part::ActiveModel {
    fn set_base_id(mut self, id: i32) -> Self {
        self.base_id = Set(id);

        self
    }
}

impl_filter_by_claims!(part, BaseId);

#[derive(Clone, Debug)]
pub struct PartService {}

impl PartService {
    pub async fn create(ctx: ApiContext, mut dto: CreatePartAggregateDTO) -> Result<(), DbErr> {
        let transaction = ctx.db.begin().await?;

        let part_id = dto
            .part
            .into_active_model()
            .stamp_user(ctx.claims)
            .insert(&transaction)
            .await?
            .id;

        dto.responsability.part_id = part_id;

        dto.responsability
            .into_active_model()
            .insert(&transaction)
            .await?;

        async {
            for mut development in dto.developments {
                development.part_id = part_id;

                development.into_active_model().insert(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        async {
            for mut official in dto.officials {
                official.part_id = part_id;

                official.into_active_model().insert(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        transaction.commit().await?;

        Ok(())
    }

    pub async fn find_by_id(ctx: ApiContext, id: i32) -> Result<GetPartAggregateDTO, DbErr> {
        let part = part::Entity::find_by_id(id)
            .filter_by_claims(ctx.claims)
            .into_partial_model::<GetPartDTO>()
            .one(&ctx.db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "Part with id {} not found",
                id
            )))?;

        let officials = part_official::Entity::find()
            .filter(part_official::Column::PartId.eq(id))
            .select_only()
            .column(part_official::Column::Id)
            .column(part_official::Column::Type)
            .columns([
                official::Column::Id,
                official::Column::Ci,
                official::Column::LastName,
                official::Column::FirstName,
                official::Column::Phone,
                official::Column::Code,
            ])
            .columns([charge::Column::Id, charge::Column::Name])
            .columns([hierarchy::Column::Id, hierarchy::Column::Name])
            .columns([brigade::Column::Id, brigade::Column::Name])
            .join(JoinType::LeftJoin, part_official::Relation::Official.def())
            .join(JoinType::LeftJoin, official::Relation::Charge.def())
            .join(JoinType::LeftJoin, official::Relation::Hierarchy.def())
            .join(JoinType::LeftJoin, official::Relation::Brigade.def())
            .into_partial_model::<GetPartOfficialDTO>()
            .all(&ctx.db)
            .await?;

        // First, get the responsibility record with just the basic fields
        let resp_model = part_responsability::Entity::find()
            .filter(part_responsability::Column::PartId.eq(id))
            .one(&ctx.db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "Part responsabilities with id {} not found",
                id
            )))?;

        // Helper function to fetch an official by CI
        let fetch_official = |ci: &str| {
            official::Entity::find()
                .filter(official::Column::Ci.eq(ci))
                .select_only()
                .columns([
                    official::Column::Id,
                    official::Column::Ci,
                    official::Column::LastName,
                    official::Column::FirstName,
                    official::Column::Phone,
                    official::Column::Code,
                ])
                .columns([charge::Column::Id, charge::Column::Name])
                .columns([hierarchy::Column::Id, hierarchy::Column::Name])
                .columns([brigade::Column::Id, brigade::Column::Name])
                .join(JoinType::LeftJoin, official::Relation::Charge.def())
                .join(JoinType::LeftJoin, official::Relation::Hierarchy.def())
                .join(JoinType::LeftJoin, official::Relation::Brigade.def())
                .into_partial_model::<GetOfficialDTO>()
                .one(&ctx.db)
        };

        // Fetch all officials in parallel
        let (
            official_service_ci,
            official_receive_ci,
            official_block_ci,
            official_writer_ci,
            official_deliver_ci,
            official_receive_custody_ci,
            official_hr_ci,
            official_supervisor_ci,
            official_director_ci,
            official_dae_ci,
            official_die_ci,
        ) = futures::try_join!(
            fetch_official(&resp_model.official_service_ci),
            fetch_official(&resp_model.official_receive_ci),
            fetch_official(&resp_model.official_block_ci),
            fetch_official(&resp_model.official_writer_ci),
            fetch_official(&resp_model.official_deliver_ci),
            fetch_official(&resp_model.official_receive_custody_ci),
            fetch_official(&resp_model.official_hr_ci),
            fetch_official(&resp_model.official_supervisor_ci),
            fetch_official(&resp_model.official_director_ci),
            fetch_official(&resp_model.official_dae_ci),
            fetch_official(&resp_model.official_die_ci),
        )?;

        // Construct the responsibility DTO manually
        let responsabilities = GetPartResponsabilityDTO {
            id: resp_model.id,
            part_id: resp_model.part_id,
            official_service_ci: official_service_ci.ok_or(DbErr::RecordNotFound(
                "Official service CI not found".to_string(),
            ))?,
            official_receive_ci: official_receive_ci.ok_or(DbErr::RecordNotFound(
                "Official receive CI not found".to_string(),
            ))?,
            official_block_ci: official_block_ci.ok_or(DbErr::RecordNotFound(
                "Official block CI not found".to_string(),
            ))?,
            official_writer_ci: official_writer_ci.ok_or(DbErr::RecordNotFound(
                "Official writer CI not found".to_string(),
            ))?,
            official_deliver_ci: official_deliver_ci.ok_or(DbErr::RecordNotFound(
                "Official deliver CI not found".to_string(),
            ))?,
            official_receive_custody_ci: official_receive_custody_ci.ok_or(
                DbErr::RecordNotFound("Official receive custody CI not found".to_string()),
            )?,
            official_hr_ci: official_hr_ci.ok_or(DbErr::RecordNotFound(
                "Official HR CI not found".to_string(),
            ))?,
            official_supervisor_ci: official_supervisor_ci.ok_or(DbErr::RecordNotFound(
                "Official supervisor CI not found".to_string(),
            ))?,
            official_director_ci: official_director_ci.ok_or(DbErr::RecordNotFound(
                "Official director CI not found".to_string(),
            ))?,
            official_dae_ci: official_dae_ci.ok_or(DbErr::RecordNotFound(
                "Official DAE CI not found".to_string(),
            ))?,
            official_die_ci: official_die_ci.ok_or(DbErr::RecordNotFound(
                "Official DIE CI not found".to_string(),
            ))?,
        };

        println!("responsabilities good");

        let developments = part_development::Entity::find()
            .filter(part_development::Column::PartId.eq(id))
            .left_join(novelty::Entity)
            .into_partial_model::<GetPartDevelopmentDTO>()
            .all(&ctx.db)
            .await?;

        Ok(GetPartAggregateDTO {
            part,
            officials,
            responsabilities,
            developments,
        })
    }

    pub async fn find(ctx: ApiContext) -> Result<Vec<GetPartSummaryDTO>, DbErr> {
        part::Entity::find()
            .filter_by_claims(ctx.claims)
            .select_only()
            .column(part::Column::Id)
            .column(part::Column::Date)
            .column_as(part_official::Column::Id.count(), "officials_count")
            .column_as(part_development::Column::Id.count(), "developments_count")
            .left_join(part_official::Entity)
            .left_join(part_development::Entity)
            .group_by(part::Column::Id)
            .group_by(part::Column::Date)
            .order_by_desc(part::Column::Id)
            .into_model::<GetPartSummaryDTO>()
            .all(&ctx.db)
            .await
    }

    pub async fn edit_part(
        ctx: ApiContext,
        id: i32,
        mut part: UpdatePartCompleteDTO,
    ) -> Result<(), DbErr> {
        let transaction = ctx.db.begin().await?;

        part.part.id = id;

        part.part.into_active_model().save(&transaction).await?;

        part.responsabilities.part_id = id;

        part.responsabilities
            .into_active_model()
            .save(&transaction)
            .await?;

        async {
            for mut development in part.developments {
                development.part_id = id;

                development.into_active_model().save(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        transaction.commit().await?;

        Ok(())
    }
}
