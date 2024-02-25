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

use std::sync::Arc;

use tonic::transport::Server;
use tracing::info;

pub mod proto {
    tonic::include_proto!("git_proto");
}

use proto::git_repository_crud_server::GitRepositoryCrudServer;

pub mod config;
pub use config::*;

pub mod error;
pub mod git_repository;

use crate::{
    adapter::outgoing::{
        provider::GithubProvider, repository::GitRepositoryPersistence,
    },
    application::service::GithubRepositoryService,
    config::Config,
};

pub async fn server(conf: &Config) -> Result<(), error::Error> {
    let pool = conf.database.new_pool().await.unwrap();

    let github_provider = GithubProvider::new(conf.github.clone());
    let git_repository_persitence = GitRepositoryPersistence::new(pool.clone());
    let git_repository_service = GithubRepositoryService::new(
        Box::new(github_provider),
        Box::new(git_repository_persitence),
    );

    let addr = conf.grpc_server.server_address()?;

    info!("Start grpc server on {addr}");

    let proj = git_repository::GitRepositoryCrudService::new(Arc::new(
        git_repository_service,
    ));

    Server::builder()
        .add_service(GitRepositoryCrudServer::new(proj))
        .serve(addr)
        .await?;

    Ok(())
}
