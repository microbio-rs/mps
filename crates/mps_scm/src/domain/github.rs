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

use derive_new::new;
use uuid::Uuid;

use super::ApplicationId;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, new)]
pub struct GithubRepositoryId(Uuid);

impl GithubRepositoryId {
    pub fn to_uuid(&self) -> Uuid {
        self.0
    }
}

impl ToString for GithubRepositoryId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<GithubRepositoryId> for String {
    fn from(p: GithubRepositoryId) -> String {
        p.to_string()
    }
}

impl From<Uuid> for GithubRepositoryId {
    fn from(u: Uuid) -> GithubRepositoryId {
        GithubRepositoryId::new(u)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, new)]
pub struct GithubRepository {
    pub id: Option<GithubRepositoryId>,
    pub application_id: ApplicationId,
    pub default_branch: String,
    pub description: Option<String>,
    pub full_name: String,
    pub name: String,
    pub private: bool,
    pub provider_id: i64,
    pub size: i64,
    pub ssh_url: String,
    pub url: String,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, new, serde::Deserialize,
)]
pub struct GithubCreateRepositoryResponse {
    pub id: i64,
    #[serde(skip_serializing)]
    pub application_id: Option<ApplicationId>,
    pub default_branch: String,
    pub description: Option<String>,
    pub full_name: String,
    pub name: String,
    pub private: bool,
    pub size: i64,
    pub ssh_url: String,
    pub url: String,
}

impl From<GithubCreateRepositoryResponse> for GithubRepository {
    fn from(r: GithubCreateRepositoryResponse) -> Self {
        Self::new(
            None,
            r.application_id.unwrap(),
            r.default_branch,
            r.description,
            r.full_name,
            r.name,
            r.private,
            r.id,
            r.size,
            r.ssh_url,
            r.url,
        )
    }
}
