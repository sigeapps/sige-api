#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error("Not found registered user")]
    NotFound,
    #[error("Standard error")]
    Io(#[from] std::io::Error),
    #[error("Database error")]
    Db(#[from] sea_orm::DbErr),
}
