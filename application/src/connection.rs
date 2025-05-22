use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn connect(db_url: &str) -> Result<DatabaseConnection, DbErr> {
    Ok(Database::connect(db_url).await?)
}
