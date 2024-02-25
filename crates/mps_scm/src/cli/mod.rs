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

use clap::Command;

mod common;
mod consts;
mod grpc;
mod migrations;
mod seed;
mod version;

mod error;
use error::*;

fn subcommand_run() -> Command {
    Command::new(consts::SUBCMD_RUN)
        .about("Run grpc server, worker, seed, migrations")
        .subcommand(migrations::subcommand())
        .subcommand(seed::subcommand())
        .subcommand(grpc::subcommand())
}

pub async fn run() -> Result<(), error::Error> {
    let matches = Command::new("mps_scm")
        .styles(common::styles())
        .color(consts::COLOR_CHOICE)
        .max_term_width(consts::MAX_TERM_WIDTH)
        .about(common::banner())
        .subcommand(subcommand_run())
        .subcommand(version::subcommand())
        .get_matches();

    mps_log::MpsLog::builder().filter_level("debug").with_ansi(true).init()?;

    match matches.subcommand() {
        Some((consts::SUBCMD_RUN, sub_m)) => match sub_m.subcommand() {
            Some((consts::SUBCMD_GRPC, m)) => grpc::run(m).await?,
            Some((consts::SUBCMD_MIGRATIONS, m)) => migrations::run(m).await?,
            Some((consts::SUBCMD_SEED, m)) => seed::run(m).await?,
            _ => {}
        },
        Some((consts::SUBCMD_VERSION, sub_m)) => version::run(sub_m)?,
        _ => {}
    };

    Ok(())
}
