use tonic::{transport::Server, Request, Response, Status};

use super::scm::scm_server::{Scm, ScmServer};
use super::scm::{CreateRepoRequest, CreateRepoResponse};

#[derive(Debug, Default)]
struct MyScm {}

#[tonic::async_trait]
impl Scm for MyScm {
    async fn create_repo(
        &self,
        request: Request<CreateRepoRequest>,
    ) -> Result<Response<CreateRepoResponse>, Status> {
        println!("Got a request: {:?}", &request);

        let reply = CreateRepoResponse { name: request.into_inner().name };

        Ok(Response::new(reply))
    }
}

pub async fn server() {
    let addr = "[::1]:50051".parse().unwrap();
    let scm = MyScm::default();

    Server::builder()
        .add_service(ScmServer::new(scm))
        .serve(addr)
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grpc::scm::{
        scm_client::ScmClient, CreateRepoRequest, Provider,
    };
    use tokio::task;
    use tonic::transport::Channel;

    // Função de teste para criar um repositorio github
    #[tokio::test]
    async fn test_create_repo_github() {
        // Inicie o servidor em uma nova tarefa
        let _server_handle = task::spawn(server());

        // Aguarde alguns milissegundos para garantir que o servidor está pronto
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        // Conecte-se ao servidor
        let channel =
            Channel::from_static("http://[::1]:50051").connect().await.unwrap();
        let mut client = ScmClient::new(channel);

        // Crie uma solicitação de tarefa
        let request = tonic::Request::new(CreateRepoRequest {
            provider: Provider::Github.into(),
            name: "aninha".into(),
        });

        // Chame o método do cliente para adicionar a tarefa
        let response = client.create_repo(request).await.unwrap();

        // Verifique se recebemos um ID de tarefa
        let name = response.into_inner().name;
        assert_eq!(name, String::from("aninha"));
    }
}
