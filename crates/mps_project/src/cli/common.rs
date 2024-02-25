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

use clap::builder::{styling::AnsiColor, Styles};
use colored::Colorize;

pub fn banner() -> String {
    let info = format!(
        "𓍊𓋼𓍊𓋼𓍊 mps - project manager service v{}",
        env!("CARGO_PKG_VERSION")
    );
    let b: String = format!(
        "{}\n{}",
        "
███╗   ███╗██████╗ ███████╗      ██████╗ ██████╗  ██████╗
████╗ ████║██╔══██╗██╔════╝      ██╔══██╗██╔══██╗██╔═══██╗
██╔████╔██║██████╔╝███████╗█████╗██████╔╝██████╔╝██║   ██║
██║╚██╔╝██║██╔═══╝ ╚════██║╚════╝██╔═══╝ ██╔══██╗██║   ██║
██║ ╚═╝ ██║██║     ███████║      ██║     ██║  ██║╚██████╔╝
╚═╝     ╚═╝╚═╝     ╚══════╝      ╚═╝     ╚═╝  ╚═╝ ╚═════╝"
        center_banner(&info)
    )
    .trim()
    .green()
    .to_string();
    let b = center_banner(&b);
    b
}

fn center_banner(texto: &str) -> String {
    let largura_total = 80;
    let largura_texto = texto.len();

    if largura_texto >= largura_total {
        texto.to_string()
    } else {
        let espacos_a_esquerda = (largura_total - largura_texto) / 2;
        let espacos_a_direita =
            largura_total - largura_texto - espacos_a_esquerda;

        let texto_centralizado = format!(
            "{:width_left$}{}{:width_right$}",
            "",
            texto,
            "",
            width_left = espacos_a_esquerda,
            width_right = espacos_a_direita
        );

        texto_centralizado.to_string()
    }
}

pub fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::White.on_default())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Green.on_default())
}
