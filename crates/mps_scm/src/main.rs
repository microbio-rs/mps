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
use color_eyre::eyre::Result;

use mps_scm::{config::MpsScmConfig, github, local};

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    init_tracing();

    // load config
    let scm_config = MpsScmConfig::load("./crates/mps_scm/config.toml")?;

    // init a github provider
    let _provider = github::GithubProvider::new(scm_config.github.clone());

    //TODO: create github repo
    let new_repo = github::NewRepository { name: "test-repo".to_string() };
    // let result = provider.create_github_repository(new_repo).await;
    // println!("{:?}", result);

    // TODO: clone sample repo
    let output = format!(
        "{path}/{owner}/{repo_name}",
        path = &scm_config.path,
        owner = &scm_config.github.owner,
        repo_name = &new_repo.name
    );
    let sample_repo =
        local::LocalProvider::clone(&scm_config.sample_repo, &output);
    // TODO: render template files and write it into filesystem
    // TODO: create ecr repository
    // TODO: push files to new repo
    // TODO: update manifest k8s (dev,prod)
    // TODO: get url load balancer

    Ok(())
}

fn init_tracing() {
    use tracing_subscriber::{fmt, prelude::*, registry, EnvFilter};
    registry().with(fmt::layer()).with(EnvFilter::from_env("MPS_LOG")).init();
}
