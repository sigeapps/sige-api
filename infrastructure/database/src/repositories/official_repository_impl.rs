use async_trait::async_trait;
use domain::entities::{prelude::*, *};
use domain::error::RepositoryError;
use domain::repositories::official_repository::OfficialRepository;
use sea_orm::*;

#[derive(Debug, Clone)]
pub struct SeaOrmOfficialRepository {
    db: DatabaseConnection,
}

impl SeaOrmOfficialRepository {
    pub async fn new(database_url: &str) -> Result<Self, RepositoryError> {
        let db = Database::connect(database_url)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(Self { db })
    }
}

#[async_trait]
impl OfficialRepository for SeaOrmOfficialRepository {
    async fn find(
        &self,
        search: Option<String>,
        brigade_id: Option<i32>,
    ) -> Result<Vec<official::Model>, RepositoryError> {
        let mut query = Official::find().filter(if let Some(search) = search {
            Condition::all()
                .add(official::Column::FirstName.contains(&search))
                .add(official::Column::LastName.contains(&search))
                .add(official::Column::Ci.contains(&search))
                .add(official::Column::Phone.contains(&search))
        } else {
            Condition::all()
        });

        if let Some(brigade_id) = brigade_id {
            query = query.filter(official::Column::BrigadeId.eq(brigade_id));
        }

        let officials = query
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(officials)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<official::Model>, RepositoryError> {
        let official = Official::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(official)
    }

    async fn create(&self, official: official::ActiveModel) -> Result<(), RepositoryError> {
        Official::insert(official)
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn update(&self, official: official::ActiveModel) -> Result<(), RepositoryError> {
        Official::update(official)
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        Official::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }
}
