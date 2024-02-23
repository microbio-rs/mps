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

use std::{process::Command, time::Duration};

use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};

pub mod project;
pub use project::*;

pub mod application;
pub use application::*;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("repository errror : {0}")]
    ApplicationError(#[from] ApplicationRepositoryError),

    #[error("Error IO: {0}")]
    IoError(#[from] std::io::Error),

    #[error("project repository error: {0}")]
    ProjectError(#[from] ProjectRepositoryError),

    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RepositoryConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub max_pool: u32,
    pub timeout: u64,
}

impl RepositoryConfig {
    pub async fn new_pool(&self) -> Result<PgPool, RepositoryError> {
        let options = PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(&self.password)
            .database(&self.password);

        Ok(PgPoolOptions::new()
            .idle_timeout(Duration::from_secs(self.timeout))
            .max_connections(self.max_pool)
            .connect_with(options)
            .await?)
    }
}

pub fn run_migrations(
    database_url: &str,
    migrations_dir: &str,
) -> Result<(), RepositoryError> {
    use tracing::{error, info};
    let status = Command::new("sqlx")
        .arg("migrate")
        .arg("run")
        .arg("--database-url")
        .arg(database_url)
        .arg("--source")
        .arg(migrations_dir)
        .status()?;

    if status.success() {
        info!("Migrações concluídas com sucesso!");
    } else {
        error!("Erro ao executar migrações!");
    }

    Ok(())
}
