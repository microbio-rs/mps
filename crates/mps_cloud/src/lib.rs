// Copyright (c) 2023 Murilo Ijanc' <mbsd@m0x.ru>
////
//// create ecr repository
////
//// Configuração do cliente AWS ECR
//// let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
//// // Note: requires the `behavior-version-latest` feature enabled
//// let client_config = aws_config::from_env().region(region_provider).load().await;
//// let client = Client::new(&client_config);
//// // Nome do repositório a ser criado
//// let repository_name = &new_repo.name;
//// // Criação do repositório
//// ecr::create_repository(&client, repository_name).await?;
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
pub async fn create_repository(
    client: &aws_sdk_ecr::Client,
    repository_name: &str,
) -> Result<(), EcrError> {
    // Chamada para criar o repositório
    match client
        .create_repository()
        .repository_name(repository_name)
        .send()
        .await
    {
        Ok(_) => {
            info!("Repositório {} criado com sucesso", repository_name);
            Ok(())
        }
        Err(err) => Err(EcrError::CreateRepositoryError(format!("{}", err))),
    }
}
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
