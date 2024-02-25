// Copyright (c) 2023 Murilo Ijanc' <mbsd@m0x.ru>
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

use derive_new::new;

use crate::{
    application::{
        error,
        port::{
            incoming::{
                CreateGithubRepositoryCommand, GithubRepositoryUseCase,
            },
            outgoing::{
                CreateGithubRepositoryPortCommand,
                GithubRepositoryPersistencePort, GithubRepositoryPort,
            },
        },
    },
    domain::GithubRepository,
};

///////////////////////////////////////////////////////////////////////////////
// GithubRepository
///////////////////////////////////////////////////////////////////////////////
#[derive(new)]
pub struct GithubRepositoryService {
    github_port: Box<dyn GithubRepositoryPort + Send + Sync>,
    persistence_port: Box<dyn GithubRepositoryPersistencePort + Send + Sync>,
}

#[async_trait::async_trait]
impl GithubRepositoryUseCase for GithubRepositoryService {
    async fn create(
        &self,
        command: CreateGithubRepositoryCommand,
    ) -> Result<GithubRepository, error::Error> {
        let github_command: CreateGithubRepositoryPortCommand =
            command.clone().into();
        let mut repository =
            self.github_port.create_repository(github_command).await?;
        repository.application_id = Some(command.application_id);
        let repository = repository.into();
        let repository =
            self.persistence_port.save_repository(repository).await?;
        Ok(repository)
    }
}
