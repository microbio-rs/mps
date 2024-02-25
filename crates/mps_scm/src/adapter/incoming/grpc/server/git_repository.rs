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

use std::sync::Arc;

use derive_new::new;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use super::proto::{
    create_git_repository_response::Result as CreateResult,
    // delete_git_repository_response::Result as DeleteResult,
    // read_git_repository_response::Result as ReadResult,
    // update_git_repository_response::Result as UpdateResult,
    *,
};

use crate::{
    application::port::incoming::{
        CreateGithubRepositoryCommand, GithubRepositoryUseCase,
    },
    domain::{ApplicationId, GithubRepository},
};

impl From<CreateGitRepositoryRequest> for CreateGithubRepositoryCommand {
    fn from(c: CreateGitRepositoryRequest) -> Self {
        CreateGithubRepositoryCommand::new(
            ApplicationId::new(Uuid::parse_str(&c.application_id).unwrap()),
            c.name,
        )
    }
}

impl From<GithubRepository> for GitRepositoryResponse {
    fn from(p: GithubRepository) -> Self {
        GitRepositoryResponse {
            id: p.id.unwrap().into(),
            application_id: p.application_id.into(),
            default_branch: p.default_branch,
            description: p.description,
            full_name: p.full_name,
            name: p.name,
            private: p.private,
            provider_id: p.provider_id,
            size: p.size,
            ssh_url: p.ssh_url,
            url: p.url,
        }
    }
}

#[derive(new)]
pub struct GitRepositoryCrudService {
    pub git_repository_usecase: Arc<dyn GithubRepositoryUseCase + Send + Sync>,
}

#[tonic::async_trait]
impl git_repository_crud_server::GitRepositoryCrud
    for GitRepositoryCrudService
{
    async fn create_git_repository(
        &self,
        request: Request<CreateGitRepositoryRequest>,
    ) -> Result<Response<CreateGitRepositoryResponse>, Status> {
        let git_repository_req = request.into_inner();
        let command = git_repository_req.into();
        let git_repository =
            self.git_repository_usecase.create(command).await.map_err(
                |_| Status::internal("Failed to create git_repository"),
            )?;

        // if let Err(e) = git_repository.validate() {
        //     return Err(Status::invalid_argument(e.to_string()));
        // }

        let response = CreateGitRepositoryResponse {
            result: CreateResult::Success.into(),
            repository: Some(git_repository.into()),
        };
        Ok(Response::new(response))
    }

    // async fn read_git_repository(
    //     &self,
    //     request: Request<ReadGitRepositoryRequest>,
    // ) -> Result<Response<ReadGitRepositoryResponse>, Status> {
    //     let git_repository_id = Uuid::parse_str(&request.into_inner().id)
    //         .map_err(|_| Status::invalid_argument("Invalid git_repository ID"))?;
    //     match self.git_repository_repository.read(git_repository_id).await {
    //         Ok(git_repository) => {
    //             let response = ReadGitRepositoryResponse {
    //                 result: ReadResult::Success.into(),
    //                 git_repository: Some(git_repository.into()),
    //             };
    //             Ok(Response::new(response))
    //         }
    //         Err(_) => Err(Status::not_found("GitRepositoryEntity not found")),
    //     }
    // }

    // async fn update_git_repository(
    //     &self,
    //     request: Request<UpdateGitRepositoryRequest>,
    // ) -> Result<Response<UpdateGitRepositoryResponse>, Status> {
    //     let git_repository_req = request.into_inner();
    //     let git_repository_id = Uuid::parse_str(&git_repository_req.id)
    //         .map_err(|_| Status::invalid_argument("Invalid git_repository ID"))?;

    //     let mut git_repository = match self.git_repository_repository.read(git_repository_id).await {
    //         Ok(git_repository) => git_repository,
    //         Err(_) => return Err(Status::not_found("GitRepositoryEntity not found")),
    //     };

    //     git_repository.user_id = Uuid::parse_str(&git_repository_req.user_id)
    //         .map_err(|_| Status::invalid_argument("Invalid user ID"))?;
    //     git_repository.name = git_repository_req.name;
    //     git_repository.description = git_repository_req.description;
    //     git_repository.updated_at = Utc::now();

    //     // if let Err(e) = git_repository.validate() {
    //     //     return Err(Status::invalid_argument(e.to_string()));
    //     // }

    //     match self.git_repository_repository.update(&git_repository).await {
    //         Ok(_) => {
    //             let response = UpdateGitRepositoryResponse {
    //                 result: UpdateResult::Success.into(),
    //             };
    //             Ok(Response::new(response))
    //         }
    //         Err(_) => Err(Status::internal("Failed to update git_repository")),
    //     }
    // }

    // async fn delete_git_repository(
    //     &self,
    //     request: Request<DeleteGitRepositoryRequest>,
    // ) -> Result<Response<DeleteGitRepositoryResponse>, Status> {
    //     let git_repository_id = Uuid::parse_str(&request.into_inner().id)
    //         .map_err(|_| Status::invalid_argument("Invalid git_repository ID"))?;
    //     match self.git_repository_repository.delete(git_repository_id).await {
    //         Ok(_) => {
    //             let response = DeleteGitRepositoryResponse {
    //                 result: DeleteResult::Success.into(),
    //             };
    //             Ok(Response::new(response))
    //         }
    //         Err(_) => Err(Status::internal("Failed to delete git_repository")),
    //     }
    // }
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
