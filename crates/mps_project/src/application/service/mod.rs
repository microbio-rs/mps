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

use crate::{
    application::{
        error,
        port::{
            incoming::{
                CreateEnvironmentCommand, CreateProjectCommand,
                EnvironmentUseCase, ProjectUseCase,
            },
            outgoing::{EnvironmentPersistencePort, ProjectPersistencePort},
        },
    },
    domain::{Environment, Project},
};

///////////////////////////////////////////////////////////////////////////////
// Environment
///////////////////////////////////////////////////////////////////////////////

pub struct EnvironmentService {
    persistence_port: Box<dyn EnvironmentPersistencePort + Send + Sync>,
}

impl EnvironmentService {
    pub fn new(
        persistence_port: Box<dyn EnvironmentPersistencePort + Send + Sync>,
    ) -> Self {
        Self { persistence_port }
    }
}

#[async_trait::async_trait]
impl EnvironmentUseCase for EnvironmentService {
    async fn create(
        &self,
        command: CreateEnvironmentCommand,
    ) -> Result<Environment, error::Error> {
        let environment: Environment = command.into();
        let environment =
            self.persistence_port.save_environment(environment).await?;
        Ok(environment)
    }
}

///////////////////////////////////////////////////////////////////////////////
// Project
///////////////////////////////////////////////////////////////////////////////
pub struct ProjectService {
    persistence_port: Box<dyn ProjectPersistencePort + Send + Sync>,
}

impl ProjectService {
    pub fn new(
        persistence_port: Box<dyn ProjectPersistencePort + Send + Sync>,
    ) -> Self {
        Self { persistence_port }
    }
}

#[async_trait::async_trait]
impl ProjectUseCase for ProjectService {
    async fn create(
        &self,
        command: CreateProjectCommand,
    ) -> Result<Project, error::Error> {
        let project: Project = command.into();
        let project = self.persistence_port.save_project(project).await?;
        Ok(project)
    }
}
