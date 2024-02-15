#![allow(unused_imports)]
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
use aws_sdk_ecr::{Client, Config};
use color_eyre::eyre::Result;
use tracing::{debug, info, instrument};

use mps_scm::{config::MpsScmConfig, ecr, github, local};

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    init_tracing();

    //
    // load config
    //
    let scm_config = MpsScmConfig::load("./crates/mps_scm/config.toml")?;

    //
    // init a github provider
    //
    let _provider = github::GithubProvider::new(scm_config.github.clone());

    //
    // create github repo
    //
    // let new_repo = github::NewRepository { name: "test-repo".to_string() };
    // let result = provider.create_github_repository(new_repo).await;
    // println!("{:?}", result);

    //
    // clone sample repo
    //
    // let _output = format!(
    //     "{path}/{owner}/{repo_name}",
    //     path = &scm_config.path,
    //     owner = &scm_config.github.owner,
    //     repo_name = &new_repo.name
    // );
    // let sample_repo =
    //     local::LocalProvider::clone(&scm_config.sample_repo, &output);
    // let git_dir = format!("{output}/.git", output=&output);

    //
    // remove git folder to reinit repo
    //
    // match std::fs::remove_dir_all(&git_dir) {
    //     Ok(()) => debug!("Pasta .git removida com sucesso!"),
    //     Err(err) => panic!("Erro ao remover a pasta .git: {}", err),
    // };

    // TODO: render template files and write it into filesystem

    //
    // create ecr repository
    //
    // Configuração do cliente AWS ECR
    // let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    // // Note: requires the `behavior-version-latest` feature enabled
    // let client_config = aws_config::from_env().region(region_provider).load().await;
    // let client = Client::new(&client_config);
    // // Nome do repositório a ser criado
    // let repository_name = &new_repo.name;
    // // Criação do repositório
    // ecr::create_repository(&client, repository_name).await?;

    //
    // mps_container: build dockerfile, push docker image to registry
    //

    //
    // mps_scm: init, commit, push files to git repo
    //
    // local::icp(
    //     "/tmp/murilobsd/test-repo",
    //     "git@github.com:murilobsd/test-repo.git",
    //     "git",
    //     Path::new("/home/user/.ssh/mykey"),
    // )?;

    //
    // mps_orchestration: create manifest k8s (dev,prod) (deploy,service,namespace,ingress)
    // mps_orchestration TODO: get url load balancer

    Ok(())
}

fn init_tracing() {
    use tracing_subscriber::{fmt, prelude::*, registry, EnvFilter};
    registry().with(fmt::layer()).with(EnvFilter::from_env("MPS_LOG")).init();
}
