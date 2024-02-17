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
use aws_sdk_ecr::{
    config::{Credentials, Region},
    error::SdkError,
    operation::create_repository::CreateRepositoryError,
    Client,
};

#[derive(thiserror::Error, Debug)]
pub enum MpsCloudError {
    #[error("failed to create ecr repository: {0}")]
    AwsSdkError(#[from] SdkError<CreateRepositoryError>),
}

pub async fn ecr_create_repository(
    access_key: &str,
    access_secret: &str,
    repository_name: &str,
) -> Result<String, MpsCloudError> {
    let credentials =
        Credentials::new(access_key, access_secret, None, None, "ecr");

    let config = aws_config::from_env()
        .credentials_provider(credentials)
        .region(Region::new("us-east-1"))
        .load()
        .await;

    let client = Client::new(&config);

    let resp = client
        .create_repository()
        .repository_name(repository_name)
        .send()
        .await?;
    Ok(resp.repository().unwrap().repository_uri().unwrap().to_string())
}
