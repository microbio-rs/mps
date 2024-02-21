#![allow(dead_code, unused_must_use)]
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

////
//// mps_container: build dockerfile, push docker image to registry
////

//
// TODO: multi arch
//
use aws_config::meta::region::RegionProviderChain;
use aws_config::Region;
use bollard::image::{BuildImageOptions, BuilderVersion};
use bollard::models::BuildInfoAux;
use bollard::Docker;
use dockerfile_parser::Dockerfile;

use base64::prelude::*;
use futures_util::stream::StreamExt;

use bollard::auth::DockerCredentials;
use bollard::image::PushImageOptions;
use std::io::Write;

use std::default::Default;

use tonic::{transport::Server, Request, Response, Status};

pub mod container {
    tonic::include_proto!("docker_proto");
}

use crate::container::container_server::{Container, ContainerServer};
use crate::container::{
    build_container_response::Result as BuildResult, BuildContainerRequest,
    BuildContainerResponse, BuildResponse,
};

#[derive(thiserror::Error, Debug)]
pub enum MpsContainerError {
    #[error("failed bollard: {0}")]
    Bollard(#[from] bollard::errors::Error),

    // #[error("failed io: {0}")]
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub async fn build_image(
    docker: &Docker,
    image_name: &str,
    tag: &str,
    build_path: &str,
) -> Result<(), MpsContainerError> {
    println!("{image_name}");
    let compressed = compress(build_path)?;
    let build_image_options = BuildImageOptions {
        dockerfile: "Dockerfile",
        t: &format!("{image_name}:{tag}"),
        // nocache: true,
        // q: true,
        version: BuilderVersion::BuilderBuildKit,
        // pull: true,
        // rm: true,
        session: Some(String::from(image_name)),
        ..Default::default()
    };

    let mut image_build_stream =
        docker.build_image(build_image_options, None, Some(compressed.into()));

    while let Some(Ok(bollard::models::BuildInfo {
        aux: Some(BuildInfoAux::BuildKit(inner)),
        ..
    })) = image_build_stream.next().await
    {
        println!("Response: {:?}", inner);
    }

    Ok(())
}

pub async fn docker_connect() -> Result<Docker, MpsContainerError> {
    Ok(Docker::connect_with_socket_defaults()?)
}

fn compress(build_path: &str) -> Result<Vec<u8>, MpsContainerError> {
    let mut buf = vec![];
    let mut tar = tar::Builder::new(&mut buf);
    tar.append_dir_all(".", build_path)?;
    let uncompressed = tar.into_inner()?;
    let mut c = flate2::write::GzEncoder::new(
        Vec::new(),
        flate2::Compression::default(),
    );
    c.write_all(&uncompressed)?;
    Ok(c.finish()?)
}

pub async fn get_credential() -> (String, String) {
    let region_provider =
        RegionProviderChain::first_try(Some("us-east-1").map(Region::new))
            .or_default_provider()
            .or_else(Region::new("us-east-1"));

    let shared_config =
        aws_config::from_env().region(region_provider).load().await;
    let client = aws_sdk_ecr::Client::new(&shared_config);
    let token = client.get_authorization_token().send().await.unwrap();
    let authorization =
        token.authorization_data()[0].authorization_token().unwrap();
    let data = BASE64_STANDARD.decode(authorization.as_bytes()).unwrap();
    let parts = String::from_utf8(data).unwrap();
    let parts: Vec<&str> = parts.split(':').collect();
    (parts[0].to_string(), parts[1].to_string())
}

pub async fn push_image(
    docker: &Docker,
    reposity_uri: &str,
    tag: &str,
    password: &str,
) {
    ////
    //// Tag
    ////
    //let tag_options = Some(TagImageOptions { repo, tag: "latest" });

    //docker.tag_image(id, tag_options).await;

    ////
    //// Push
    ////
    let push_options = Some(PushImageOptions { tag });

    let credentials = Some(DockerCredentials {
        username: Some("AWS".to_string()),
        password: Some(password.to_string()),
        ..Default::default()
    });
    let repo = format!("{reposity_uri}:{tag}");

    let stream = docker.push_image(&repo, push_options, credentials);

    stream
        .for_each(|l| async {
            println!("{:?}", l.unwrap());
        })
        .await;
}

fn get_port_from_dockerfile(dockerfile: &str) -> Option<u16> {
    let dockerfile = Dockerfile::parse(dockerfile).unwrap();
    let mut port: u16 = 0;

    for stage in dockerfile.iter_stages() {
        println!(
            "stage #{} (parent: {:?}, root: {:?})",
            stage.index, stage.parent, stage.root
        );

        for ins in stage.instructions {
            match ins {
                dockerfile_parser::Instruction::Misc(misc) => {
                    if misc.instruction.content.as_str() == "EXPOSE" {
                        match misc.arguments.components.get(0).unwrap() {
                            dockerfile_parser::BreakableStringComponent::String(c)
                                => {
                                    port = c.content.trim().parse().unwrap();
                                    break;
                                }
                            _ => {},
                        }
                    }
                }
                _ => {}
            }
        }
    }
    if port == 0 {
        None
    } else {
        Some(port)
    }
}

#[derive(Default)]
pub struct MpsContainerGrpcServer;

// impl From<String> for RepoResponse {
//     fn from(s: String) -> Self {
//         RepoResponse { name: s }
//     }
// }

#[derive(Debug)]
pub struct DockerBuildParams {
    local_path: String,
    repo_uri: String,
    tag: String,
    path: String,
    dockerfile_name: String,
    dockerfile_path: String,
}

impl From<BuildContainerRequest> for DockerBuildParams {
    fn from(p: BuildContainerRequest) -> Self {
        Self {
            local_path: p.local_path,
            repo_uri: p.repo_uri,
            tag: p.tag,
            path: p.path,
            dockerfile_name: p.dockerfile_name,
            dockerfile_path: p.dockerfile_path,
        }
    }
}

#[tonic::async_trait]
impl Container for MpsContainerGrpcServer {
    async fn build(
        &self,
        request: Request<BuildContainerRequest>,
    ) -> Result<Response<BuildContainerResponse>, Status> {
        let params: DockerBuildParams = request.into_inner().into();

        let docker = docker_connect().await.unwrap();
        if let Err(e) = build_image(
            &docker,
            &params.repo_uri,
            &params.tag,
            params.local_path.as_str(),
        )
        .await
        {
            return Err(Status::invalid_argument(e.to_string()));
        }

        let (_, password) = get_credential().await;
        push_image(&docker, &params.repo_uri, &params.tag, &password).await;

        let response = BuildContainerResponse {
            result: BuildResult::Success.into(),
            build: Some(BuildResponse { repo_uri: params.repo_uri }),
        };

        Ok(Response::new(response))
    }
}

pub async fn server() {
    let addr = "[::1]:50061".parse().unwrap();
    let container = MpsContainerGrpcServer::default();

    Server::builder()
        .add_service(ContainerServer::new(container))
        .serve(addr)
        .await
        .unwrap();
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[ignore]
//     #[tokio::test]
//     async fn docker_push_image() {
//         let client = docker_connect().await;
//         push_image(&client, "myimage", "").await;
//         assert!(true);
//     }

//     #[ignore]
//     #[tokio::test]
//     async fn aws_ecr_credential() {
//         let _credential = get_credential().await;
//         assert!(true);
//     }

//     #[ignore]
//     #[tokio::test]
//     async fn docker_build_image() {
//         let client = docker_connect().await;
//         let dockerfile = String::from(
//             "FROM alpine as builder1
//     RUN touch bollard.txt
//     FROM alpine as builder2
//     RUN --mount=type=bind,from=builder1,target=mnt cp mnt/bollard.txt buildkit-bollard.txt
//     ENTRYPOINT ls buildkit-bollard.txt
//     ",
//         );

//         build_image(&client, "myimage", &dockerfile).await;

//         assert!(true);
//     }

//     #[test]
//     fn get_port_dockerfile() {
//         let dockerfile = String::from(
//             "FROM alpine as builder1
//     RUN touch bollard.txt
//     FROM alpine as builder2
//     RUN --mount=type=bind,from=builder1,target=mnt cp mnt/bollard.txt buildkit-bollard.txt
//     EXPOSE 3000
//     ENTRYPOINT ls buildkit-bollard.txt
//             "
//         );
//         let port = get_port_from_dockerfile(&dockerfile);
//         assert!(port.is_some());
//         assert_eq!(3000 as u16, port.unwrap());
//     }
// }
