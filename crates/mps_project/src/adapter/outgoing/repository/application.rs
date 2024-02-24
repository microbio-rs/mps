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

use crate::domain::{Application, ApplicationId};

#[derive(Debug, thiserror::Error)]
pub enum ApplicationRepositoryError {
    #[error("SQL error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Error parsing UUID: {0}")]
    UuidError(#[from] uuid::Error),
}

#[derive(Debug, sqlx::FromRow, fake::Dummy, new)]
pub struct ApplicationEntity {
    pub id: Option<Uuid>,
    pub environment_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl ApplicationEntity {
    pub fn from_domain(p: Application) -> Self {
        Self::new(
            p.id.map(|id| id.to_uuid()),
            p.environment_id.to_uuid(),
            p.name,
            p.description,
            None,
            None,
        )
    }
}

impl From<ApplicationEntity> for Application {
    fn from(p: ApplicationEntity) -> Application {
        Application::new(
            p.id.map(|id| ApplicationId::new(id)),
            p.environment_id.into(),
            p.name,
            p.description,
        )
    }
}

#[derive(Clone)]
pub struct ApplicationRepository {
    pool: PgPool,
}

impl ApplicationRepository {
    pub fn new(pool: PgPool) -> Self {
        ApplicationRepository { pool }
    }

    pub async fn save(
        &self,
        application: ApplicationEntity,
    ) -> Result<ApplicationEntity, ApplicationRepositoryError> {
        // TODO: return error if ApplicationEntity has id
        debug!("Saving application {:?}", &application);
        let row = sqlx::query_as!(
            ApplicationEntity,
            "INSERT INTO applications
                (environment_id, name, description)
             VALUES
                ($1, $2, $3)
            RETURNING
                *",
            application.environment_id,
            application.name,
            application.description,
        )
        .fetch_one(&self.pool)
        .await?;

        debug!("application {} saved", &application.name);

        Ok(row)
    }

    // pub async fn read(
    //     &self,
    //     application_id: Uuid,
    // ) -> Result<ApplicationEntity, ApplicationRepositoryError> {
    //     Ok(sqlx::query_as!(
    //         ApplicationEntity,
    //         "SELECT * FROM applications WHERE id = $1",
    //         application_id
    //     )
    //     .fetch_one(&self.pool)
    //     .await?)
    // }

    // pub async fn update(
    //     &self,
    //     application: &ApplicationEntity,
    // ) -> Result<ApplicationEntity, ApplicationRepositoryError> {
    //     sqlx::query_as!(
    //         ApplicationEntity,
    //         "UPDATE applications SET name = $1, description = $2, updated_at = $3 WHERE id = $4 RETURNING *",
    //         application.name, application.description, application.updated_at, application.id
    //     )
    //     .fetch_one(&self.pool)
    //     .await
    //     .map_err(ApplicationRepositoryError::from)
    // }

    // pub async fn delete(
    //     &self,
    //     application_id: Uuid,
    // ) -> Result<(), ApplicationRepositoryError> {
    //     sqlx::query!("DELETE FROM applications WHERE id = $1", application_id)
    //         .execute(&self.pool)
    //         .await
    //         .map_err(ApplicationRepositoryError::from)?;

    //     Ok(())
    // }

    // pub async fn list(
    //     &self,
    //     page: i64,
    //     page_size: i64,
    // ) -> Result<Vec<ApplicationEntity>, ApplicationRepositoryError> {
    //     let offset = (page - 1) * page_size;

    //     sqlx::query_as!(
    //         ApplicationEntity,
    //         "SELECT * FROM applications ORDER BY created_at LIMIT $1 OFFSET $2",
    //         page_size,
    //         offset
    //     )
    //     .fetch_all(&self.pool)
    //     .await
    //     .map_err(ApplicationRepositoryError::from)
    // }

    // pub async fn seed(
    //     &self,
    //     count: usize,
    // ) -> Result<(), ApplicationRepositoryError> {
    //     for _ in 0..count {
    //         let mut p: ApplicationEntity = Faker.fake();
    //         // fix user id
    //         p.environment_id = uuid!("a97dfb95-2805-79bc-5e02-86083146a3a4");

    //         self.save(&p).await?;
    //     }

    //     Ok(())
    // }
}
