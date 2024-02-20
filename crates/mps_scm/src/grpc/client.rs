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

use serde::Deserialize;
use tonic::transport::Channel;

use super::scm::scm_client::ScmClient;
use super::scm::{CreateRepoRequest, CreateRepoResponse};
use crate::NewRepo;

#[derive(thiserror::Error, Debug)]
pub enum ScmGrpcClientError {
    #[error(transparent)]
    Transport(#[from] tonic::transport::Error),
}

pub struct ScmGrpcClient {
    client: ScmClient<Channel>,
}

#[derive(Debug, Deserialize)]
pub struct ScmGrpcClientConfig {
    pub host: String,
    pub port: u16,
}

impl ScmGrpcClientConfig {
    pub fn addr(&self) -> String {
        format!("{}:{}", &self.host, self.port)
    }
}

impl From<CreateRepoResponse> for NewRepo {
    fn from(r: CreateRepoResponse) -> Self {
        Self { name: r.name, html_url: r.html_url }
    }
}

impl ScmGrpcClient {
    pub async fn new(
        config: &ScmGrpcClientConfig,
    ) -> Result<Self, ScmGrpcClientError> {
        let addr = config.addr();
        println!("trying connect to {addr:?}");
        let client = ScmClient::connect(config.addr()).await?;
        Ok(Self { client })
    }

    pub async fn create_repo(
        &mut self,
        request: CreateRepoRequest,
    ) -> Result<crate::NewRepo, ScmGrpcClientError> {
        let response = self.client.create_repo(request).await.unwrap();
        let repo: crate::NewRepo = response.into_inner().into();
        Ok(repo)
    }
}
