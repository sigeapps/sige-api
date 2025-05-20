use domain::{
    error::RepositoryError,
    repositories::commission_repository::{CommissionAggregate, CommissionAggregateRepository},
};

pub struct CreateCommissionUseCase {
    commission_repo: Box<dyn CommissionAggregateRepository>,
}

impl CreateCommissionUseCase {
    pub async fn execute(&self, commission: CommissionAggregate) -> Result<i32, RepositoryError> {
        let result = self.commission_repo.create_with_relations(commission).await;

        match result {
            Ok(id) => Ok(id),
            Err(e) => Err(e),
        }
    }
}
