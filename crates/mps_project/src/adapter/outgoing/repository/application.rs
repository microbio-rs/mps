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
use fake::{Fake, Faker};
use sqlx::PgPool;
use tracing::error;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationRepositoryError {
    #[error("SQL error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Error parsing UUID: {0}")]
    UuidError(#[from] uuid::Error),
}

#[derive(Debug, sqlx::FromRow, fake::Dummy)]
pub struct Application {
    pub id: Uuid,
    pub user_id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct ApplicationRepository {
    pool: PgPool,
}

impl ApplicationRepository {
    pub fn new(pool: PgPool) -> Self {
        ApplicationRepository { pool }
    }

    pub async fn create(
        &self,
        project: Application,
    ) -> Result<Application, ApplicationRepositoryError> {
        // let id = Uuid::new_v4();
        // let created_at = Utc::now();
        // let updated_at = created_at;

        // let project = Application {
        //     id,
        //     user_id,
        //     name: name.to_owned(),
        //     description: description.to_owned(),
        //     created_at,
        //     updated_at,
        // };

        Ok(sqlx::query_as!(
                Application,
            "INSERT INTO applications (id, user_id, name, description, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
            project.id, project.user_id, project.name, project.description, project.created_at, project.updated_at
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn read(
        &self,
        project_id: Uuid,
    ) -> Result<Application, ApplicationRepositoryError> {
        Ok(sqlx::query_as!(
            Application,
            "SELECT * FROM applications WHERE id = $1",
            project_id
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn update(
        &self,
        project_id: Uuid,
        name: &str,
        description: &str,
    ) -> Result<Application, ApplicationRepositoryError> {
        let updated_at = Utc::now();

        sqlx::query_as!(
            Application,
            "UPDATE applications SET name = $1, description = $2, updated_at = $3 WHERE id = $4 RETURNING *",
            name, description, updated_at, project_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(ApplicationRepositoryError::from)
    }

    pub async fn delete(
        &self,
        project_id: Uuid,
    ) -> Result<(), ApplicationRepositoryError> {
        sqlx::query!("DELETE FROM applications WHERE id = $1", project_id)
            .execute(&self.pool)
            .await
            .map_err(ApplicationRepositoryError::from)?;

        Ok(())
    }

    pub async fn list(
        &self,
        page: i64,
        page_size: i64,
    ) -> Result<Vec<Application>, ApplicationRepositoryError> {
        let offset = (page - 1) * page_size;

        sqlx::query_as!(
            Application,
            "SELECT * FROM applications ORDER BY created_at LIMIT $1 OFFSET $2",
            page_size,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(ApplicationRepositoryError::from)
    }

    pub async fn seed(
        &self,
        count: usize,
    ) -> Result<(), ApplicationRepositoryError> {
        for _ in 0..count {
            let p: Application = Faker.fake();

            self.create(p).await?;
        }

        Ok(())
    }
}
