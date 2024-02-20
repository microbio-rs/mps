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
use aws_sdk_ecr::{
    config::{Credentials, Region},
    error::SdkError,
    operation::create_repository::CreateRepositoryError,
    Client,
};

pub mod ecr {
    tonic::include_proto!("ecr_proto");
}
use tonic::{transport::Server, Request, Response, Status};

use crate::ecr::ecr_server::{Ecr, EcrServer};
use crate::ecr::{
    create_repo_response::Result as CreateResult,
    CreateRepoRequest, CreateRepoResponse, RepoResponse,
};

#[derive(thiserror::Error, Debug)]
pub enum MpsCloudError {
    #[error("not found `repository uri` in sdk response")]
    RepositoryUriNotFound,

    #[error("not found `repository` in sdk response")]
    RepositoryNotFound,

    #[error("failed to create ecr repository: {0}")]
    AwsSdkError(#[from] SdkError<CreateRepositoryError>),
}

#[derive(Default)]
pub struct MpsEcrGrpcServer;

impl From<String> for RepoResponse {
    fn from(s: String) -> Self {
        RepoResponse { name: s }
    }
}

#[tonic::async_trait]
impl Ecr for MpsEcrGrpcServer {
    async fn create_repo(
        &self,
        request: Request<CreateRepoRequest>,
    ) -> Result<Response<CreateRepoResponse>, Status> {
        let name: String = request.into_inner().name;
        let resp = ecr_create_repository("", "", &name).await;
        if let Err(e) = resp {
            return Err(Status::invalid_argument(e.to_string()));
        }

        let resp = resp.unwrap();

        let response = CreateRepoResponse {
            result: CreateResult::Success.into(),
            repository: Some(resp.into()),
        };

        Ok(Response::new(response))
    }
}

pub async fn server() {
    let addr = "[::1]:50059".parse().unwrap();
    let ecr = MpsEcrGrpcServer::default();

    Server::builder()
        .add_service(EcrServer::new(ecr))
        .serve(addr)
        .await
        .unwrap();
}

pub async fn ecr_create_repository(
    access_key: &str,
    access_secret: &str,
    repository_name: &str,
) -> Result<String, MpsCloudError> {
    let credentials =
        Credentials::new(access_key, access_secret, None, None, "ecr");

    let config = aws_config::from_env()
        .credentials_provider(credentials)
        .region(Region::new("us-east-1"))
        .load()
        .await;

    let client = Client::new(&config);

    let resp = client
        .create_repository()
        .repository_name(repository_name)
        .send()
        .await?;

    match resp.repository() {
        Some(repository) => match repository.repository_uri() {
            Some(repository_uri) => Ok(repository_uri.to_string()),
            None => Err(MpsCloudError::RepositoryUriNotFound),
        },
        None => Err(MpsCloudError::RepositoryNotFound),
    }
}
