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

use sqlx::PgPool;

use super::{EnvironmentEntity, EnvironmentRepository};

use crate::application::port::outgoing::EnvironmentPersistencePort;
use crate::{application::error, domain::Environment};

pub struct EnvironmentPersistence {
    environment_repository: EnvironmentRepository,
}

impl EnvironmentPersistence {
    pub fn new(pool: PgPool) -> Self {
        Self { environment_repository: EnvironmentRepository::new(pool) }
    }
}

#[async_trait::async_trait]
impl EnvironmentPersistencePort for EnvironmentPersistence {
    async fn save_environment(
        &self,
        environment: Environment,
    ) -> Result<Environment, error::Error> {
        let environment_entity = EnvironmentEntity::from_domain(environment);
        let environment_entity = self
            .environment_repository
            .save(environment_entity)
            .await
            .map_err(|e| {
                error::Error::EnvironmentPersistenceError(e.to_string())
            })?;
        let environment = environment_entity.into();
        Ok(environment)
    }
}
