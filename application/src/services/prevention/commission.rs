use domain::entities::{brigade, commission, commission_official, commission_reason, commission_seized_transport, hierarchy, official, temporal_seclusion, transport};
use sea_orm::{prelude::Expr, *};

use crate::{
    connection::connect,
    dtos::prevention::{commission::{
        seclusion_dto::GetTemporalSeclusionDTO, CreateCommissionAggregateDTO, GetCommissionStatusAggregateDTO, GetCommissionSummaryDTO, UpdateCommissionExitDTO, UpdateCommissionStatusAggregateDTO
    }, transport::GetTransportDTO},
};

#[derive(Debug, Clone)]
pub struct CommissionService {
    db: DatabaseConnection,
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
    pub async fn new(db_url: &str) -> Result<Self, DbErr> {
        let db = connect(db_url).await?;

        Ok(CommissionService { db })
    }

    pub async fn find(self, search: Option<String>) -> Result<Vec<GetCommissionSummaryDTO>, DbErr> {
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
                    ]
                ),
                "boss"
            )
            .select_column_as(Expr::col((BossOfficialHierarchy, hierarchy::Column::Name)), "boss_hierarchy")
            .select_column_as(
                Expr::cust_with_exprs(
                    "CONCAT($1, ' ', $2)",
                    [
                        Expr::col((AuthOfficial, official::Column::FirstName)).into_simple_expr(),
                        Expr::col((AuthOfficial, official::Column::LastName)).into_simple_expr(),
                    ]
                ),
                "auth_official"
            )
            .select_column_as(Expr::col((AuthOfficialHierarchy, hierarchy::Column::Name)), "auth_official_hierarchy")
            .select_column_as(commission_official::Column::Id.count(), "officials_count");

        query = query
            .join(JoinType::InnerJoin, commission::Relation::CommissionReason.def())
            .join(JoinType::InnerJoin, commission::Relation::Brigade.def())
            .join(JoinType::LeftJoin, commission::Relation::CommissionOfficial.def())
            .join_as(JoinType::LeftJoin, commission::Relation::Official1.def(), BossOfficial)
            .join_as(
                JoinType::LeftJoin,
                official::Relation::Hierarchy.def().from_alias(BossOfficial),
                BossOfficialHierarchy,
            )
            .join_as(JoinType::LeftJoin, commission::Relation::Official2.def(), AuthOfficial)
            .join_as(
                JoinType::LeftJoin,
                official::Relation::Hierarchy.def().from_alias(AuthOfficial),
                AuthOfficialHierarchy,
            );

        // Add search filter if provided
        if let Some(search) = search {
            query = query.filter(commission::Column::Observations.contains(search));
        }

        // Group by all non-aggregated columns
        query = query
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

        query.into_model::<GetCommissionSummaryDTO>().all(&self.db).await
    }

    pub async fn find_status_by_id(self, id: i32) -> Result<GetCommissionStatusAggregateDTO, DbErr> {
        let seclusions  = temporal_seclusion::Entity::find()
            .filter(temporal_seclusion::Column::CommissionId.eq(id))
            .into_partial_model::<GetTemporalSeclusionDTO>()
            .all(&self.db)
            .await?;

        let transports = commission_seized_transport::Entity::find()
            .filter(commission_seized_transport::Column::CommissionId.eq(id))
            .join(JoinType::LeftJoin, transport::Relation::CommissionTransport.def())
            .into_partial_model::<GetTransportDTO>()
            .all(&self.db)
            .await?;

        Ok(GetCommissionStatusAggregateDTO { seclusions, transports })
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

        commission
            .reason
            .into_active_model()
            .insert(&txn)
            .await?;

        async {
            for official in &mut commission.officials {
                official.commission_id = commission_id;

                official.into_active_model().insert(&txn).await?;
            }Ok::<(), DbErr>(())
        }.await?;

        async {
            for mut transport in commission.transports {
                transport.commission_id = commission_id;

                transport.into_active_model().insert(&txn).await?;
            }
            Ok::<(), DbErr>(())
        }.await?;

        txn.commit().await?;

        Ok(commission_id)
    }

    pub async fn update_exit(self, id: i32, mut dto: UpdateCommissionExitDTO) -> Result<(), DbErr> {
        let txn = self.db.begin().await?;

        dto.commission.id = id;
        dto.commission.into_active_model().update(&txn).await?;

        async {
            for transport in dto.transports {
                let transport_id = transport
                    .into_active_model()
                    .insert(&txn)
                    .await?
                    .id;

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
