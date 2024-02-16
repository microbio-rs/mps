// Copyright (c) 2023 Murilo Ijanc' <mbsd@m0x.ru>
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
