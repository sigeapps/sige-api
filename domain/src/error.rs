use tokio::task;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("User not found")]
    NotFound,
    #[error("Standard IO error")]
    Io(#[from] std::io::Error),
    #[error("Task Join error")]
    TaskJoin(#[from] task::JoinError),
}

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Not found register")]
    NotFound,
    #[error("Standard IO error")]
    Io(#[from] std::io::Error),
    #[error("Database error")]
    Database(String),
    #[error("Unexpected error")]
    Unexpected(String),
}
