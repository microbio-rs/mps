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

use std::{path::PathBuf, sync::Arc};

use clap::{
    builder::styling::AnsiColor, value_parser, Arg, ArgAction, ColorChoice,
    Command,
};
use colored::Colorize;

use crate::{grpc, MpsScmConfig};

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
    let banner: String = r#"
        ███╗   ███╗██████╗ ███████╗      ███████╗ ██████╗███╗   ███╗
        ████╗ ████║██╔══██╗██╔════╝      ██╔════╝██╔════╝████╗ ████║
        ██╔████╔██║██████╔╝███████╗█████╗███████╗██║     ██╔████╔██║
        ██║╚██╔╝██║██╔═══╝ ╚════██║╚════╝╚════██║██║     ██║╚██╔╝██║
        ██║ ╚═╝ ██║██║     ███████║      ███████║╚██████╗██║ ╚═╝ ██║
        ╚═╝     ╚═╝╚═╝     ╚══════╝      ╚══════╝ ╚═════╝╚═╝     ╚═╝
            𓍊𓋼𓍊𓋼𓍊 mps - source control manager service v0.1.0"#
        .green()
        .to_string();

    let matches = Command::new("mps_scm")
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
                .hide(false)
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

    // if matches.get_flag("quiet") {
    //    flags.log_level = Some(Level::Error);
    //  } else if let Some(log_level) = matches.get_one::<String>("log-level") {
    //    flags.log_level = match log_level.as_str() {
    //      "trace" => Some(Level::Trace),
    //      "debug" => Some(Level::Debug),
    //      "info" => Some(Level::Info),
    //      _ => unreachable!(),
    //    };
    //  }

    // read config
    let config_path: &PathBuf =
        matches.get_one("config").expect("`config` is required");
    let scm_config = MpsScmConfig::load(config_path).unwrap();

    match matches.subcommand() {
        Some(("grpc", _)) => {
            // TODO: better aprote
            let provider =
                crate::GithubProvider::new(scm_config.github.clone());
            let service = crate::MpsScmService::new(Box::new(provider));
            let state = grpc::MpsScmGrpcState::new(Arc::new(service));

            grpc::server(Arc::new(state)).await
        }
        _ => {}
    };
}
