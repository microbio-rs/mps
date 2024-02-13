use tonic::{transport::Server, Request, Response, Status};

use scm::scm_server::{Scm, ScmServer};
use scm::{CreateRepoRequest, CreateRepoResponse};

pub mod scm {
    tonic::include_proto!("scm");
}

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

pub async fn server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let scm = MyScm::default();

    Server::builder().add_service(ScmServer::new(scm)).serve(addr).await?;

    Ok(())
}
