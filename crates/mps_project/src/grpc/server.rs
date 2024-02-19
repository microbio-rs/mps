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
use prost::Message;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Result};
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;

use super::proto::{
    create_project_response::Result as CreateResult, delete_project_response::Result as DeleteResult,
    read_project_response::Result as ReadResult, update_project_response::Result as UpdateResult,
    *,
};

#[derive(Debug, Serialize, Deserialize)]
struct Project {
    id: Uuid,
    user_id: Uuid,
    name: String,
    description: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}


impl From<Project> for ProjectResponse {
    fn from(project: Project) -> Self {
        ProjectResponse {
            id: project.id.to_string(),
            user_id: project.user_id.to_string(),
            name: project.name,
            description: project.description,
            created_at: project.created_at.to_rfc3339(),
            updated_at: project.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Default)]
pub struct CrudService {
    db_pool: PgPool,
}

#[tonic::async_trait]
impl crud_server::Crud for CrudService {
    async fn create_project(
        &self,
        request: Request<CreateProjectRequest>,
    ) -> Result<Response<CreateProjectResponse>, Status> {
        let project_req = request.into_inner();

        let project = Project {
            id: Uuid::new_v4(),
            user_id: Uuid::parse_str(&project_req.user_id).map_err(|_| Status::invalid_argument("Invalid user ID"))?,
            name: project_req.name,
            description: project_req.description,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        if let Err(e) = project.validate() {
            return Err(Status::invalid_argument(e.to_string()));
        }

        match Project::create(&self.db_pool, &project).await {
            Ok(_) => {
                let response = CreateProjectResponse {
                    result: Some(CreateResult::Success.into()),
                    project: Some(project.into()),
                };
                Ok(Response::new(response))
            }
            Err(_) => Err(Status::internal("Failed to create project")),
        }
    }

    async fn read_project(
        &self,
        request: Request<ReadProjectRequest>,
    ) -> Result<Response<ReadProjectResponse>, Status> {
        let project_id = Uuid::parse_str(&request.into_inner().id).map_err(|_| Status::invalid_argument("Invalid project ID"))?;
        match Project::read(&self.db_pool, project_id).await {
            Ok(project) => {
                let response = ReadProjectResponse {
                    result: Some(ReadResult::Success.into()),
                    project: Some(project.into()),
                };
                Ok(Response::new(response))
            }
            Err(_) => Err(Status::not_found("Project not found")),
        }
    }

    async fn update_project(
        &self,
        request: Request<UpdateProjectRequest>,
    ) -> Result<Response<UpdateProjectResponse>, Status> {
        let project_req = request.into_inner();
        let project_id = Uuid::parse_str(&project_req.id).map_err(|_| Status::invalid_argument("Invalid project ID"))?;

        let mut project = match Project::read(&self.db_pool, project_id).await {
            Ok(project) => project,
            Err(_) => return Err(Status::not_found("Project not found")),
        };

        project.user_id = Uuid::parse_str(&project_req.user_id).map_err(|_| Status::invalid_argument("Invalid user ID"))?;
        project.name = project_req.name;
        project.description = project_req.description;
        project.updated_at = Utc::now();

        if let Err(e) = project.validate() {
            return Err(Status::invalid_argument(e.to_string()));
        }

        match Project::update(&self.db_pool, &project).await {
            Ok(_) => {
                let response = UpdateProjectResponse {
                    result: Some(UpdateResult::Success.into()),
                };
                Ok(Response::new(response))
            }
            Err(_) => Err(Status::internal("Failed to update project")),
        }
    }

    async fn delete_project(
        &self,
        request: Request<DeleteProjectRequest>,
    ) -> Result<Response<DeleteProjectResponse>, Status> {
        let project_id = Uuid::parse_str(&request.into_inner().id).map_err(|_| Status::invalid_argument("Invalid project ID"))?;
        match Project::delete(&self.db_pool, project_id).await {
            Ok(_) => {
                let response = DeleteProjectResponse {
                    result: Some(DeleteResult::Success.into()),
                };
                Ok(Response::new(response))
            }
            Err(_) => Err(Status::internal("Failed to delete project")),
        }
    }
}

// use std::sync::Arc;

// use tonic::{transport::Server, Request, Response, Status};

// use super::scm::scm_server::{Scm, ScmServer};
// use super::scm::{CreateRepoRequest, CreateRepoResponse};

// #[derive(Clone)]
// pub(crate) struct MpsScmGrpcState {
//     create_repo_usecase: Arc<dyn crate::MpsScmUseCase + Send + Sync>,
// }

// impl MpsScmGrpcState {
//     pub fn new(
//         create_repo_usecase: Arc<dyn crate::MpsScmUseCase + Send + Sync>,
//     ) -> Self {
//         Self { create_repo_usecase }
//     }
// }

// #[derive(Clone)]
// struct MpsScmGrpcServer {
//     state: Arc<MpsScmGrpcState>,
// }

// impl MpsScmGrpcServer {
//     pub fn new(state: Arc<MpsScmGrpcState>) -> Self {
//         Self { state }
//     }
// }

// impl From<crate::NewRepo> for CreateRepoResponse {
//     fn from(r: crate::NewRepo) -> Self {
//         Self { name: r.name, html_url: r.html_url }
//     }
// }

// #[tonic::async_trait]
// impl Scm for MpsScmGrpcServer {
//     async fn create_repo(
//         &self,
//         request: Request<CreateRepoRequest>,
//     ) -> Result<Response<CreateRepoResponse>, Status> {
//         println!("Got a request: {:?}", &request);

//         let name: String = request.into_inner().name;
//         let resp = self.state.create_repo_usecase.create_repo(&name).await;

//         Ok(Response::new(resp.into()))
//     }
// }

// pub async fn server(state: Arc<MpsScmGrpcState>) {
//     let addr = "[::1]:50051".parse().unwrap();
//     let scm = MpsScmGrpcServer::new(state);

//     Server::builder()
//         .add_service(ScmServer::new(scm))
//         .serve(addr)
//         .await
//         .unwrap();
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::grpc::scm::{
//         scm_client::ScmClient, CreateRepoRequest, Provider,
//     };
//     use tokio::task;
//     use tonic::transport::Channel;

//     // Função de teste para criar um repositorio github
//     #[tokio::test]
//     async fn test_create_repo_github() {
//         // Inicie o servidor em uma nova tarefa
//         let _server_handle = task::spawn(server());

//         // Aguarde alguns milissegundos para garantir que o servidor está pronto
//         tokio::time::sleep(std::time::Duration::from_millis(100)).await;

//         // Conecte-se ao servidor
//         let channel =
//             Channel::from_static("http://[::1]:50051").connect().await.unwrap();
//         let mut client = ScmClient::new(channel);

//         // Crie uma solicitação de tarefa
//         let request = tonic::Request::new(CreateRepoRequest {
//             provider: Provider::Github.into(),
//             name: "aninha".into(),
//         });

//         // Chame o método do cliente para adicionar a tarefa
//         let response = client.create_repo(request).await.unwrap();

//         // Verifique se recebemos um ID de tarefa
//         let name = response.into_inner().name;
//         assert_eq!(name, String::from("aninha"));
//     }
// }
