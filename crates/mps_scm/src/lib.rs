use color_eyre::eyre::{eyre, Result};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::Duration;
use thiserror::Error;
use tracing::{error, info, instrument};

const GITHUB_API_URL: &str = "https://api.github.com";

// Estrutura que representa o erro retornado pela API REST do GitHub
#[derive(Debug, Deserialize, Error, Serialize)]
pub struct GitHubAPIError {
    pub message: String,
    pub documentation_url: Option<String>,
}

impl fmt::Display for GitHubAPIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GitHub API Error: {}", self.message)
    }
}

// Erros personalizados
#[derive(Error, Debug)]
enum GitHubError {
    #[error("Erro na requisição HTTP: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Erro ao desserializar a resposta JSON: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Erro na API do GitHub: {0}")]
    APIError(GitHubAPIError),
    #[error("Limite de taxa atingido. Tentativas esgotadas.")]
    RateLimitExceeded,
}

#[derive(Debug)]
pub struct GithubProvider {
    client: reqwest::Client,
    token: String,
    base_url: String,
}

impl GithubProvider {
    pub fn new(token: &str, base_url: &str) -> Self {
        // Configurar o tempo limite para a conexão e para a leitura da resposta
        let timeout = Duration::from_secs(10); // 10 segundos
        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            HeaderValue::from_str(&format!(
                "{}/{}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ))
            .expect("Invalid header value"),
        );
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&format!("token {}", token))
                .expect("Invalid token"),
        );

        let client = reqwest::Client::builder()
            .timeout(timeout)
            .default_headers(headers)
            .build()
            .expect("Failed to create reqwest client");

        GithubProvider {
            client,
            base_url: base_url.to_string(),
            token: token.to_string(),
        }
    }

    #[instrument]
    async fn make_request<R, T>(
        &self,
        method: reqwest::Method,
        path: &str,
        payload: Option<T>,
    ) -> Result<R, GitHubError>
    where
        T: Serialize + fmt::Debug,
        R: for<'de> Deserialize<'de>,
    {
        let url = format!("{}{}", self.base_url, path);
        let mut request_builder = self.client.request(method, url);

        if let Some(payload) = payload {
            request_builder = request_builder.json(&payload);
        }

        let response = request_builder.send().await?;

        // Verificar se a solicitação foi bem-sucedida
        if response.status().is_success() {
            // Desserializar a resposta em uma estrutura de dados genérica
            let data: R = response.json().await?;
            // info!("Request successful: {:?}", response);
            Ok(data)
        } else if response.status().as_u16() == 403 {
            // Se atingiu o limite de taxa, retorne o erro correspondente
            error!("Rate limit exceeded");
            Err(GitHubError::RateLimitExceeded)
        } else {
            // Se a solicitação falhar, retorne o erro correspondente
            error!("Request failed: {:?}", response);
            Err(GitHubError::RateLimitExceeded)
            // Err(GitHubError::APIError(reqwest::Error::from(response)))
        }
    }

    pub async fn get_rate_limit(&self) -> Result<RateLimit, GitHubError> {
        let url = format!("{}/rate_limit", GITHUB_API_URL);
        self.make_request::<RateLimit, ()>(reqwest::Method::GET, &url, None)
            .await
    }

    pub async fn create_github_repository(
        &self,
        new_repo: NewRepository,
    ) -> Result<RepositoryResponse, GitHubError> {
        let path = "/user/repos";
        self.make_request(reqwest::Method::POST, path, Some(new_repo)).await
    }
}

// Struct para representar as informações sobre o rate limit da API do GitHub
#[derive(Debug, Serialize, Deserialize)]
pub struct RateLimit {
    pub limit: u32,
    pub remaining: u32,
    pub reset: u64,
}

// Struct para representar os dados necessários para criar um repositório no GitHub
#[derive(Debug, Serialize, Deserialize)]
pub struct NewRepository {
    pub name: String,
    // Outros campos opcionais podem ser adicionados aqui, como descrição, visibilidade, etc.
}

// Struct para representar a resposta da API do GitHub ao criar um repositório
#[derive(Deserialize, Debug)]
pub struct RepositoryResponse {
    pub name: String,
    pub html_url: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_create_github_repository_success() {
        let mut server = mockito::Server::new();

        // Use one of these addresses to configure your client
        let host = server.host_with_port();
        let url = server.url();
        // Inicializa o servidor mockito
        let _m = server.mock("POST", "/user/repos")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(r#"{"name": "test-repo", "html_url": "https://github.com/test/test-repo"}"#)
            .create();

        // Configura o GithubProvider com a URL do servidor mockito
        let provider = GithubProvider::new("token", url.as_str());

        // Cria um novo repositório
        let new_repo = NewRepository { name: "test-repo".to_string() };
        let result = provider.create_github_repository(new_repo).await;

        // Verifica se o repositório foi criado com sucesso
        assert!(result.is_ok());
        let repository = result.unwrap();
        assert_eq!(repository.name, "test-repo");
        assert_eq!(repository.html_url, "https://github.com/test/test-repo");
    }

    // #[tokio::test]
    // async fn test_create_github_repository_rate_limit_exceeded() {
    //     // Inicializa o servidor mockito
    //     let _m = mock("POST", "/user/repos")
    //         .with_status(403)
    //         .create();

    //     // Configura o GithubProvider com a URL do servidor mockito
    //     let provider = GithubProvider::new("token", server_address().to_string().as_str());

    //     // Cria um novo repositório
    //     let new_repo = NewRepository { name: "test-repo".to_string() };
    //     let result = provider.create_github_repository(&new_repo).await;

    //     // Verifica se o rate limit é detectado corretamente
    //     assert!(result.is_err());
    //     match result.unwrap_err() {
    //         GitHubError::RateLimitExceeded => assert!(true),
    //         _ => assert!(false, "Expected RateLimitExceeded error"),
    //     }
    // }

    // #[tokio::test]
    // async fn test_create_github_repository_retry() {
    //     // Inicializa o servidor mockito
    //     let _m = mock("POST", "/user/repos")
    //         .with_status(500)
    //         .expect(2)
    //         .create();

    //     // Configura o GithubProvider com a URL do servidor mockito
    //     let provider = GithubProvider::new("token", server_address().to_string().as_str());

    //     // Cria um novo repositório
    //     let new_repo = NewRepository { name: "test-repo".to_string() };
    //     let result = provider.create_github_repository(&new_repo).await;

    //     // Verifica se houve uma retentativa após um erro
    //     assert!(result.is_err());
    // }
}
