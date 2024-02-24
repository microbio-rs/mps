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

use std::str::FromStr;

use chrono::{DateTime, Utc};
use derive_new::new;
use sqlx::PgPool;
use tracing::{debug, error};
use uuid::Uuid;

use crate::domain::{Environment, EnvironmentId, EnvironmentMode};

#[derive(Debug, thiserror::Error)]
pub enum EnvironmentRepositoryError {
    #[error("SQL error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Error parsing UUID: {0}")]
    UuidError(#[from] uuid::Error),
}

#[derive(Debug, sqlx::FromRow, new)]
pub struct EnvironmentEntity {
    pub id: Option<Uuid>,
    pub project_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub mode: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl EnvironmentEntity {
    pub fn from_domain(p: Environment) -> Self {
        Self::new(
            p.id.map(|id| id.to_uuid()),
            p.project_id.to_uuid(),
            p.name,
            p.description,
            p.mode.to_string(),
            None,
            None,
        )
    }
}

impl From<EnvironmentEntity> for Environment {
    fn from(p: EnvironmentEntity) -> Environment {
        Environment::new(
            p.id.map(|id| EnvironmentId::new(id)),
            p.project_id.into(),
            p.name,
            p.description,
            EnvironmentMode::from_str(&p.mode).unwrap(),
        )
    }
}

#[derive(Clone, new)]
pub struct EnvironmentRepository {
    pool: PgPool,
}

impl EnvironmentRepository {
    pub async fn save(
        &self,
        environment: EnvironmentEntity,
    ) -> Result<EnvironmentEntity, EnvironmentRepositoryError> {
        debug!("Saving environment {:?}", &environment);
        let row = sqlx::query_as!(
            EnvironmentEntity,
            "INSERT INTO environments
                (project_id, name, description, mode)
             VALUES
                ($1, $2, $3, $4)
            RETURNING
                *",
            environment.project_id,
            environment.name,
            environment.description,
            environment.mode as _,
        )
        .fetch_one(&self.pool)
        .await?;

        debug!("environment {} saved", &environment.name);

        Ok(row)
    }

    // pub async fn read(
    //     &self,
    //     environment_id: Uuid,
    // ) -> Result<EnvironmentEntity, EnvironmentRepositoryError> {
    //     Ok(sqlx::query_as!(
    //         EnvironmentEntity,
    //         "SELECT * FROM environments WHERE id = $1",
    //         environment_id
    //     )
    //     .fetch_one(&self.pool)
    //     .await?)
    // }

    // pub async fn update(
    //     &self,
    //     environment: &EnvironmentEntity,
    // ) -> Result<EnvironmentEntity, EnvironmentRepositoryError> {
    //     sqlx::query_as!(
    //         EnvironmentEntity,
    //         "UPDATE environments SET name = $1, description = $2, updated_at = $3 WHERE id = $4 RETURNING *",
    //         environment.name, environment.description, environment.updated_at, environment.id
    //     )
    //     .fetch_one(&self.pool)
    //     .await
    //     .map_err(EnvironmentRepositoryError::from)
    // }

    // pub async fn delete(
    //     &self,
    //     environment_id: Uuid,
    // ) -> Result<(), EnvironmentRepositoryError> {
    //     sqlx::query!("DELETE FROM environments WHERE id = $1", environment_id)
    //         .execute(&self.pool)
    //         .await
    //         .map_err(EnvironmentRepositoryError::from)?;

    //     Ok(())
    // }

    // pub async fn list(
    //     &self,
    //     page: i64,
    //     page_size: i64,
    // ) -> Result<Vec<EnvironmentEntity>, EnvironmentRepositoryError> {
    //     let offset = (page - 1) * page_size;

    //     sqlx::query_as!(
    //         EnvironmentEntity,
    //         "SELECT * FROM environments ORDER BY created_at LIMIT $1 OFFSET $2",
    //         page_size,
    //         offset
    //     )
    //     .fetch_all(&self.pool)
    //     .await
    //     .map_err(EnvironmentRepositoryError::from)
    // }

    // pub async fn seed(
    //     &self,
    //     count: usize,
    // ) -> Result<(), EnvironmentRepositoryError> {
    //     for _ in 0..count {
    //         let mut p: EnvironmentEntity = Faker.fake();
    //         // fix user id
    //         p.user_id = uuid!("a97dfb95-2805-79bc-5e02-86083146a3a4");

    //         self.create(&p).await?;
    //     }

    //     Ok(())
    // }
}
