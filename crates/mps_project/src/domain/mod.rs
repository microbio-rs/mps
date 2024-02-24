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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, new)]
pub struct ProjectId(Uuid);

impl ProjectId {
    pub fn to_uuid(&self) -> Uuid {
        self.0
    }
}

impl ToString for ProjectId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<ProjectId> for String {
    fn from(p: ProjectId) -> String {
        p.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, new)]
pub struct UserId(Uuid);

impl ToString for UserId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<UserId> for String {
    fn from(p: UserId) -> String {
        p.to_string()
    }
}

impl UserId {
    pub fn to_uuid(&self) -> Uuid {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, new)]
pub struct Project {
    pub id: Option<ProjectId>,
    pub user_id: UserId,
    pub name: String,
    pub description: Option<String>,
}

pub struct NewRepo {
    pub name: String,
    pub html_url: String,
}
