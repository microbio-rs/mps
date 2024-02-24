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
    create_application_response::Result as CreateResult,
    // delete_application_response::Result as DeleteResult,
    // read_application_response::Result as ReadResult,
    // update_application_response::Result as UpdateResult,
    *,
};

use crate::{
    application::port::incoming::{
        ApplicationUseCase, CreateApplicationCommand,
    },
    domain::{Application, EnvironmentId},
};

impl From<CreateApplicationRequest> for CreateApplicationCommand {
    fn from(c: CreateApplicationRequest) -> Self {
        CreateApplicationCommand::new(
            EnvironmentId::new(Uuid::parse_str(&c.environment_id).unwrap()),
            c.name,
            c.description,
        )
    }
}

impl From<Application> for ApplicationResponse {
    fn from(p: Application) -> Self {
        ApplicationResponse {
            id: p.id.unwrap().into(),
            environment_id: p.environment_id.into(),
            name: p.name,
            description: p.description,
        }
    }
}

#[derive(new)]
pub struct ApplicationCrudService {
    pub application_usecase: Arc<dyn ApplicationUseCase + Send + Sync>,
}

#[tonic::async_trait]
impl application_crud_server::ApplicationCrud for ApplicationCrudService {
    async fn create_application(
        &self,
        request: Request<CreateApplicationRequest>,
    ) -> Result<Response<CreateApplicationResponse>, Status> {
        let application_req = request.into_inner();
        let command = application_req.into();
        let application =
            self.application_usecase.create(command).await.map_err(|_| {
                Status::internal("Failed to create application")
            })?;

        // if let Err(e) = application.validate() {
        //     return Err(Status::invalid_argument(e.to_string()));
        // }

        let response = CreateApplicationResponse {
            result: CreateResult::Success.into(),
            application: Some(application.into()),
        };
        Ok(Response::new(response))
    }

    // async fn read_application(
    //     &self,
    //     request: Request<ReadApplicationRequest>,
    // ) -> Result<Response<ReadApplicationResponse>, Status> {
    //     let application_id = Uuid::parse_str(&request.into_inner().id)
    //         .map_err(|_| Status::invalid_argument("Invalid application ID"))?;
    //     match self.application_repository.read(application_id).await {
    //         Ok(application) => {
    //             let response = ReadApplicationResponse {
    //                 result: ReadResult::Success.into(),
    //                 application: Some(application.into()),
    //             };
    //             Ok(Response::new(response))
    //         }
    //         Err(_) => Err(Status::not_found("ApplicationEntity not found")),
    //     }
    // }

    // async fn update_application(
    //     &self,
    //     request: Request<UpdateApplicationRequest>,
    // ) -> Result<Response<UpdateApplicationResponse>, Status> {
    //     let application_req = request.into_inner();
    //     let application_id = Uuid::parse_str(&application_req.id)
    //         .map_err(|_| Status::invalid_argument("Invalid application ID"))?;

    //     let mut application = match self.application_repository.read(application_id).await {
    //         Ok(application) => application,
    //         Err(_) => return Err(Status::not_found("ApplicationEntity not found")),
    //     };

    //     application.environment_id = Uuid::parse_str(&application_req.environment_id)
    //         .map_err(|_| Status::invalid_argument("Invalid user ID"))?;
    //     application.name = application_req.name;
    //     application.description = application_req.description;
    //     application.updated_at = Utc::now();

    //     // if let Err(e) = application.validate() {
    //     //     return Err(Status::invalid_argument(e.to_string()));
    //     // }

    //     match self.application_repository.update(&application).await {
    //         Ok(_) => {
    //             let response = UpdateApplicationResponse {
    //                 result: UpdateResult::Success.into(),
    //             };
    //             Ok(Response::new(response))
    //         }
    //         Err(_) => Err(Status::internal("Failed to update application")),
    //     }
    // }

    // async fn delete_application(
    //     &self,
    //     request: Request<DeleteApplicationRequest>,
    // ) -> Result<Response<DeleteApplicationResponse>, Status> {
    //     let application_id = Uuid::parse_str(&request.into_inner().id)
    //         .map_err(|_| Status::invalid_argument("Invalid application ID"))?;
    //     match self.application_repository.delete(application_id).await {
    //         Ok(_) => {
    //             let response = DeleteApplicationResponse {
    //                 result: DeleteResult::Success.into(),
    //             };
    //             Ok(Response::new(response))
    //         }
    //         Err(_) => Err(Status::internal("Failed to delete application")),
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
