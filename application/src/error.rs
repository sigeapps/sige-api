use sea_orm::DbErr;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("database error")]
    Database(#[from] DbErr),
}
