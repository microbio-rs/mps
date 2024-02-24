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
    application::error,
    domain::{Environment, EnvironmentMode, Project, ProjectId, UserId},
};

///////////////////////////////////////////////////////////////////////////////
// Environment
///////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, new)]
pub struct CreateEnvironmentCommand {
    pub project_id: ProjectId,
    pub name: String,
    pub description: Option<String>,
    pub mode: EnvironmentMode,
}

impl From<CreateEnvironmentCommand> for Environment {
    fn from(c: CreateEnvironmentCommand) -> Environment {
        Environment::new(None, c.project_id, c.name, c.description, c.mode)
    }
}

#[async_trait::async_trait]
pub trait EnvironmentUseCase {
    async fn create(
        &self,
        command: CreateEnvironmentCommand,
    ) -> Result<Environment, error::Error>;
}

///////////////////////////////////////////////////////////////////////////////
// Project
///////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, new)]
pub struct CreateProjectCommand {
    pub user_id: UserId,
    pub name: String,
    pub description: Option<String>,
}

impl From<CreateProjectCommand> for Project {
    fn from(c: CreateProjectCommand) -> Project {
        Project::new(None, c.user_id, c.name, c.description)
    }
}

#[async_trait::async_trait]
pub trait ProjectUseCase {
    async fn create(
        &self,
        command: CreateProjectCommand,
    ) -> Result<Project, error::Error>;
}
