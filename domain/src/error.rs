use tokio::task;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Standard IO error")]
    Io(#[from] std::io::Error),
    #[error("Task Join error")]
    TaskJoin(#[from] task::JoinError),
    #[error("Error querying users")]
    RepositoryError(#[from] RepositoryError),
}

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Not found register")]
    NotFound,
    #[error("Standard IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}
