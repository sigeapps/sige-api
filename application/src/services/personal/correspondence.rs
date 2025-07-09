use sea_orm::*;

use crate::{api::ApiContext, dtos::personal::correspondence::CreateCorrespondenceRequest};

#[derive(Debug, Clone)]
pub struct CorrespondenceService {}

impl CorrespondenceService {
    pub async fn create(ctx: ApiContext, dto: CreateCorrespondenceRequest) -> Result<i32, DbErr> {
        let transaction = ctx.db.begin().await?;

        let id = dto.dto.into_active_model().insert(&transaction).await?.id;

        async {
            for mut document in dto.documents {
                document.correspondence_id = id;

                document.into_active_model().insert(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        transaction.commit().await?;

        Ok(id)
    }
}
