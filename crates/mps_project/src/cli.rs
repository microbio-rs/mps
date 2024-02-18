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

use std::{path::PathBuf, sync::Arc, time::Duration};

use clap::{
    builder::styling::AnsiColor, value_parser, Arg, ArgAction, ColorChoice,
    Command,
};
use colored::Colorize;
use sqlx::postgres::PgConnectOptions;
use sqlx::PgPool;

use crate::{grpc, MpsProjectConfig, MpsProjectError};

pub async fn run() -> Result<(), MpsProjectError> {
    let banner: String = r#"
███╗   ███╗██████╗ ███████╗      ██████╗ ██████╗  ██████╗      ██╗███████╗ ██████╗████████╗
████╗ ████║██╔══██╗██╔════╝      ██╔══██╗██╔══██╗██╔═══██╗     ██║██╔════╝██╔════╝╚══██╔══╝
██╔████╔██║██████╔╝███████╗█████╗██████╔╝██████╔╝██║   ██║     ██║█████╗  ██║        ██║   
██║╚██╔╝██║██╔═══╝ ╚════██║╚════╝██╔═══╝ ██╔══██╗██║   ██║██   ██║██╔══╝  ██║        ██║   
██║ ╚═╝ ██║██║     ███████║      ██║     ██║  ██║╚██████╔╝╚█████╔╝███████╗╚██████╗   ██║   
╚═╝     ╚═╝╚═╝     ╚══════╝      ╚═╝     ╚═╝  ╚═╝ ╚═════╝  ╚════╝ ╚══════╝ ╚═════╝   ╚═╝   
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

    mps_log::MpsLog::builder().filter_level("debug").with_ansi(true).init()?;

    // let pool_options = PgConnectOptions::new()
    //     .connect_timeout(Duration::from_secs(5))
    //     .max_connections(10)
    //     .connection_str("postgres://postgres:postgres@0.0.0.0:5432/mps_project");

    let pool = PgPool::connect(
        "postgres://postgres:postgres@0.0.0.0:5432/mps_project",
    )
    .await
    .expect("Failed to connect to the database");

    let project_repo = crate::ProjectRepository::new(pool);
    project_repo.seed(10).await?;

    // read config
    let config_path: &PathBuf =
        matches.get_one("config").expect("`config` is required");
    let project_config = MpsProjectConfig::load(config_path)?;

    // match matches.subcommand() {
    //     Some(("grpc", _)) => {
    //         // TODO: better aprote
    //         let provider =
    //             crate::GithubProvider::new(project_config.github.clone());
    //         let service = crate::MpsProjectService::new(Box::new(provider));
    //         let state = grpc::MpsProjectGrpcState::new(Arc::new(service));

    //         grpc::server(Arc::new(state)).await
    //     }
    //     _ => {}
    // };
    Ok(())
}
