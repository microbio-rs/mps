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
use fake::{Fake, Faker};
use sqlx::{PgPool, Executor};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use thiserror::Error;

#[derive(Debug, Error)]
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

pub struct ProjectRepository {
    pool: PgPool,
}

impl ProjectRepository {
    pub fn new(pool: PgPool) -> Self {
        ProjectRepository { pool }
    }

    pub async fn create(&self, project: Project) -> Result<Project, ProjectRepositoryError> {
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

    pub async fn read(&self, project_id: Uuid) -> Result<Project, ProjectRepositoryError> {
        Ok(sqlx::query_as!(
            Project,
            "SELECT * FROM projects WHERE id = $1",
            project_id
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn update(&self, project_id: Uuid, name: &str, description: &str) -> Result<Project, ProjectRepositoryError> {
        let updated_at = Utc::now();

        sqlx::query_as!(
            Project,
            "UPDATE projects SET name = $1, description = $2, updated_at = $3 WHERE id = $4 RETURNING *",
            name, description, updated_at, project_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(ProjectRepositoryError::from)
    }

    pub async fn delete(&self, project_id: Uuid) -> Result<(), ProjectRepositoryError> {
        sqlx::query!("DELETE FROM projects WHERE id = $1", project_id)
            .execute(&self.pool)
            .await
            .map_err(ProjectRepositoryError::from)?;

        Ok(())
    }

    pub async fn list(&self, page: i64, page_size: i64) -> Result<Vec<Project>, ProjectRepositoryError> {
        let offset = (page - 1) * page_size;

        sqlx::query_as!(
            Project,
            "SELECT * FROM projects ORDER BY created_at LIMIT $1 OFFSET $2",
            page_size, offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(ProjectRepositoryError::from)
    }

    pub async fn seed(&self, count: usize) -> Result<(), ProjectRepositoryError> {
        for _ in 0..count {
            let p: Project = Faker.fake();

            self.create(p).await?;
        }

        Ok(())
    }
}

// // A função de migração para criar a tabela
// async fn create_projects_table(pool: &PgPool) -> Result<(), ProjectRepositoryError> {
//     sqlx::migrate!("./migrations")
//         .run(pool)
//         .await
//         .map_err(ProjectRepositoryError::from)?;

//     Ok(())
// }

// #[tokio::main]
// async fn main() {
//     let database_url = "postgresql://your_username:your_password@localhost/your_database";
//     let pool = PgPool::connect(&database_url).await.expect("Failed to connect to the database");

//     // Crie a tabela de projetos
//     create_projects_table(&pool).await.expect("Failed to create projects table");

//     // Crie a instância do repositório
//     let project_repository = ProjectRepository::new(pool);

//     // Exemplo de operações CRUD
//     let user_id = Uuid::new_v4();
//     let created_project = project_repository.create(user_id, "Project Name", "Project Description").await.expect("Failed to create project");
//     println!("Created project: {:?}", created_project);

//     let read_project = project_repository.read(created_project.id).await.expect("Failed to read project");
//     println!("Read project: {:?}", read_project);

//     let updated_project = project_repository.update(created_project.id, "Updated Name", "Updated Description").await.expect("Failed to update project");
//     println!("Updated project: {:?}", updated_project);

//     project_repository.delete(created_project.id).await.expect("Failed to delete project");
//     println!("Deleted project with id: {:?}", created_project.id);

//     // Exemplo de listagem com paginação
//     let projects = project_repository.list(1, 10).await.expect("Failed to list projects");
//     println!("Projects list: {:?}", projects);

//     // Exemplo de seed
//     project_repository.seed(5).await.expect("Failed to seed projects");
//     println!("Seeded 5 projects");
// }

