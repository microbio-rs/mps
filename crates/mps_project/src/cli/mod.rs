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

use std::path;

use clap::{
    value_parser, Arg, ColorChoice, Command,
};

mod migrations;
mod seed;
mod version;
mod common;
mod grpc;
pub(crate) mod error;

const MAX_TERM_WIDTH: usize = 80;
const COLOR_CHOICE: ColorChoice = ColorChoice::Auto;

fn subcommand_run() -> Command {
    Command::new("run")
        .about("Run grpc server, worker, seed, migrations")
        .subcommand(migrations::subcommand())
        .subcommand(seed::subcommand())
        .subcommand(grpc::subcommand())
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("ARQUIVO")
                .help("Caminho do arquivo de configuração")
                .value_parser(value_parser!(path::PathBuf))
                .required(true),
        )
}

pub async fn run() -> Result<(), error::Error> {
    let matches = Command::new("mps_project")
        .styles(common::styles())
        .color(COLOR_CHOICE)
        .max_term_width(MAX_TERM_WIDTH)
        .about(common::banner())
        .subcommand(subcommand_run())
        .subcommand(version::subcommand())
        .get_matches();

    mps_log::MpsLog::builder().filter_level("debug").with_ansi(true).init()?;

    // read config
    // let config_path: &PathBuf =
    //     matches.get_one("config").expect("`config` is required");
    // let project_config = MpsProjectConfig::load(config_path)?;
    // let pool = PgPool::connect(&project_config.database.uri)
    //     .await
    //     .expect("Failed to connect to the database");

    // let project_repo = crate::ProjectRepository::new(pool);

    // let repo = grpc_scm_client.create_repo(create_request).await.unwrap();
    // println!("repo = {repo:?}");

    match matches.subcommand() {
        Some(("grpc", _)) => {
            // TODO: better aprote
            // let provider =
            //     crate::GithubProvider::new(project_config.github.clone());
            // let service = crate::MpsProjectService::new(Box::new(provider));
            // let state = grpc::MpsProjectGrpcState::new(Arc::new(service));

            // grpc::server(Arc::new(state)).await
            // grpc::server(&project_config.grpc_server, project_repo).await
            println!("");
        }
        Some(("version", sub_m)) => {
            let info = version::Info::new();
            match sub_m.get_flag("json") {
                true => println!("{}", info.to_json()),
                false => println!("{}", info),
            }
        }
        _ => {}
    };

    Ok(())
}
