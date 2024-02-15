use aws_sdk_ecr::Error;
use thiserror::Error;
use tracing::{info, instrument};

#[derive(Error, Debug)]
pub enum EcrError {
    #[error("Failed to create repository: {0}")]
    CreateRepositoryError(String),
    #[error("AWS SDK Error: {0}")]
    AwsSdkError(#[from] Error),
}


#[instrument(skip(client))]
pub async fn create_repository(client: &aws_sdk_ecr::Client, repository_name: &str) -> Result<(), EcrError> {

    // Chamada para criar o repositório
    match client.create_repository()
        .repository_name(repository_name)
        .send()
        .await {
        Ok(_) => {
            info!("Repositório {} criado com sucesso", repository_name);
            Ok(())
        },
        Err(err) => {
            Err(EcrError::CreateRepositoryError(format!("{}", err)))
        }
    }
}

