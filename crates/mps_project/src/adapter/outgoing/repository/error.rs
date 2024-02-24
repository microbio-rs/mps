use super::project::ProjectRepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    // #[error("repository errror : {0}")]
    // ApplicationError(#[from] ApplicationRepositoryError),

    // #[error("repository errror : {0}")]
    // EnvironmentRepositoryError(#[from] EnvironmentRepositoryError),
    #[error("Error IO: {0}")]
    IoError(#[from] std::io::Error),

    #[error("project repository error: {0}")]
    ProjectError(#[from] ProjectRepositoryError),

    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),
}
