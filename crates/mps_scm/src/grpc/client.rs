use scm::scm_client::ScmClient;
use scm::CreateRepoRequest;

pub mod scm {
    tonic::include_proto!("scm");
}

pub async fn create_repo(
    request: CreateRepoRequest,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ScmClient::connect("http://[::1]:50051").await?;

    let response = client.create_repo(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
