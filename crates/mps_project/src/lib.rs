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

pub mod cli;

pub(crate) mod config;
pub(crate) use config::*;

pub(crate) mod repository;
pub(crate) use repository::*;

pub(crate) mod grpc;

#[derive(thiserror::Error, Debug)]
pub enum MpsProjectError {
    #[error("failed parse cli arguments: {0}")]
    Clap(#[from] clap::Error),
    #[error("failed parse config: {0}")]
    Config(#[from] config::MpsProjectConfigError),
    #[error("failed load log: {0}")]
    Log(#[from] mps_log::MpsLogError),
    #[error("failed project repository: {0}")]
    Repository(#[from] repository::RepositoryError),
}

pub(crate) struct NewRepo {
    pub name: String,
    pub html_url: String,
}

#[async_trait::async_trait]
pub(crate) trait MpsScmUseCase {
    async fn create_repo(&self, name: &str) -> NewRepo;
}

#[async_trait::async_trait]
pub(crate) trait MpsScmGithubPort {
    async fn create_repo(&self, name: &str) -> NewRepo;
}

pub(crate) struct MpsScmService {
    github_port: Box<dyn MpsScmGithubPort + Send + Sync>,
}

impl MpsScmService {
    pub(crate) fn new(
        github_port: Box<dyn MpsScmGithubPort + Send + Sync>,
    ) -> Self {
        Self { github_port }
    }
}

#[async_trait::async_trait]
impl MpsScmUseCase for MpsScmService {
    async fn create_repo(&self, name: &str) -> NewRepo {
        self.github_port.create_repo(name).await
    }
}
