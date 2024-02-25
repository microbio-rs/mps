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

pub mod config;
pub use config::*;

pub mod git_repository;
pub use git_repository::*;

pub mod git_repository_persistence;
pub use git_repository_persistence::*;

pub mod error;
pub use error::*;

pub fn run_migrations(
    database_url: &str,
    migrations_dir: &str,
) -> Result<(), Error> {
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
