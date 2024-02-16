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

use std::{sync::Arc, path::PathBuf};

use clap::{value_parser, Arg, Command};

use crate::{config::MpsScmConfig, grpc};

////
//// init a github provider
////
//let _provider = github::GithubProvider::new(scm_config.github.clone());

////
//// create github repo
////
//// let new_repo = github::NewRepository { name: "test-repo".to_string() };
//// let result = provider.create_github_repository(new_repo).await;
//// println!("{:?}", result);

////
//// clone sample repo
////
//// let _output = format!(
////     "{path}/{owner}/{repo_name}",
////     path = &scm_config.path,
////     owner = &scm_config.github.owner,
////     repo_name = &new_repo.name
//// );
//// let sample_repo =
////     local::LocalProvider::clone(&scm_config.sample_repo, &output);
//// let git_dir = format!("{output}/.git", output=&output);

////
//// remove git folder to reinit repo
////
//// match std::fs::remove_dir_all(&git_dir) {
////     Ok(()) => debug!("Pasta .git removida com sucesso!"),
////     Err(err) => panic!("Erro ao remover a pasta .git: {}", err),
//// };

////
//// mps_scm: init, commit, push files to git repo
////
//// local::icp(
////     "/tmp/murilobsd/test-repo",
////     "git@github.com:murilobsd/test-repo.git",
////     "git",
////     Path::new("/home/user/.ssh/mykey"),
//// )?;
pub async fn run() {
    let matches = Command::new("mps_scm")
        .version("0.1.0")
        .author("Murilo Ijanc'")
        .about("mps microservice - source control manager")
        .subcommand(Command::new("grpc").about("Run grpc server"))
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("ARQUIVO")
                .help("Caminho do arquivo de configuração")
                .value_parser(value_parser!(PathBuf))
                .required(true),
        )
        .get_matches();

    let config_path: &PathBuf =
        matches.get_one("config").expect("`config` is required");

    let scm_config = MpsScmConfig::load(config_path).unwrap();

    match matches.subcommand() {
        Some(("grpc", _)) => {
            let provider = crate::GithubProvider::new(scm_config.github.clone());
            let service = crate::MpsScmService::new(Box::new(provider));
            let state = grpc::MpsScmGrpcState::new(Arc::new(service));
            grpc::server(Arc::new(state)).await
        }
        _ => {}
    };
}
