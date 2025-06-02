use sea_orm::{
    sqlx::{Pool, Postgres},
    ConnectOptions, Database, DatabaseConnection, DbErr, SqlxError,
};

pub async fn connect(db_url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(db_url.to_owned());

    opt.sqlx_logging(true);

    let db = Database::connect(opt).await?;

    Ok(db)
}

pub async fn connect_pool(db_url: &str) -> Result<Pool<Postgres>, SqlxError> {
    let db: Pool<Postgres> = Pool::connect(db_url).await?;

    Ok(db)
}
