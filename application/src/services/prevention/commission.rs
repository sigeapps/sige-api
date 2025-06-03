use std::sync::Arc;

use domain::entities::{
    brigade, charge, commission, commission_official, commission_reason,
    commission_seized_transport, commission_transport, hierarchy, municipality, official,
    temporal_seclusion, transport,
};
use sea_orm::{prelude::Expr, *};

use crate::dtos::{
    prevention::{
        commission::{
            dto::GetCommissionDTO, reason_dto::GetCommissionReasonDTO,
            seclusion_dto::GetTemporalSeclusionDTO, CreateCommissionAggregateDTO,
            GetCommissionAggregateDTO, GetCommissionStatusAggregateDTO, GetCommissionSummaryDTO,
            UpdateCommissionExitDTO, UpdateCommissionStatusAggregateDTO,
        },
        lookup::GetBrigadeDTO,
        official::GetOfficialDTO,
        transport::GetTransportDTO,
    },
    CommonQueryFilterDTO, PaginationDTO,
};

#[derive(Debug, Clone)]
pub struct CommissionService {
    db: Arc<DatabaseConnection>,
}

#[derive(DeriveIden, Clone, Copy)]
pub struct AuthOfficial;

#[derive(DeriveIden, Clone, Copy)]
pub struct BossOfficial;

#[derive(DeriveIden, Clone, Copy)]
pub struct BossOfficialHierarchy;

#[derive(DeriveIden, Clone, Copy)]
pub struct AuthOfficialHierarchy;

