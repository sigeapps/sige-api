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
