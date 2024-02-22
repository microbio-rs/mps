#![allow(dead_code)]
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
use k8s_openapi::api::core::v1::{Namespace, Pod};
use serde::{Deserialize, Serialize};

use kube::{
    api::{
        Api, PostParams,
        ResourceExt,
    },
    runtime::wait::{await_condition, conditions::is_pod_running},
    Client,
};

use tera::{Context, Tera};
use tonic::{transport::Server, Request, Response, Status};

pub mod kubernetes {
    tonic::include_proto!("kubernetes_proto");
}

use crate::kubernetes::kubernetes_server::{Kubernetes, KubernetesServer};
use crate::kubernetes::{
    apply_kubernetes_response::Result as ApplyResult, ApplyKubernetesRequest,
    ApplyKubernetesResponse, ApplyResponse,
};

#[derive(thiserror::Error, Debug)]
pub enum MpsOrchestratorError {
    #[error("Template error: {0}")]
    Tera(#[from] tera::Error),
    #[error("failed kube api: {0}")]
    Kube(#[from] kube::Error),
    #[error("failed parse yaml: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("failed parse json: {0}")]
    Json(#[from] serde_json::Error),
}

pub async fn create_namespace(
    resource_content: &str,
) -> Result<(), MpsOrchestratorError> {
    // Configurar o cliente Kubernetes
    let client = Client::try_default().await?;

    // Acessar a API do Kubernetes para namespaces
    let namespaces: Api<Namespace> = Api::all(client.clone());

    let n: Namespace = serde_yaml::from_str(resource_content)?;

    // Criar o novo namespace
    let created_namespace = namespaces.create(&Default::default(), &n).await?;

    println!("Namespace criado: {created_namespace:?}");

    Ok(())
}

pub async fn create_deployment(
    resource_content: &str,
    name: &str,
    namespace: &str,
) -> Result<(), MpsOrchestratorError> {
    let mut config = kube::Config::infer().await.unwrap();
    config.default_namespace = namespace.to_string();
    let client = Client::try_from(config).unwrap();

    // Manage deployment
    let deployment: Api<Deployment> = Api::default_namespaced(client.clone());
    let pod: Api<Pod> = Api::default_namespaced(client);

    // Create Pod blog
    // println!("Creating Pod instance blog");
    // let p: Pod = serde_json::from_value(json!({
    //     "apiVersion": "v1",
    //     "kind": "Pod",
    //     "metadata": { "name": "blog" },
    //     "spec": {
    //         "containers": [{
    //           "name": "blog",
    //           "image": "clux/blog:0.1.0"
    //         }],
    //     }
    // }))
    // .unwrap();

    let p: Deployment = serde_yaml::from_str(resource_content)?;
    let pp = PostParams::default();
    match deployment.create(&pp, &p).await {
        Ok(o) => {
            let name = o.name_any();
            assert_eq!(p.name_any(), name);
            println!("Created {}", name);
        }
        Err(kube::Error::Api(ae)) => assert_eq!(ae.code, 409), // if you skipped delete, for instance
        Err(e) => panic!("Error creating pod {:?}", e), // any other case is probably bad
    }

    // Watch it phase for a few seconds
    // let establish = await_condition(pod.clone(), "mps-sample-nestjs-6d5f9fdf4-c5qs7", is_pod_running());
    // let _ = tokio::time::timeout(std::time::Duration::from_secs(10), establish)
    //     .await
    //     .unwrap();
    // Verify we can get it
    println!("Get Pod {name}");
    let p1cpy = deployment.get(name).await.unwrap();
    println!("Got blog pod with containers: {:?}", p1cpy.metadata);
    assert_eq!(p1cpy.metadata.name.unwrap(), name);

    // Verify we can get it
    // println!("Get Pod blog");
    // let p1cpy = deployment.get(name).await.unwrap();
    // if let Some(spec) = &p1cpy.spec {
    //     println!("Got blog pod with containers: {:?}", spec.containers);
    //     assert_eq!(spec.containers[0].name, name);
    // }

    // let lp = ListParams::default().fields(&format!("metadata.name={}", name)); // only want results for our pod
    // for p in pods.list(&lp).await.unwrap() {
    //     println!("Found Pod: {}", p.name_any());
    // }

    // // Delete it
    // let dp = DeleteParams::default();
    // pods.delete(name, &dp).await.unwrap().map_left(|pdel| {
    //     assert_eq!(pdel.name_any(), name);
    //     println!("Deleting blog pod started: {:?}", pdel);
    // });
    Ok(())
}

#[derive(Debug)]
pub struct MpsKubernetesGrpcServer {
    tera: Tera,
}

impl MpsKubernetesGrpcServer {
    pub fn new(tera: Tera) -> Self {
        Self { tera }
    }
}

// impl From<String> for RepoResponse {
//     fn from(s: String) -> Self {
//         RepoResponse { name: s }
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicKubernetesContext {
    pub name: String,
    pub port: u16,
    pub image: String,
    pub version: String,
    pub namespace: String,
    pub domain: String,
    pub replicas: u8,
}

#[tonic::async_trait]
impl Kubernetes for MpsKubernetesGrpcServer {
    async fn apply(
        &self,
        request: Request<ApplyKubernetesRequest>,
    ) -> Result<Response<ApplyKubernetesResponse>, Status> {
        let req = request.into_inner();
        let context_json: BasicKubernetesContext =
            serde_json::from_str(&req.context).unwrap();
        // // Crie um contexto de dados para o template
        // let mut context = Context::new();
        // context.insert("name", "mps-sample-nestjs");
        // context.insert("port", &3000);
        // context.insert(
        //     "image",
        //     "account_id.dkr.ecr.region.amazonaws.com/project_name",
        // );
        // context.insert("version", "0.0.1");
        // context.insert("namespace", "platform-engineering");
        // context.insert("domain", "info");
        // context.insert("replicas", &2);

        let context = Context::from_serialize(&context_json).unwrap();

        let deployment = self.tera.render("deployment.yml", &context).unwrap();
        dbg!(&deployment);
        if let Err(e) = create_deployment(
            &deployment,
            &context_json.name,
            &context_json.namespace,
        )
        .await
        {
            return Err(Status::invalid_argument(e.to_string()));
        }

        let response = ApplyKubernetesResponse {
            result: ApplyResult::Success.into(),
            application: Some(ApplyResponse { name: context_json.name }),
        };

        Ok(Response::new(response))
    }
}

pub async fn server() {
    // Crie uma instância Tera
    let tera = Tera::new("templates/*.yml").unwrap();

    let addr = "[::1]:50060".parse().unwrap();
    let kubernetes = MpsKubernetesGrpcServer::new(tera);

    Server::builder()
        .add_service(KubernetesServer::new(kubernetes))
        .serve(addr)
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn simple_test() {
        pod_crud().await;
        assert!(true);
    }
}