// TODO: use transactions and async tasks
impl CommissionService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        CommissionService { db }
    }

    // TODO: MAKE MORE QUERY PERFORMANT THIS FN

    pub async fn find_by_id(self, id: i32) -> Result<GetCommissionAggregateDTO, DbErr> {
        // First, get the commission entity to get the foreign keys
        let commission_entity = commission::Entity::find_by_id(id)
            .one(&*self.db)
            .await?
            .ok_or(DbErr::RecordNotFound("Commission not found".to_string()))?;

        // Get brigade
        let brigade = brigade::Entity::find_by_id(commission_entity.brigade_id)
            .into_partial_model::<GetBrigadeDTO>()
            .one(&*self.db)
            .await?
            .ok_or(DbErr::RecordNotFound("Brigade not found".to_string()))?;

        // Get authorized official if exists
        let authorized_official = if let Some(auth_id) = commission_entity.authorized_official_id {
            official::Entity::find_by_id(auth_id)
                .left_join(hierarchy::Entity)
                .left_join(charge::Entity)
                .left_join(brigade::Entity)
                .into_partial_model::<GetOfficialDTO>()
                .one(&*self.db)
                .await?
        } else {
            None
        };

        // Get boss if exists
        let boss = if let Some(boss_id) = commission_entity.boss_id {
            official::Entity::find_by_id(boss_id)
                .left_join(hierarchy::Entity)
                .left_join(charge::Entity)
                .left_join(brigade::Entity)
                .into_partial_model::<GetOfficialDTO>()
                .one(&*self.db)
                .await?
        } else {
            None
        };

        // Construct the commission DTO
        let commission = GetCommissionDTO {
            id: commission_entity.id,
            brigade,
            authorized_official,
            boss,
            entry_at: commission_entity.entry_at,
            exit_at: commission_entity.exit_at,
            status_at: commission_entity.status_at,
            observations: commission_entity.observations,
            created_at: commission_entity.created_at,
        };

        // Get reason with municipality in one query
        let reason = commission_reason::Entity::find()
            .filter(commission_reason::Column::CommissionId.eq(id))
            .left_join(municipality::Entity)
            .into_partial_model::<GetCommissionReasonDTO>()
            .one(&*self.db)
            .await?
            .ok_or(DbErr::RecordNotFound("Reason not found".to_string()))?;

        // Get all seclusions with their statuses in one query
        let seclusions = temporal_seclusion::Entity::find()
            .filter(temporal_seclusion::Column::CommissionId.eq(id))
            .join(
                JoinType::LeftJoin,
                temporal_seclusion::Relation::SeclusionStatuses.def(),
            )
            .into_partial_model::<GetTemporalSeclusionDTO>()
            .all(&*self.db)
            .await?;

        // Get all transports with their related data in one query
        let transports = commission_transport::Entity::find()
            .filter(commission_transport::Column::CommissionId.eq(id))
            .join(
                JoinType::LeftJoin,
                commission_transport::Relation::Transport.def(),
            )
            .join(JoinType::LeftJoin, transport::Relation::TransportType.def())
            .join(JoinType::LeftJoin, transport::Relation::Brand.def())
            .join(JoinType::LeftJoin, transport::Relation::VehicleModel.def())
            .join(
                JoinType::LeftJoin,
                transport::Relation::TransportStatuses.def(),
            )
            .into_partial_model::<GetTransportDTO>()
            .all(&*self.db)
            .await?;

        Ok(GetCommissionAggregateDTO {
            commission,
            reason,
            seclusions,
            transports,
        })
    }

    pub async fn find(
        self,
        filter: CommonQueryFilterDTO,
    ) -> Result<Vec<GetCommissionSummaryDTO>, DbErr> {
        let mut query = commission::Entity::find()
            .select_only()
            .select_column_as(commission::Column::Id, "id")
            .select_column_as(commission::Column::EntryAt, "entry_at")
            .select_column_as(commission::Column::ExitAt, "exit_at")
            .select_column_as(commission::Column::StatusAt, "status_at")
            .select_column_as(commission::Column::Observations, "observations")
            .select_column_as(brigade::Column::Name, "brigade")
            .select_column_as(commission_reason::Column::Zone, "zone")
            .select_column_as(commission_reason::Column::Name, "reason")
            .select_column_as(
                Expr::cust_with_exprs(
                    "CONCAT($1, ' ', $2)",
                    [
                        Expr::col((BossOfficial, official::Column::FirstName)).into_simple_expr(),
                        Expr::col((BossOfficial, official::Column::LastName)).into_simple_expr(),
                    ],
                ),
                "boss",
            )
            .select_column_as(
                Expr::col((BossOfficialHierarchy, hierarchy::Column::Name)),
                "boss_hierarchy",
            )
            .select_column_as(
                Expr::cust_with_exprs(
                    "CONCAT($1, ' ', $2)",
                    [
                        Expr::col((AuthOfficial, official::Column::FirstName)).into_simple_expr(),
                        Expr::col((AuthOfficial, official::Column::LastName)).into_simple_expr(),
                    ],
                ),
                "auth_official",
            )
            .select_column_as(
                Expr::col((AuthOfficialHierarchy, hierarchy::Column::Name)),
                "auth_official_hierarchy",
            )
            .select_column_as(commission_official::Column::Id.count(), "officials_count");

        query = query
            .join(
                JoinType::InnerJoin,
                commission::Relation::CommissionReason.def(),
            )
            .join(JoinType::InnerJoin, commission::Relation::Brigade.def())
            .join(
                JoinType::LeftJoin,
                commission::Relation::CommissionOfficial.def(),
            )
            .join_as(
                JoinType::LeftJoin,
                commission::Relation::Official1.def(),
                BossOfficial,
            )
            .join_as(
                JoinType::LeftJoin,
                official::Relation::Hierarchy.def().from_alias(BossOfficial),
                BossOfficialHierarchy,
            )
            .join_as(
                JoinType::LeftJoin,
                commission::Relation::Official2.def(),
                AuthOfficial,
            )
            .join_as(
                JoinType::LeftJoin,
                official::Relation::Hierarchy.def().from_alias(AuthOfficial),
                AuthOfficialHierarchy,
            );

        if let Some(search) = &filter.search {
            query = query.filter(commission::Column::Observations.contains(search));
        }

        if let Some(from_date) = &filter.from_date {
            query = query.filter(commission::Column::EntryAt.gte(*from_date));
        }

        if let Some(to_date) = &filter.to_date {
            query = query.filter(commission::Column::EntryAt.lte(*to_date));
        }

        let pagination = &filter.into_pagination();

        query = query
            .limit(pagination.limit)
            .offset(pagination.offset)
            .group_by(commission::Column::Id)
            .group_by(commission::Column::EntryAt)
            .group_by(commission::Column::ExitAt)
            .group_by(commission::Column::StatusAt)
            .group_by(commission::Column::Observations)
            .group_by(brigade::Column::Name)
            .group_by(commission_reason::Column::Zone)
            .group_by(commission_reason::Column::Name)
            .group_by(Expr::col((BossOfficial, official::Column::FirstName)))
            .group_by(Expr::col((BossOfficial, official::Column::LastName)))
            .group_by(Expr::col((BossOfficialHierarchy, hierarchy::Column::Name)))
            .group_by(Expr::col((AuthOfficial, official::Column::FirstName)))
            .group_by(Expr::col((AuthOfficial, official::Column::LastName)))
            .group_by(Expr::col((AuthOfficialHierarchy, hierarchy::Column::Name)));

        query
            .into_model::<GetCommissionSummaryDTO>()
            .all(&*self.db)
            .await
    }

    pub async fn get_pagination(
        &self,
        filter: CommonQueryFilterDTO,
    ) -> Result<PaginationDTO, DbErr> {
        let mut query = commission::Entity::find();

        if let Some(search) = &filter.search {
            query = query.filter(commission::Column::Observations.contains(search));
        }

        if let Some(from_date) = &filter.from_date {
            query = query.filter(commission::Column::EntryAt.gte(*from_date));
        }

        if let Some(to_date) = &filter.to_date {
            query = query.filter(commission::Column::EntryAt.lte(*to_date));
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

    pub async fn find_status_by_id(
        self,
        id: i32,
    ) -> Result<GetCommissionStatusAggregateDTO, DbErr> {
        let seclusions = temporal_seclusion::Entity::find()
            .filter(temporal_seclusion::Column::CommissionId.eq(id))
            .join(
                JoinType::LeftJoin,
                temporal_seclusion::Relation::SeclusionStatuses.def(),
            )
            .into_partial_model::<GetTemporalSeclusionDTO>()
            .all(&*self.db)
            .await?;

        // TODO: USE COMMISSION SEIZED ON THIS TABLE
        let transports = commission_transport::Entity::find()
            .filter(commission_transport::Column::CommissionId.eq(id))
            .join(
                JoinType::LeftJoin,
                commission_transport::Relation::Transport.def(),
            )
            .join(JoinType::LeftJoin, transport::Relation::TransportType.def())
            .join(JoinType::LeftJoin, transport::Relation::Brand.def())
            .join(JoinType::LeftJoin, transport::Relation::VehicleModel.def())
            .join(
                JoinType::LeftJoin,
                transport::Relation::TransportStatuses.def(),
            )
            .into_partial_model::<GetTransportDTO>()
            .all(&*self.db)
            .await?;

        Ok(GetCommissionStatusAggregateDTO {
            seclusions,
            transports,
        })
    }

    pub async fn create(self, mut commission: CreateCommissionAggregateDTO) -> Result<i32, DbErr> {
        commission.commission.boss_id = commission.officials.first().unwrap().official_id;

        let txn = self.db.begin().await?;

        let commission_id = commission
            .commission
            .into_active_model()
            .insert(&txn)
            .await?
            .id;

        commission.reason.commission_id = commission_id;

        commission.reason.into_active_model().insert(&txn).await?;

        async {
            for official in &mut commission.officials {
                official.commission_id = commission_id;

                official.into_active_model().insert(&txn).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        async {
            for mut transport in commission.transports {
                transport.commission_id = commission_id;

                transport.into_active_model().insert(&txn).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        txn.commit().await?;

        Ok(commission_id)
    }

    pub async fn update_exit(self, id: i32, mut dto: UpdateCommissionExitDTO) -> Result<(), DbErr> {
        let txn = self.db.begin().await?;

        dto.commission.id = id;
        dto.commission.into_active_model().update(&txn).await?;

        async {
            for transport in dto.transports {
                let transport_id = transport.into_active_model().insert(&txn).await?.id;

                commission_seized_transport::ActiveModel {
                    commission_id: Set(id),
                    transport_id: Set(transport_id),
                    ..Default::default()
                }
                .insert(&txn)
                .await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        async {
            for mut seclusion in dto.seclusions {
                seclusion.commission_id = id;
                seclusion.into_active_model().insert(&txn).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        txn.commit().await?;

        Ok(())
    }

    // TODO: USE COMMISSION SEIZED ON THIS TABLE AND UPDATE THE STATUS OF THE TRANSPORT AND SECLUSION
    pub async fn update_status(
        self,
        id: i32,
        mut dto: UpdateCommissionStatusAggregateDTO,
    ) -> Result<(), DbErr> {
        let txn = self.db.begin().await?;

        dto.commission.id = id;
        dto.commission.into_active_model().update(&txn).await?;

        async {
            for transport in dto.transports {
                println!("{}", transport.id);

                transport.into_active_model().update(&txn).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        async {
            for mut seclusion in dto.seclusions {
                seclusion.commission_id = id;

                seclusion.into_active_model().update(&txn).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        txn.commit().await?;

        Ok(())
    }
}
