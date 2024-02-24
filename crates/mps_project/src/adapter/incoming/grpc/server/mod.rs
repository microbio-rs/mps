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
    tonic::include_proto!("project_proto");
    tonic::include_proto!("environment_proto");
    // tonic::include_proto!("application_proto");
}

use proto::{
    environment_crud_server::EnvironmentCrudServer,
    project_crud_server::ProjectCrudServer,
};

pub mod config;
pub use config::*;

pub mod environment;
pub mod error;
pub mod project;

use crate::{
    adapter::outgoing::repository::{
        EnvironmentPersistence, ProjectPersistence,
    },
    application::service::{EnvironmentService, ProjectService},
    config::MpsProjectConfig,
};

// pub async fn server(state: Arc<MpsScmGrpcState>) {
pub async fn server(conf: &MpsProjectConfig) -> Result<(), error::Error> {
    let pool = conf.database.new_pool().await.unwrap();

    let repository_persitence = ProjectPersistence::new(pool.clone());
    let project_service = ProjectService::new(Box::new(repository_persitence));

    let environment_persitence = EnvironmentPersistence::new(pool);
    let environment_service =
        EnvironmentService::new(Box::new(environment_persitence));

    let addr = conf.grpc_server.server_address()?;

    info!("Start grpc server on {addr}");

    let proj = project::CrudService::new(Arc::new(project_service));
    let env =
        environment::EnvironmentCrudService::new(Arc::new(environment_service));

    Server::builder()
        .add_service(ProjectCrudServer::new(proj))
        .add_service(EnvironmentCrudServer::new(env))
        .serve(addr)
        .await?;

    Ok(())
}
