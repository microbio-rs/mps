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

#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let repo_uri = "{account_id}.dkr.ecr.{region}.amazonaws.com/{project_name}";
    let tag = "0.0.1";
    let path = Path::new("/tmp/murilobsd/mps-sample-nestjs-1");
    let dockerfile_name = "Dockerfile";
    let dockerfile_path = path.join(dockerfile_name);

    mps_container::server().await;

    // let docker = mps_container::docker_connect().await?;
    // mps_container::build_image(&docker, &repo_uri, tag, path.to_str().unwrap())
    //     .await?;

    // let (_, password) = mps_container::get_credential().await;
    // mps_container::push_image(&docker, &repo_uri, tag, &password).await;

    Ok(())
}
