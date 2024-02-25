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

pub mod github;
pub use github::*;

use derive_new::new;
use uuid::Uuid;

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, new, serde::Deserialize,
)]
pub struct ApplicationId(Uuid);

impl ToString for ApplicationId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<ApplicationId> for String {
    fn from(p: ApplicationId) -> String {
        p.to_string()
    }
}

impl From<Uuid> for ApplicationId {
    fn from(u: Uuid) -> ApplicationId {
        ApplicationId::new(u)
    }
}

impl ApplicationId {
    pub fn to_uuid(&self) -> Uuid {
        self.0
    }
}

// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     new,
//     strum::Display,
//     strum::EnumString,
//     num_derive::FromPrimitive,
//     num_derive::ToPrimitive,
// )]
// pub enum ServiceKind {
//     Application,
//     Database,
// }
