use mps_scm::grpc::server;

#[tokio::main]
async fn main() {
    let _ = server().await;
}
