#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error("Not found registered user")]
    NotFound,
    #[error("Standard error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Database error: {0}")]
    Db(#[from] sea_orm::DbErr),
}
