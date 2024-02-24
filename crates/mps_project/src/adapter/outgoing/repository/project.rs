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

use chrono::{DateTime, Utc};
// use fake::{Fake, Faker};
use derive_new::new;
use sqlx::PgPool;
use tracing::{debug, error};
use uuid::Uuid;

use crate::domain::{Project, ProjectId};

#[derive(Debug, thiserror::Error)]
pub enum ProjectRepositoryError {
    #[error("SQL error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Error parsing UUID: {0}")]
    UuidError(#[from] uuid::Error),
}

#[derive(Debug, sqlx::FromRow, fake::Dummy, new)]
pub struct ProjectEntity {
    pub id: Option<Uuid>,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl ProjectEntity {
    pub fn from_domain(p: Project) -> Self {
        Self::new(
            p.id.map(|id| id.to_uuid()),
            p.user_id.to_uuid(),
            p.name,
            p.description,
            None,
            None,
        )
    }
}

impl From<ProjectEntity> for Project {
    fn from(p: ProjectEntity) -> Project {
        Project::new(
            p.id.map(|id| ProjectId::new(id)),
            p.user_id.into(),
            p.name,
            p.description,
        )
    }
}

#[derive(Clone)]
pub struct ProjectRepository {
    pool: PgPool,
}

impl ProjectRepository {
    pub fn new(pool: PgPool) -> Self {
        ProjectRepository { pool }
    }

    pub async fn save(
        &self,
        project: ProjectEntity,
    ) -> Result<ProjectEntity, ProjectRepositoryError> {
        // TODO: return error if ProjectEntity has id
        debug!("Saving project {:?}", &project);
        let row = sqlx::query_as!(
            ProjectEntity,
            "INSERT INTO projects
                (user_id, name, description)
             VALUES
                ($1, $2, $3)
            RETURNING
                *",
            project.user_id,
            project.name,
            project.description,
        )
        .fetch_one(&self.pool)
        .await?;

        debug!("project {} saved", &project.name);

        Ok(row)
    }

    // pub async fn read(
    //     &self,
    //     project_id: Uuid,
    // ) -> Result<ProjectEntity, ProjectRepositoryError> {
    //     Ok(sqlx::query_as!(
    //         ProjectEntity,
    //         "SELECT * FROM projects WHERE id = $1",
    //         project_id
    //     )
    //     .fetch_one(&self.pool)
    //     .await?)
    // }

    // pub async fn update(
    //     &self,
    //     project: &ProjectEntity,
    // ) -> Result<ProjectEntity, ProjectRepositoryError> {
    //     sqlx::query_as!(
    //         ProjectEntity,
    //         "UPDATE projects SET name = $1, description = $2, updated_at = $3 WHERE id = $4 RETURNING *",
    //         project.name, project.description, project.updated_at, project.id
    //     )
    //     .fetch_one(&self.pool)
    //     .await
    //     .map_err(ProjectRepositoryError::from)
    // }

    // pub async fn delete(
    //     &self,
    //     project_id: Uuid,
    // ) -> Result<(), ProjectRepositoryError> {
    //     sqlx::query!("DELETE FROM projects WHERE id = $1", project_id)
    //         .execute(&self.pool)
    //         .await
    //         .map_err(ProjectRepositoryError::from)?;

    //     Ok(())
    // }

    // pub async fn list(
    //     &self,
    //     page: i64,
    //     page_size: i64,
    // ) -> Result<Vec<ProjectEntity>, ProjectRepositoryError> {
    //     let offset = (page - 1) * page_size;

    //     sqlx::query_as!(
    //         ProjectEntity,
    //         "SELECT * FROM projects ORDER BY created_at LIMIT $1 OFFSET $2",
    //         page_size,
    //         offset
    //     )
    //     .fetch_all(&self.pool)
    //     .await
    //     .map_err(ProjectRepositoryError::from)
    // }

    // pub async fn seed(
    //     &self,
    //     count: usize,
    // ) -> Result<(), ProjectRepositoryError> {
    //     for _ in 0..count {
    //         let mut p: ProjectEntity = Faker.fake();
    //         // fix user id
    //         p.user_id = uuid!("a97dfb95-2805-79bc-5e02-86083146a3a4");

    //         self.save(&p).await?;
    //     }

    //     Ok(())
    // }
}
