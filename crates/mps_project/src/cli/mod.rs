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

use std::path::PathBuf;

use clap::{
    builder::styling::AnsiColor, value_parser, Arg, ArgAction, ColorChoice,
    Command,
};
use colored::Colorize;

mod version;

use crate::{Error, MpsProjectConfig};

pub async fn run() -> Result<(), Error> {
    let banner: String = r#"
            ███╗   ███╗██████╗ ███████╗      ██████╗ ██████╗  ██████╗
            ████╗ ████║██╔══██╗██╔════╝      ██╔══██╗██╔══██╗██╔═══██╗
            ██╔████╔██║██████╔╝███████╗█████╗██████╔╝██████╔╝██║   ██║
            ██║╚██╔╝██║██╔═══╝ ╚════██║╚════╝██╔═══╝ ██╔══██╗██║   ██║
            ██║ ╚═╝ ██║██║     ███████║      ██║     ██║  ██║╚██████╔╝
            ╚═╝     ╚═╝╚═╝     ╚══════╝      ╚═╝     ╚═╝  ╚═╝ ╚═════╝
                 𓍊𓋼𓍊𓋼𓍊 mps - project manager service v0.1.0"#
        .green()
        .to_string();

    let matches = Command::new("mps_project")
        .styles(
            clap::builder::Styles::styled()
                .header(AnsiColor::Yellow.on_default())
                .usage(AnsiColor::White.on_default())
                .literal(AnsiColor::Green.on_default())
                .placeholder(AnsiColor::Green.on_default()),
        )
        .color(ColorChoice::Auto)
        .max_term_width(80)
        .version("0.1.0")
        .next_display_order(1000)
        .author("Murilo Ijanc'")
        .about(banner)
        .subcommand(Command::new("grpc").about("Run grpc server"))
        .subcommand(version::subcommand())
        .subcommand(
            Command::new("migration").about("Run migrations").arg(
                Arg::new("path")
                    .long("path")
                    .value_name("PATH")
                    .help("Caminho da pasta migrations")
                    .value_parser(value_parser!(PathBuf))
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("seed").about("Run seed").arg(
                Arg::new("size")
                    .long("size")
                    .value_name("SIZE")
                    .help("Quantidade de registros gerados")
                    .default_value("10")
                    .required(true),
            ),
        )
        .arg(
            Arg::new("log-level")
                .short('L')
                .long("log-level")
                .help("Set log level")
                .hide(true)
                .value_parser(["trace", "debug", "info"])
                .global(true),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Suppress diagnostic output")
                .action(ArgAction::SetTrue)
                .global(true),
        )
        // .arg(
        //     Arg::new("config")
        //         .short('c')
        //         .long("config")
        //         .value_name("ARQUIVO")
        //         .help("Caminho do arquivo de configuração")
        //         .value_parser(value_parser!(PathBuf))
        //         .required(true),
        // )
        .get_matches();

    mps_log::MpsLog::builder().filter_level("debug").with_ansi(true).init()?;

    // let database_uri = "postgres://postgres:postgres@0.0.0.0:5432/mps_project";
    // project_repo.seed(10).await?;

    // crate::run_migration(
    //     database_uri,
    //     "/home/msi/src/mps/crates/mps_project/migrations",
    // )?;

    // read config
    // let config_path: &PathBuf =
    //     matches.get_one("config").expect("`config` is required");
    // let project_config = MpsProjectConfig::load(config_path)?;
    // let pool = PgPool::connect(&project_config.database.uri)
    //     .await
    //     .expect("Failed to connect to the database");

    // let project_repo = crate::ProjectRepository::new(pool);

    // crate::kafka::kafka_check_run().await;

    // request grpc
    // let create_request = mps_scm::grpc::scm::CreateRepoRequest {
    //     provider: mps_scm::grpc::scm::Provider::Github.into(),
    //     name: "mps-simple-repo".to_string(),
    // };
    // let grpc_scm_config = mps_scm::grpc::client::ScmGrpcClientConfig {
    //     host: "http://[::1]".to_string(),
    //     port: 50051,
    // };
    // let mut grpc_scm_client =
    //     mps_scm::grpc::client::ScmGrpcClient::new(&grpc_scm_config)
    //         .await
    //         .unwrap();

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
