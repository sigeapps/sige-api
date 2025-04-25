use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn connect(url: &str) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(url).await?;

    Ok(db)
}
