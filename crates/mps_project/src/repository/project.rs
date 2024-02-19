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

use std::process::Command;

use chrono::{DateTime, Utc};
use fake::{Fake, Faker};
use sqlx::{Executor, PgPool};
use tracing::{error, info};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum ProjectRepositoryError {
    #[error("SQL error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Error parsing UUID: {0}")]
    UuidError(#[from] uuid::Error),
}

#[derive(Debug, sqlx::FromRow, fake::Dummy)]
pub struct Project {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct ProjectRepository {
    pool: PgPool,
}

impl ProjectRepository {
    pub fn new(pool: PgPool) -> Self {
        ProjectRepository { pool }
    }

    pub async fn create(
        &self,
        project: &Project,
    ) -> Result<Project, ProjectRepositoryError> {
        // let id = Uuid::new_v4();
        // let created_at = Utc::now();
        // let updated_at = created_at;

        // let project = Project {
        //     id,
        //     user_id,
        //     name: name.to_owned(),
        //     description: description.to_owned(),
        //     created_at,
        //     updated_at,
        // };

        Ok(sqlx::query_as!(
                Project,
            "INSERT INTO projects (id, user_id, name, description, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
            project.id, project.user_id, project.name, project.description, project.created_at, project.updated_at
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn read(
        &self,
        project_id: Uuid,
    ) -> Result<Project, ProjectRepositoryError> {
        Ok(sqlx::query_as!(
            Project,
            "SELECT * FROM projects WHERE id = $1",
            project_id
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn update(
        &self,
        project: &Project,
    ) -> Result<Project, ProjectRepositoryError> {
        sqlx::query_as!(
            Project,
            "UPDATE projects SET name = $1, description = $2, updated_at = $3 WHERE id = $4 RETURNING *",
            project.name, project.description, project.updated_at, project.id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(ProjectRepositoryError::from)
    }

    pub async fn delete(
        &self,
        project_id: Uuid,
    ) -> Result<(), ProjectRepositoryError> {
        sqlx::query!("DELETE FROM projects WHERE id = $1", project_id)
            .execute(&self.pool)
            .await
            .map_err(ProjectRepositoryError::from)?;

        Ok(())
    }

    pub async fn list(
        &self,
        page: i64,
        page_size: i64,
    ) -> Result<Vec<Project>, ProjectRepositoryError> {
        let offset = (page - 1) * page_size;

        sqlx::query_as!(
            Project,
            "SELECT * FROM projects ORDER BY created_at LIMIT $1 OFFSET $2",
            page_size,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(ProjectRepositoryError::from)
    }

    pub async fn seed(
        &self,
        count: usize,
    ) -> Result<(), ProjectRepositoryError> {
        for _ in 0..count {
            let p: Project = Faker.fake();

            self.create(&p).await?;
        }

        Ok(())
    }
}
