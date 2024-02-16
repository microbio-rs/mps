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

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // Crie uma instância Tera
    let tera = Tera::new("templates/*.yml")?;

    // Crie um contexto de dados para o template
    let mut context = Context::new();
    context.insert("name", "hello-world");
    context.insert("port", &8081);
    context.insert("image", "gcr.io/google-samples/hello-app");
    context.insert("version", "1.0");
    context.insert("namespace", "platform-engineering");
    context.insert("domain", "info");
    context.insert("replicas", &2);

    let resultado = tera.render("deployment.yml", &context);

    // Verifique se a renderização foi bem-sucedida
    match resultado {
        Ok(renderizado) => {
            let d: Deployment = serde_yaml::from_str(&renderizado)?;
            println!("template renderizado:\n\n{:?}", d);
        }
        Err(erro) => eprintln!("Erro ao renderizar o template: {:?}", erro),
    };

    Ok(())
}
