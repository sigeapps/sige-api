use domain::entities::{brigade, commission, commission_official, commission_reason, commission_seized_transport, hierarchy, official};
use sea_orm::{prelude::Expr, sea_query::Alias, *};

use crate::{
    connection::connect,
    dtos::prevention::commission::{
        CreateCommissionAggregateDTO, GetCommissionSummaryDTO, UpdateCommissionExitDTO, UpdateCommissionStatusAggregateDTO
    },
};

#[derive(Debug, Clone)]
pub struct CommissionService {
    db: DatabaseConnection,
}

// TODO: use transactions and async tasks
impl CommissionService {
    pub async fn new(db_url: &str) -> Result<Self, DbErr> {
        let db = connect(db_url).await?;

        Ok(CommissionService { db })
    }

    pub async fn find(self, search: Option<String>) -> Result<Vec<GetCommissionSummaryDTO>, DbErr> {
        let mut query = commission::Entity::find().select_only()
        .select_column_as(commission::Column::Id, "id")
        .select_column_as(commission::Column::EntryAt, "entry_at")
        .select_column_as(commission::Column::ExitAt, "exit_at")
        .select_column_as(commission::Column::StatusAt, "status_at")
        .select_column_as(brigade::Column::Name, "brigade")
        .select_column_as(commission_reason::Column::Zone, "zone")
        .select_column_as(
            Expr::cust_with_exprs(
                "CONCAT(?, ' ', ?)",
                [
                    Expr::col((Alias::new("boss"), official::Column::FirstName)).into_simple_expr(),
                    Expr::col((Alias::new("boss"), official::Column::LastName)).into_simple_expr(),
                ]
            ),
            "boss"
        )
        .select_column_as(
            Expr::cust_with_exprs(
                "CONCAT(?, ' ', ?)",
                [
                    Expr::col((Alias::new("auth"), official::Column::FirstName)).into_simple_expr(),
                    Expr::col((Alias::new("auth"), official::Column::LastName)).into_simple_expr(),
                ]
            ),
            "auth_official"
        )
        .select_column_as(hierarchy::Column::Name, "boss_hierarchy")
        .select_column_as(commission_official::Column::Id.count(), "officials_count")
        .select_column_as(hierarchy::Column::Name, "auth_hierarchy")
        .select_column_as(commission::Column::Observations, "observations")
            .join(
                JoinType::InnerJoin,
                commission_reason::Relation::Commission.def(),
            )
            .join(
                JoinType::InnerJoin,
                brigade::Relation::Official.def(),
            )
            .join(JoinType::InnerJoin, hierarchy::Relation::Official.def())
            .join_as(
                JoinType::InnerJoin,
                commission::Relation::Official1.def(),
                Alias::new("boss"),
            )
            .join_as(
                JoinType::InnerJoin,
                commission::Relation::Official2.def(),
                Alias::new("auth"),
            );

    if let Some(search) = search {
        query = query.filter(commission::Column::Observations.contains(search));
    };

        query.into_model::<GetCommissionSummaryDTO>().all(&self.db).await
    }

    pub async fn create(self, mut commission: CreateCommissionAggregateDTO) -> Result<i32, DbErr> {
        commission.commission.boss_id = commission.officials.first().unwrap().official_id;

        let commission_id = commission
            .commission
            .into_active_model()
            .insert(&self.db)
            .await?
            .id;

        commission.reason.commission_id = commission_id;

        commission
            .reason
            .into_active_model()
            .insert(&self.db)
            .await?;

        let _tasks = commission.officials.into_iter().map(|mut official| {
            let db = self.db.clone();

            {
                async move {
            official.commission_id = commission_id;

            official.into_active_model().insert(&db).await
            }
        }
        });

        let _tasks = commission
            .transports
            .into_iter()
            .map(|mut transport| async {
                transport.commission_id = commission_id;

                transport.into_active_model().insert(&self.db).await
            });

        Ok(commission_id)
    }

    pub async fn update_exit(self, id: i32, mut dto: UpdateCommissionExitDTO) -> Result<(), DbErr> {
        dto.commission.id = id;

        dto.commission.into_active_model().update(&self.db).await?;

        let _tasks = dto.transports.into_iter().map(|transport| async {
            let transport_id = transport
                .into_active_model()
                .insert(&self.db)
                .await
                .unwrap()
                .id;

            commission_seized_transport::ActiveModel {
                commission_id: Set(id),
                transport_id: Set(transport_id),
                ..Default::default()
            }
            .insert(&self.db)
            .await
        });

        let _tasks = dto.seclusions.into_iter().map(|mut seclusion| async {
            seclusion.commission_id = id;

            seclusion.into_active_model().insert(&self.db).await
        });

        Ok(())
    }

    pub async fn update_status(
        self,
        id: i32,
        mut dto: UpdateCommissionStatusAggregateDTO,
    ) -> Result<(), DbErr> {
        dto.commission.id = id;

        dto.commission.into_active_model().update(&self.db).await?;

        let _tasks = dto
            .transports
            .into_iter()
            .map(|transport| async { transport.into_active_model().update(&self.db).await });

        let _tasks = dto.seclusions.into_iter().map(|mut seclusion| async {
            seclusion.commission_id = id;

            seclusion.into_active_model().update(&self.db).await
        });

        Ok(())
    }
}
