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

use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use tonic::{transport::Server, Request, Response, Status};

pub mod template {
    tonic::include_proto!("template_proto");
}

use crate::template::template_server::{Template, TemplateServer};
use crate::template::{
    render_template_response::Result as RenderResult, RenderResponse,
    RenderTemplateRequest, RenderTemplateResponse,
};

#[derive(thiserror::Error, Debug)]
pub enum MpsTemplateError {
    #[error("Template error: {0}")]
    Tera(#[from] tera::Error),

    #[error("Io error: {0}")]
    Io(#[from] io::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicTemplateContext {
    pub project_name: String,
}

pub fn render<P: AsRef<Path>>(
    origem: P,
    destino: P,
    context: Context,
) -> Result<(), MpsTemplateError> {
    let mut tera = Tera::default();
    // TODO: improve this, this gets bad when the directory has numerous files
    mount_tera(&mut tera, origem.as_ref())?;
    copiar_arquivos(&tera, &context, origem.as_ref(), destino.as_ref())?;

    Ok(())
}

fn mount_tera(tera: &mut Tera, origem: &Path) -> Result<(), MpsTemplateError> {
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
) -> Result<(), MpsTemplateError> {
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
        // println!("template from {} to {}", origem.display(), destino.display());
        let content = tera.render(origem.to_str().unwrap(), context)?;
        let mut destino_file = std::fs::File::create(&destino)?;
        destino_file.write_all(&content.as_bytes())?;
        // std::fs::copy(origem, destino)?;
    }

    Ok(())
}

#[derive(Default)]
pub struct MpsTemplateGrpcServer;

// impl From<String> for RepoResponse {
//     fn from(s: String) -> Self {
//         RepoResponse { name: s }
//     }
// }

#[tonic::async_trait]
impl Template for MpsTemplateGrpcServer {
    async fn render(
        &self,
        request: Request<RenderTemplateRequest>,
    ) -> Result<Response<RenderTemplateResponse>, Status> {
        let input_path: String = request.get_ref().input.to_string();
        let output_path = String::from("/tmp/murilobsd/mps-sample-nestjs-1");
        let context_json: BasicTemplateContext =
            serde_json::from_str(&request.get_ref().context).unwrap();
        let context: Context = Context::from_serialize(context_json).unwrap();

        if let Err(e) =
            render(input_path.as_str(), output_path.as_str(), context)
        {
            return Err(Status::invalid_argument(e.to_string()));
        }

        let response = RenderTemplateResponse {
            result: RenderResult::Success.into(),
            render: Some(RenderResponse { output: output_path }),
        };

        Ok(Response::new(response))
    }
}

pub async fn server() {
    let addr = "[::1]:50060".parse().unwrap();
    let template = MpsTemplateGrpcServer::default();

    Server::builder()
        .add_service(TemplateServer::new(template))
        .serve(addr)
        .await
        .unwrap();
}
