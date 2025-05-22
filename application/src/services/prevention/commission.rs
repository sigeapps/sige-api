use sea_orm::*;

use crate::{connection::connect, dtos::prevention::commission::CreateCommissionAggregateDTO};

#[derive(Debug, Clone)]
pub struct CommissionService {
    db: DatabaseConnection,
}

impl CommissionService {
    pub async fn new(db_url: &str) -> Result<Self, DbErr> {
        let db = connect(db_url).await?;

        Ok(CommissionService { db })
    }

    pub async fn create(self, mut commission: CreateCommissionAggregateDTO) -> Result<i32, DbErr> {
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

        let _tasks = commission.officials.into_iter().map(|mut official| async {
            official.commission_id = commission_id;

            official.into_active_model().insert(&self.db).await
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
}
