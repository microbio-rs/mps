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
use k8s_openapi::api::apps::v1::Deployment;
use tera::{Context, Tera};

//
// mps_orchestration: create manifest k8s (dev,prod) (deploy,service,namespace,ingress)
// mps_orchestration TODO: get url load balancer
//

#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // Crie uma instância Tera
    let tera = Tera::new("templates/*.yml")?;

    // Crie um contexto de dados para o template
    let mut context = Context::new();
    context.insert("name", "mps-sample-nestjs");
    context.insert("port", &3000);
    context.insert(
        "image",
        "account_id.dkr.ecr.region.amazonaws.com/project_name",
    );
    context.insert("version", "0.0.1");
    context.insert("namespace", "platform-engineering");
    context.insert("domain", "info");
    context.insert("replicas", &2);

    // let namespace = tera.render("namespace.yml", &context);
    // match namespace {
    //     Ok(renderizado) => {
    //         // let d: Deployment = serde_yaml::from_str(&renderizado)?;
    //         // println!("template renderizado:\n\n{:?}", d);
    //         mps_orchestrator::create_namespace(&renderizado).await.unwrap()
    //     }
    //     Err(erro) => eprintln!("Erro ao renderizar o template: {:?}", erro),
    // };

    let deployment = tera.render("deployment.yml", &context);
    match deployment {
        Ok(renderizado) => mps_orchestrator::create_deployment(
            &renderizado,
            "mps-sample-nestjs",
            "platform-engineering",
        )
        .await
        .unwrap(),
        Err(erro) => eprintln!("Erro ao renderizar o template: {:?}", erro),
    };

    Ok(())
}
