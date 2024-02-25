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

use crate::domain::{GithubRepository, GithubRepositoryId};

#[derive(Debug, thiserror::Error)]
pub enum GitRepositoryError {
    #[error("SQL error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Error parsing UUID: {0}")]
    UuidError(#[from] uuid::Error),
}

#[derive(Debug, sqlx::FromRow, fake::Dummy, new)]
pub struct GitRepositoryEntity {
    pub id: Option<Uuid>,
    pub application_id: Uuid,
    pub default_branch: String,
    pub description: Option<String>,
    pub full_name: String,
    pub name: String,
    pub private: bool,
    pub provider_id: i64,
    pub size: i64,
    pub ssh_url: String,
    pub url: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl GitRepositoryEntity {
    pub fn from_domain(p: GithubRepository) -> Self {
        Self::new(
            p.id.map(|id| id.to_uuid()),
            p.application_id.to_uuid(),
            p.default_branch,
            p.description,
            p.full_name,
            p.name,
            p.private,
            p.provider_id,
            p.size,
            p.ssh_url,
            p.url,
            None,
            None,
        )
    }
}

impl From<GitRepositoryEntity> for GithubRepository {
    fn from(p: GitRepositoryEntity) -> GithubRepository {
        GithubRepository::new(
            p.id.map(|id| GithubRepositoryId::new(id)),
            p.application_id.into(),
            p.default_branch,
            p.description,
            p.full_name,
            p.name,
            p.private,
            p.provider_id,
            p.size,
            p.ssh_url,
            p.url,
        )
    }
}

#[derive(Clone, new)]
pub struct GitRepository {
    pool: PgPool,
}

impl GitRepository {
    pub async fn save(
        &self,
        repository: GitRepositoryEntity,
    ) -> Result<GitRepositoryEntity, GitRepositoryError> {
        // TODO: return error if ProjectEntity has id
        debug!("git repository {:?}", &repository);
        let row = sqlx::query_as!(
            GitRepositoryEntity,
            "INSERT INTO git_repositories 
                (application_id, default_branch, description, full_name, name,
                private, provider_id, size, ssh_url, url)
             VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING
                *",
            repository.application_id,
            repository.default_branch,
            repository.description,
            repository.full_name,
            repository.name,
            repository.private,
            repository.provider_id,
            repository.size,
            repository.ssh_url,
            repository.url,
        )
        .fetch_one(&self.pool)
        .await?;

        debug!("git repository {} saved", &repository.name);

        Ok(row)
    }

    // pub async fn read(
    //     &self,
    //     project_id: Uuid,
    // ) -> Result<GitRepositoryEntity, GitRepositoryRepositoryError> {
    //     Ok(sqlx::query_as!(
    //         GitRepositoryEntity,
    //         "SELECT * FROM projects WHERE id = $1",
    //         project_id
    //     )
    //     .fetch_one(&self.pool)
    //     .await?)
    // }

    // pub async fn update(
    //     &self,
    //     project: &GitRepositoryEntity,
    // ) -> Result<GitRepositoryEntity, GitRepositoryRepositoryError> {
    //     sqlx::query_as!(
    //         GitRepositoryEntity,
    //         "UPDATE projects SET name = $1, description = $2, updated_at = $3 WHERE id = $4 RETURNING *",
    //         project.name, project.description, project.updated_at, project.id
    //     )
    //     .fetch_one(&self.pool)
    //     .await
    //     .map_err(GitRepositoryRepositoryError::from)
    // }

    // pub async fn delete(
    //     &self,
    //     project_id: Uuid,
    // ) -> Result<(), GitRepositoryRepositoryError> {
    //     sqlx::query!("DELETE FROM projects WHERE id = $1", project_id)
    //         .execute(&self.pool)
    //         .await
    //         .map_err(GitRepositoryRepositoryError::from)?;

    //     Ok(())
    // }

    // pub async fn list(
    //     &self,
    //     page: i64,
    //     page_size: i64,
    // ) -> Result<Vec<GitRepositoryEntity>, GitRepositoryRepositoryError> {
    //     let offset = (page - 1) * page_size;

    //     sqlx::query_as!(
    //         GitRepositoryEntity,
    //         "SELECT * FROM projects ORDER BY created_at LIMIT $1 OFFSET $2",
    //         page_size,
    //         offset
    //     )
    //     .fetch_all(&self.pool)
    //     .await
    //     .map_err(GitRepositoryRepositoryError::from)
    // }

    // pub async fn seed(
    //     &self,
    //     count: usize,
    // ) -> Result<(), GitRepositoryRepositoryError> {
    //     for _ in 0..count {
    //         let mut p: GitRepositoryEntity = Faker.fake();
    //         // fix user id
    //         p.user_id = uuid!("a97dfb95-2805-79bc-5e02-86083146a3a4");

    //         self.save(&p).await?;
    //     }

    //     Ok(())
    // }
}
