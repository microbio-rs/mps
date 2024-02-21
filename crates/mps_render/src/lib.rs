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
use std::io;
use std::io::Write;
use std::path::Path;

use tera::{Context, Tera};

#[derive(thiserror::Error, Debug)]
pub enum MpsRenderError {
    #[error("Template error: {0}")]
    Tera(#[from] tera::Error),

    #[error("Io error: {0}")]
    Io(#[from] io::Error),
}

pub fn render<P: AsRef<Path>>(
    origem: P,
    destino: P,
    context: Context,
) -> Result<(), MpsRenderError> {
    let mut tera = Tera::default();
    // TODO: improve this, this gets bad when the directory has numerous files
    mount_tera(&mut tera, origem.as_ref())?;
    copiar_arquivos(&tera, &context, origem.as_ref(), destino.as_ref())?;

    Ok(())
}

fn mount_tera(tera: &mut Tera, origem: &Path) -> Result<(), MpsRenderError> {
    if origem.is_dir() {
        if !origem.to_str().unwrap().contains(".git") {
            for entrada in std::fs::read_dir(origem)? {
                let entrada = entrada?;
                let origem_arquivo = entrada.path();

                mount_tera(tera, &origem_arquivo)?;
            }
        }
    } else {
        tera.add_template_file(origem.to_str().unwrap(), None)?;
    }

    Ok(())
}

fn copiar_arquivos(
    tera: &Tera,
    context: &Context,
    origem: &Path,
    destino: &Path,
) -> Result<(), MpsRenderError> {
    if origem.is_dir() {
        if !origem.to_str().unwrap().contains(".git") {
            if !destino.exists() {
                std::fs::create_dir_all(destino)?;
            }

            for entrada in std::fs::read_dir(origem)? {
                let entrada = entrada?;
                let origem_arquivo = entrada.path();
                let destino_arquivo = destino.join(entrada.file_name());

                copiar_arquivos(
                    tera,
                    context,
                    &origem_arquivo,
                    &destino_arquivo,
                )?;
            }
        }
    } else {
        // println!("render from {} to {}", origem.display(), destino.display());
        let content = tera.render(origem.to_str().unwrap(), context)?;
        let mut destino_file = std::fs::File::create(&destino)?;
        destino_file.write_all(&content.as_bytes())?;
        // std::fs::copy(origem, destino)?;
    }

    Ok(())
}

pub mod render {
    tonic::include_proto!("render_proto");
}
use tonic::{transport::Server, Request, Response, Status};

use crate::render::render_server::{Render, RenderServer};
use crate::render::{
    create_repo_response::Result as CreateResult,
    CreateRepoRequest, CreateRepoResponse, RepoResponse,
};

#[derive(Default)]
pub struct MpsRenderGrpcServer;

impl From<String> for RepoResponse {
    fn from(s: String) -> Self {
        RepoResponse { name: s }
    }
}

#[tonic::async_trait]
impl Render for MpsRenderGrpcServer {
    async fn create_repo(
        &self,
        request: Request<CreateRepoRequest>,
    ) -> Result<Response<CreateRepoResponse>, Status> {
        let name: String = request.into_inner().name;
        todo!()
        // let resp = render_create_repository("", "", &name).await;
        // if let Err(e) = resp {
        //     return Err(Status::invalid_argument(e.to_string()));
        // }

        // let resp = resp.unwrap();

        // let response = CreateRepoResponse {
        //     result: CreateResult::Success.into(),
        //     repository: Some(resp.into()),
        // };

        // Ok(Response::new(response))
    }
}

pub async fn server() {
    let addr = "[::1]:50060".parse().unwrap();
    let render = MpsRenderGrpcServer::default();

    Server::builder()
        .add_service(RenderServer::new(render))
        .serve(addr)
        .await
        .unwrap();
}

