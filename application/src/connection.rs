use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

pub async fn connect(db_url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(db_url.to_owned());

    opt.sqlx_logging(false);

    let db = Database::connect(opt).await?;

    Ok(db)
}
