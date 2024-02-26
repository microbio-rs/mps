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
    application::{error, port::incoming::CreateGithubRepositoryCommand},
    domain::{GithubCreateRepositoryResponse, GithubRepository},
};

///////////////////////////////////////////////////////////////////////////////
// GithubRepository
///////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, new)]
pub struct CloneGitRepositoryPortCommand {
    pub src: String,
    pub to: String,
}

#[async_trait::async_trait]
pub trait LocalGitPort {
    async fn clone_repository(
        &self,
        repository: CloneGitRepositoryPortCommand,
    ) -> Result<(), error::Error>;
}

#[derive(Debug, Clone, new)]
pub struct CreateGithubRepositoryPortCommand {
    pub name: String,
    pub private: bool,
}

impl From<CreateGithubRepositoryCommand> for CreateGithubRepositoryPortCommand {
    fn from(c: CreateGithubRepositoryCommand) -> Self {
        Self::new(c.name, false)
    }
}

#[async_trait::async_trait]
pub trait GithubRepositoryPort {
    async fn create_repository(
        &self,
        repository: CreateGithubRepositoryPortCommand,
    ) -> Result<GithubCreateRepositoryResponse, error::Error>;
}

#[async_trait::async_trait]
pub trait GithubRepositoryPersistencePort {
    async fn save_repository(
        &self,
        repository: GithubRepository,
    ) -> Result<GithubRepository, error::Error>;
}
