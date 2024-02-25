use super::git_repository::GitRepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error IO: {0}")]
    IoError(#[from] std::io::Error),

    #[error("project repository error: {0}")]
    ProjectError(#[from] GitRepositoryError),

    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),
}
