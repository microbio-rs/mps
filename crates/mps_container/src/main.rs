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

use std::path::Path;

use aws_config::meta::region::RegionProviderChain;
use aws_config::Region;
use bollard::image::{BuildImageOptions, BuilderVersion};
use bollard::models::BuildInfoAux;
use bollard::Docker;
use dockerfile_parser::Dockerfile;

use futures_util::stream::StreamExt;

use base64::prelude::*;
use bollard::auth::DockerCredentials;
use bollard::image::PushImageOptions;
use bollard::image::TagImageOptions;
use std::io::Read;
use std::io::Write;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[derive(thiserror::Error, Debug)]
pub enum MpsContainerError {
    #[error("failed bollard: {0}")]
    Bollard(#[from] bollard::errors::Error),
    #[error("failed io: {0}")]
    IoError(#[from] std::io::Error),
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let repo_uri =
        "{account_id}.dkr.ecr.{region}.amazonaws.com/{project_name}";
    let tag = "0.0.1";
    let path = Path::new("/tmp/murilobsd/mps-sample-nestjs-1");
    let dockerfile_name = "Dockerfile";
    let dockerfile_path = path.join(dockerfile_name);

    let docker = docker_connect().await?;
    build_image(&docker, &repo_uri, tag, path.to_str().unwrap()).await?;

    let (_, password) = get_credential().await;
    push_image(&docker, &repo_uri, tag, &password).await;

    Ok(())
}

async fn build_image(
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

async fn docker_connect() -> Result<Docker, MpsContainerError> {
    Ok(Docker::connect_with_socket_defaults()?)
}

fn compress(build_path: &str) -> Result<Vec<u8>, MpsContainerError> {
    // let tar_file_path = "/tmp/murilobsd/arquivo.tar";
    // let tar_file = std::fs::File::create(tar_file_path)?;
    // let mut builder = tar::Builder::new(tar_file);
    // builder.append_dir_all("", build_path)?;

    let mut buf = vec![];
    let mut tar = tar::Builder::new(&mut buf);
    tar.append_dir_all(".", build_path)?;
    let uncompressed = tar.into_inner().unwrap();
    let mut c = flate2::write::GzEncoder::new(
        Vec::new(),
        flate2::Compression::default(),
    );
    c.write_all(&uncompressed)?;
    Ok(c.finish()?)

}

async fn get_credential() -> (String, String) {
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


async fn push_image(docker: &Docker, reposity_uri: &str, tag: &str, password: &str) {
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
