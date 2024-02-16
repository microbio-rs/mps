use std::path::PathBuf;

use clap::{value_parser, Arg, Command};

use crate::{config::MpsScmConfig, grpc};

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

    let _scm_config = MpsScmConfig::load(config_path).unwrap();

    match matches.subcommand() {
        Some(("grpc", _)) => grpc::server().await,
        _ => {}
    }
}
